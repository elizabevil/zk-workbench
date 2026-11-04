//! 应用路径等通用 DTO。

use serde::Serialize;
use ts_rs::TS;

/// 应用使用的关键路径，用于设置页展示与排查。
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct AppPaths {
    pub config_path: String,
    pub log_dir: String,
}
