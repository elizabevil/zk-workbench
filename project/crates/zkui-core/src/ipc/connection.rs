//! 连接方向的 DTO。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::connection::{ConnectionState, Identity};
use crate::transport::ProxyProfile;

/// 创建连接的入参（S1 仅内存，不落盘，不含凭据）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateConnection {
    /// 集群地址的主机部分。
    pub host: String,
    /// 集群地址的端口部分。
    pub port: u16,
    /// 展示用名称。
    pub display_name: String,
    /// 可选认证身份（仅展示）；凭据通过 `addauth` 提供，不进入 DTO。首版仅接受 digest。
    pub identity: Option<Identity>,
    /// 可选代理；None 表示直连。
    #[serde(default)]
    pub proxy: Option<ProxyProfile>,
}

/// 连接快照。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConnectionSnapshot {
    pub connection_id: String,
    pub display_name: String,
    pub host: String,
    pub port: u16,
    pub state: ConnectionState,
    #[ts(type = "number")]
    pub generation: u64,
    pub identity: Option<Identity>,
    #[ts(type = "number")]
    pub session_timeout_ms: u64,
}

/// 状态变化事件载荷。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ConnectionStateEvent {
    pub connection_id: String,
    pub state: ConnectionState,
    #[ts(type = "number")]
    pub generation: u64,
}

#[cfg(test)]
mod tests {
    use super::CreateConnection;

    #[test]
    fn frontend_payload_deserializes() {
        let payload = serde_json::json!({
            "request": {
                "host": "192.168.11.5",
                "port": 2181,
                "display_name": "local",
                "identity": null,
                "proxy": null
            }
        });
        let request: CreateConnection = serde_json::from_value(payload["request"].clone())
            .expect("前端 payload 必须可反序列化");
        assert_eq!(request.host, "192.168.11.5");
        assert_eq!(request.port, 2181);
        assert_eq!(request.display_name, "local");
    }

    #[test]
    fn legacy_endpoint_payload_is_rejected_with_missing_host() {
        let payload = serde_json::json!({
            "endpoint": "192.168.11.5:2181",
            "display_name": "local"
        });
        let error = serde_json::from_value::<CreateConnection>(payload).unwrap_err();
        assert!(error.to_string().contains("host"));
    }
}
