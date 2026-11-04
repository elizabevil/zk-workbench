//! 节点读取 DTO。

use serde::Serialize;
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
