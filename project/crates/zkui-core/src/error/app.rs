//! 应用与 IPC 侧错误构造。

use super::{AppError, ErrorFields, ErrorKind, ErrorScope};

pub fn invalid_path(path: &str) -> AppError {
    AppError::new(
        ErrorKind::InvalidPath,
        ErrorScope::App,
        2001,
        ErrorFields {
            path: Some(path.to_string()),
            ..Default::default()
        },
    )
}

pub fn not_found(connection_id: &str) -> AppError {
    AppError::new(
        ErrorKind::NotFound,
        ErrorScope::App,
        2002,
        ErrorFields {
            connection_id: Some(connection_id.to_string()),
            ..Default::default()
        },
    )
}

pub fn closed(connection_id: &str) -> AppError {
    AppError::new(
        ErrorKind::Closed,
        ErrorScope::App,
        2003,
        ErrorFields {
            connection_id: Some(connection_id.to_string()),
            ..Default::default()
        },
    )
}

pub fn invalid_argument(detail: &str) -> AppError {
    AppError::new(
        ErrorKind::InvalidArgument,
        ErrorScope::Ipc,
        3001,
        ErrorFields {
            detail: Some(detail.to_string()),
            ..Default::default()
        },
    )
}

pub fn stale_generation(connection_id: &str, expected: u64, actual: u64) -> AppError {
    AppError::new(
        ErrorKind::StaleGeneration,
        ErrorScope::Ipc,
        3002,
        ErrorFields {
            connection_id: Some(connection_id.to_string()),
            generation: Some(actual),
            detail: Some(format!("expected {expected}, actual {actual}")),
            ..Default::default()
        },
    )
}

/// 代理阶段失败。`stage` 取值：connect / handshake / relay。
pub fn proxy_failed(stage: &str, detail: &str) -> AppError {
    let kind = if stage == "handshake" {
        ErrorKind::Handshake
    } else {
        ErrorKind::Connect
    };
    let code = if stage == "handshake" { 4002 } else { 4001 };
    AppError::new(
        kind,
        ErrorScope::Proxy,
        code,
        ErrorFields {
            detail: Some(format!("{stage}: {detail}")),
            ..Default::default()
        },
    )
}

/// 系统凭据存储不可用。
pub fn secret_store_failed(detail: &str) -> AppError {
    AppError::new(
        ErrorKind::SecretStore,
        ErrorScope::App,
        2006,
        ErrorFields {
            detail: Some(detail.to_string()),
            ..Default::default()
        },
    )
}

/// 审计存储失败。
pub fn audit_failed(detail: &str) -> AppError {
    AppError::new(
        ErrorKind::AuditFailed,
        ErrorScope::App,
        2007,
        ErrorFields {
            detail: Some(detail.to_string()),
            ..Default::default()
        },
    )
}

/// OTel 导出失败。
pub fn otel_failed(detail: &str) -> AppError {
    AppError::new(
        ErrorKind::OtelExportFailed,
        ErrorScope::App,
        2008,
        ErrorFields {
            detail: Some(detail.to_string()),
            ..Default::default()
        },
    )
}

/// 操作被用户取消。
pub fn cancelled() -> AppError {
    AppError::new(
        ErrorKind::Cancelled,
        ErrorScope::App,
        2009,
        ErrorFields::default(),
    )
}

/// 命令解析或执行参数错误。
pub fn command_invalid(detail: &str) -> AppError {
    AppError::new(
        ErrorKind::CommandInvalid,
        ErrorScope::App,
        2010,
        ErrorFields {
            detail: Some(detail.to_string()),
            ..Default::default()
        },
    )
}
