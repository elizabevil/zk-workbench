//! 节点读取与写入 DTO。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 节点 Stat 的展示子集。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct NodeStat {
    pub path: String,
    pub version: i32,
    pub cversion: i32,
    pub aversion: i32,
    pub data_length: i32,
    pub num_children: i32,
    #[ts(type = "number")]
    pub ephemeral_owner: i64,
    #[ts(type = "number")]
    pub czxid: i64,
    #[ts(type = "number")]
    pub mzxid: i64,
    #[ts(type = "number")]
    pub ctime: i64,
    #[ts(type = "number")]
    pub mtime: i64,
}

/// 节点数据。`data_b64` 为原始字节，前端按需解码展示。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct NodeData {
    pub path: String,
    pub data_b64: String,
    #[ts(type = "number")]
    pub byte_len: u64,
    pub is_text: bool,
    pub stat: NodeStat,
}

/// 创建节点类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum NodeCreateMode {
    Persistent,
    Ephemeral,
    PersistentSequential,
    EphemeralSequential,
}

/// 删除影响预览。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct DeletePreview {
    pub path: String,
    #[ts(type = "number")]
    pub total: u64,
    pub paths: Vec<String>,
    pub truncated: bool,
}

/// 删除结果：已完成、失败与未执行。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct DeleteOutcome {
    #[ts(type = "number")]
    pub deleted: u64,
    pub failed_path: Option<String>,
    pub failed_error: Option<crate::error::AppError>,
    #[ts(type = "number")]
    pub unexecuted: u64,
}
