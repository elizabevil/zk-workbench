//! 连接方向的 DTO。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::connection::{ConnectionState, Identity};

/// 创建连接的入参（S1 仅内存，不落盘，不含凭据）。
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct CreateConnection {
    /// `host:port` 或逗号分隔的集群列表。
    pub endpoint: String,
    /// 展示用名称。
    pub display_name: String,
    /// 可选认证身份；凭据本身不进入 DTO。
    pub identity: Option<Identity>,
}

/// 连接快照。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct ConnectionSnapshot {
    pub connection_id: String,
    pub display_name: String,
    pub state: ConnectionState,
    #[ts(type = "number")]
    pub generation: u64,
    pub identity: Option<Identity>,
    #[ts(type = "number")]
    pub session_timeout_ms: u64,
}

/// 状态变化事件载荷。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct ConnectionStateEvent {
    pub connection_id: String,
    pub state: ConnectionState,
    #[ts(type = "number")]
    pub generation: u64,
}
