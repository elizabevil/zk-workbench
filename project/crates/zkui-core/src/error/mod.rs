//! 应用错误模型。
//!
//! 线上形状固定为 `{ kind, scope, code, message_key, fields }`：
//! - `scope` 标识域/命名空间：`zk`、`app`、`proxy`、`ipc`；
//! - `kind` 只是该域内的错误类型，不带域前缀；
//! - `message_key` 由 `error.<scope>.<kind>` 生成，交给前端 i18n 渲染；
//! - `fields` 携带路径、版本、连接与 generation 等定位信息。

pub mod app;
pub mod zk;

use serde::Serialize;
use ts_rs::TS;
use zookeeper_client::Error as ZkError;

/// 错误来源范围。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum ErrorScope {
    Zk,
    App,
    Proxy,
    Ipc,
}

impl ErrorScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorScope::Zk => "zk",
            ErrorScope::App => "app",
            ErrorScope::Proxy => "proxy",
            ErrorScope::Ipc => "ipc",
        }
    }
}

/// 错误类型，仅表示域内类型，不带域前缀；域由 `ErrorScope` 表示。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum ErrorKind {
    // zk 域
    NoNode,
    NoAuth,
    BadVersion,
    NodeExists,
    NotEmpty,
    SessionExpired,
    AuthFailed,
    ConnectionLoss,
    NoChildrenForEphemerals,
    InvalidAcl,
    QuotaExceeded,
    Unimplemented,
    Timeout,
    Other,
    // app 域
    InvalidPath,
    NotFound,
    Closed,
    SecretStore,
    AuditFailed,
    OtelExportFailed,
    Cancelled,
    // proxy 域
    Connect,
    Handshake,
    // ipc 域
    InvalidArgument,
    StaleGeneration,
    Unknown,
}

impl ErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorKind::NoNode => "no_node",
            ErrorKind::NoAuth => "no_auth",
            ErrorKind::BadVersion => "bad_version",
            ErrorKind::NodeExists => "node_exists",
            ErrorKind::NotEmpty => "not_empty",
            ErrorKind::SessionExpired => "session_expired",
            ErrorKind::AuthFailed => "auth_failed",
            ErrorKind::ConnectionLoss => "connection_loss",
            ErrorKind::NoChildrenForEphemerals => "no_children_for_ephemerals",
            ErrorKind::InvalidAcl => "invalid_acl",
            ErrorKind::QuotaExceeded => "quota_exceeded",
            ErrorKind::Unimplemented => "unimplemented",
            ErrorKind::Timeout => "timeout",
            ErrorKind::Other => "other",
            ErrorKind::InvalidPath => "invalid_path",
            ErrorKind::NotFound => "not_found",
            ErrorKind::Closed => "closed",
            ErrorKind::SecretStore => "secret_store",
            ErrorKind::AuditFailed => "audit_failed",
            ErrorKind::OtelExportFailed => "otel_export_failed",
            ErrorKind::Cancelled => "cancelled",
            ErrorKind::Connect => "connect",
            ErrorKind::Handshake => "handshake",
            ErrorKind::InvalidArgument => "invalid_argument",
            ErrorKind::StaleGeneration => "stale_generation",
            ErrorKind::Unknown => "unknown",
        }
    }
}

/// 定位信息。始终输出全部字段，未使用时为 null，避免前端做可选字段判断。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct ErrorFields {
    pub path: Option<String>,
    pub version: Option<i32>,
    pub connection_id: Option<String>,
    #[ts(type = "number | null")]
    pub generation: Option<u64>,
    pub detail: Option<String>,
}

/// 对外错误形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AppError {
    pub kind: ErrorKind,
    pub scope: ErrorScope,
    pub code: i32,
    pub message_key: String,
    pub fields: ErrorFields,
}

impl AppError {
    pub fn new(kind: ErrorKind, scope: ErrorScope, code: i32, fields: ErrorFields) -> Self {
        let message_key = format!("error.{}.{}", scope.as_str(), kind.as_str());
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

#[cfg(test)]
mod tests {
    use super::{ErrorKind, ErrorScope, app};

    #[test]
    fn message_key_is_scope_and_kind() {
        let error = app::proxy_failed("handshake", "denied");
        assert_eq!(error.scope, ErrorScope::Proxy);
        assert_eq!(error.kind, ErrorKind::Handshake);
        assert_eq!(error.message_key, "error.proxy.handshake");
        assert_eq!(error.code, 4002);
    }

    #[test]
    fn fields_always_serialize_all_keys() {
        let error = app::not_found("conn-1");
        let value = serde_json::to_value(&error).expect("serialize");
        let fields = value["fields"].as_object().expect("fields object");
        for key in ["path", "version", "connection_id", "generation", "detail"] {
            assert!(fields.contains_key(key), "missing fields.{key}");
        }
    }

    #[test]
    fn ipc_error_uses_ipc_namespace() {
        let error = app::invalid_argument("host must not be empty");
        assert_eq!(error.scope, ErrorScope::Ipc);
        assert_eq!(error.kind, ErrorKind::InvalidArgument);
        assert_eq!(error.message_key, "error.ipc.invalid_argument");
    }
}
