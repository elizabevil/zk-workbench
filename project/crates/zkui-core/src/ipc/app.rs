//! 应用路径等通用 DTO。

use serde::Serialize;
use ts_rs::TS;

/// 应用使用的关键路径，用于设置页展示与排查。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct AppPaths {
    pub config_path: String,
    pub log_dir: String,
}

/// 应用元信息，来自构建配置，不在前端写死。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}
