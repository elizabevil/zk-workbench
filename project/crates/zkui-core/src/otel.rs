//! OpenTelemetry OTLP/HTTP（protobuf）导出：默认关闭，由用户显式开启。

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use opentelemetry_proto::tonic::collector::metrics::v1::ExportMetricsServiceRequest;
use opentelemetry_proto::tonic::common::v1::{AnyValue, InstrumentationScope, KeyValue, any_value};
use opentelemetry_proto::tonic::metrics::v1::{
    Gauge, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics, metric, number_data_point,
};
use opentelemetry_proto::tonic::resource::v1::Resource;
use prost::Message;

use crate::ipc::metrics::MetricsSnapshot;

/// 导出一次指标。`endpoint` 为 OTLP HTTP 基地址（如 `http://localhost:4318`）。
pub async fn export(endpoint: &str, snapshot: &MetricsSnapshot) -> Result<(), String> {
    let request = build_request(snapshot, now_nanos());
    let body = request.encode_to_vec();
    let url = if endpoint.trim_end_matches('/').ends_with("/v1/metrics") {
        endpoint.trim_end_matches('/').to_string()
    } else {
        format!("{}/v1/metrics", endpoint.trim_end_matches('/'))
    };

    let response = reqwest::Client::new()
        .post(url)
        .header("content-type", "application/x-protobuf")
        .timeout(Duration::from_secs(10))
        .body(body)
        .send()
        .await
        .map_err(|error| format!("otlp request failed: {error}"))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("otlp export rejected: HTTP {}", response.status()))
    }
}

/// 构造 OTLP 指标请求，便于测试与复用。
pub fn build_request(
    snapshot: &MetricsSnapshot,
    time_unix_nano: u64,
) -> ExportMetricsServiceRequest {
    let gauges = [
        ("zkui.connect.attempts", snapshot.connect_attempts, "1"),
        ("zkui.connect.failures", snapshot.connect_failures, "1"),
        ("zkui.reconnects", snapshot.reconnects, "1"),
        ("zkui.node.reads", snapshot.node_reads, "1"),
        ("zkui.node.writes", snapshot.node_writes, "1"),
        ("zkui.node.creates", snapshot.node_creates, "1"),
        ("zkui.node.deletes", snapshot.node_deletes, "1"),
        ("zkui.read.p50", snapshot.read_p50_ms, "ms"),
        ("zkui.read.p95", snapshot.read_p95_ms, "ms"),
        ("zkui.write.p50", snapshot.write_p50_ms, "ms"),
        ("zkui.write.p95", snapshot.write_p95_ms, "ms"),
    ];

    let metrics = gauges
        .iter()
        .map(|(name, value, unit)| Metric {
            name: (*name).to_string(),
            description: String::new(),
            unit: (*unit).to_string(),
            metadata: Vec::new(),
            data: Some(metric::Data::Gauge(Gauge {
                data_points: vec![NumberDataPoint {
                    attributes: Vec::new(),
                    start_time_unix_nano: 0,
                    time_unix_nano,
                    exemplars: Vec::new(),
                    flags: 0,
                    value: Some(number_data_point::Value::AsInt(*value as i64)),
                }],
            })),
        })
        .collect();

    ExportMetricsServiceRequest {
        resource_metrics: vec![ResourceMetrics {
            resource: Some(Resource {
                attributes: vec![
                    KeyValue {
                        key: "service.name".to_string(),
                        key_strindex: 0,
                        value: Some(AnyValue {
                            value: Some(any_value::Value::StringValue("zkui".to_string())),
                        }),
                    },
                    KeyValue {
                        key: "service.version".to_string(),
                        key_strindex: 0,
                        value: Some(AnyValue {
                            value: Some(any_value::Value::StringValue(
                                env!("CARGO_PKG_VERSION").to_string(),
                            )),
                        }),
                    },
                ],
                dropped_attributes_count: 0,
                entity_refs: Vec::new(),
            }),
            scope_metrics: vec![ScopeMetrics {
                scope: Some(InstrumentationScope {
                    name: "zkui".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    attributes: Vec::new(),
                    dropped_attributes_count: 0,
                }),
                metrics,
                schema_url: String::new(),
            }],
            schema_url: String::new(),
        }],
    }
}

fn now_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_contains_all_gauges() {
        let request = build_request(&MetricsSnapshot::default(), 1);
        let scope = &request.resource_metrics[0].scope_metrics[0];
        assert_eq!(scope.metrics.len(), 11);
        assert!(scope.metrics.iter().all(|metric| metric.data.is_some()));
        assert!(!request.encode_to_vec().is_empty());
    }
}

#[tokio::test]
async fn export_posts_protobuf_to_endpoint() {
    use std::io::{Read, Write};

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 1024];
        loop {
            let read = stream.read(&mut chunk).expect("read");
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..read]);
            if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        let head = String::from_utf8_lossy(&buffer).to_ascii_lowercase();
        assert!(head.contains("application/x-protobuf"));
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
            .expect("write");
    });

    export(&format!("http://{addr}"), &MetricsSnapshot::default())
        .await
        .expect("export");
    server.join().expect("server");
}
