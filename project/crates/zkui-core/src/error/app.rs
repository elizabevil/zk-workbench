//! 应用与 IPC 侧错误构造。

use super::{AppError, ErrorFields, ErrorKind, ErrorScope};

pub fn invalid_path(path: &str) -> AppError {
    AppError::new(
        ErrorKind::AppInvalidPath,
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
        ErrorKind::AppNotFound,
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
        ErrorKind::AppClosed,
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
        ErrorKind::IpcInvalidArgument,
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
        ErrorKind::IpcStaleGeneration,
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
