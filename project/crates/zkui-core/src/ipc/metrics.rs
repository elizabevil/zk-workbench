//! 指标 DTO。

use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct MetricsSnapshot {
    #[ts(type = "number")]
    pub connect_attempts: u64,
    #[ts(type = "number")]
    pub connect_failures: u64,
    #[ts(type = "number")]
    pub reconnects: u64,
    #[ts(type = "number")]
    pub node_reads: u64,
    #[ts(type = "number")]
    pub node_writes: u64,
    #[ts(type = "number")]
    pub node_creates: u64,
    #[ts(type = "number")]
    pub node_deletes: u64,
    #[ts(type = "number")]
    pub read_p50_ms: u64,
    #[ts(type = "number")]
    pub read_p95_ms: u64,
    #[ts(type = "number")]
    pub write_p50_ms: u64,
    #[ts(type = "number")]
    pub write_p95_ms: u64,
}
