//! 应用错误模型。
//!
//! 线上形状固定为 `{ kind, scope, code, message_key, fields }`：
//! - `kind` 带前缀标识来源：`zk_*` 对应 ZooKeeper 原生错误，`app_*` 为应用规则，`ipc_*` 为边界错误；
//! - `message_key` 交给前端 i18n 渲染，Rust 不返回面向用户的句子；
//! - `fields` 携带路径、版本、连接与 generation 等定位信息。

pub mod app;
pub mod zk;

use serde::Serialize;
use ts_rs::TS;
use zookeeper_client::Error as ZkError;

/// 错误来源范围。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(
    rename_all = "snake_case",
    export,
    export_to = "../../../src/api/types/"
)]
pub enum ErrorScope {
    Zk,
    App,
    Ipc,
}

/// 错误类别。`zk_*` 与 ZooKeeper 错误一一对应，`app_*` / `ipc_*` 为应用侧扩展。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(
    rename_all = "snake_case",
    export,
    export_to = "../../../src/api/types/"
)]
pub enum ErrorKind {
    ZkNoNode,
    ZkNoAuth,
    ZkBadVersion,
    ZkNodeExists,
    ZkNotEmpty,
    ZkSessionExpired,
    ZkAuthFailed,
    ZkConnectionLoss,
    ZkNoChildrenForEphemerals,
    ZkInvalidAcl,
    ZkQuotaExceeded,
    ZkUnimplemented,
    ZkTimeout,
    ZkOther,
    AppInvalidPath,
    AppNotFound,
    AppClosed,
    IpcInvalidArgument,
    IpcStaleGeneration,
}

/// 定位信息，未使用的字段省略。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct ErrorFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(type = "number | null")]
    pub generation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// 对外错误形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct AppError {
    pub kind: ErrorKind,
    pub scope: ErrorScope,
    pub code: i32,
    pub message_key: String,
    pub fields: ErrorFields,
}

impl AppError {
    pub fn new(kind: ErrorKind, scope: ErrorScope, code: i32, fields: ErrorFields) -> Self {
        let message_key = zk::message_key_for(kind).to_string();
        Self {
            kind,
            scope,
            code,
            message_key,
            fields,
        }
    }
}

/// 常用返回类型：错误装箱，避免 `Result` 体积过大触发 clippy。
pub type AppResult<T> = Result<T, Box<AppError>>;

impl From<ZkError> for Box<AppError> {
    fn from(error: ZkError) -> Self {
        Box::new(zk::map_zk_error(error))
    }
}

impl From<ZkError> for AppError {
    fn from(error: ZkError) -> Self {
        zk::map_zk_error(error)
    }
}
