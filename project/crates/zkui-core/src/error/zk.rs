//! ZooKeeper 错误到应用错误的映射。
//!
//! `code` 使用 ZooKeeper 原生错误码；未知错误统一为 `zk_other` 并保留原始文本到 `detail`。

use zookeeper_client::Error as ZkError;

use super::{AppError, ErrorFields, ErrorKind, ErrorScope};

fn build(kind: ErrorKind, code: i32, detail: Option<String>) -> AppError {
    AppError::new(
        kind,
        ErrorScope::Zk,
        code,
        ErrorFields {
            detail,
            ..Default::default()
        },
    )
}

/// 把客户端错误映射为应用错误。
pub fn map_zk_error(error: ZkError) -> AppError {
    match error {
        ZkError::NoNode => build(ErrorKind::ZkNoNode, -101, None),
        ZkError::NoAuth => build(ErrorKind::ZkNoAuth, -102, None),
        ZkError::BadVersion => build(ErrorKind::ZkBadVersion, -103, None),
        ZkError::NoChildrenForEphemerals => build(ErrorKind::ZkNoChildrenForEphemerals, -108, None),
        ZkError::NodeExists => build(ErrorKind::ZkNodeExists, -110, None),
        ZkError::NotEmpty => build(ErrorKind::ZkNotEmpty, -111, None),
        ZkError::SessionExpired => build(ErrorKind::ZkSessionExpired, -112, None),
        ZkError::InvalidAcl => build(ErrorKind::ZkInvalidAcl, -114, None),
        ZkError::AuthFailed => build(ErrorKind::ZkAuthFailed, -115, None),
        ZkError::QuotaExceeded => build(ErrorKind::ZkQuotaExceeded, -125, None),
        ZkError::Unimplemented => build(ErrorKind::ZkUnimplemented, -6, None),
        ZkError::Timeout => build(ErrorKind::ZkTimeout, -7, None),
        ZkError::ConnectionLoss => build(ErrorKind::ZkConnectionLoss, -4, None),
        other => build(ErrorKind::ZkOther, -1, Some(other.to_string())),
    }
}

/// 错误对应的前端文案 key。
pub fn message_key_for(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::ZkNoNode => "error.zk.no_node",
        ErrorKind::ZkNoAuth => "error.zk.no_auth",
        ErrorKind::ZkBadVersion => "error.zk.bad_version",
        ErrorKind::ZkNodeExists => "error.zk.node_exists",
        ErrorKind::ZkNotEmpty => "error.zk.not_empty",
        ErrorKind::ZkSessionExpired => "error.zk.session_expired",
        ErrorKind::ZkAuthFailed => "error.zk.auth_failed",
        ErrorKind::ZkConnectionLoss => "error.zk.connection_loss",
        ErrorKind::ZkNoChildrenForEphemerals => "error.zk.no_children_for_ephemerals",
        ErrorKind::ZkInvalidAcl => "error.zk.invalid_acl",
        ErrorKind::ZkQuotaExceeded => "error.zk.quota_exceeded",
        ErrorKind::ZkUnimplemented => "error.zk.unimplemented",
        ErrorKind::ZkTimeout => "error.zk.timeout",
        ErrorKind::ZkOther => "error.zk.other",
        ErrorKind::AppInvalidPath => "error.app.invalid_path",
        ErrorKind::AppNotFound => "error.app.not_found",
        ErrorKind::AppClosed => "error.app.closed",
        ErrorKind::IpcInvalidArgument => "error.ipc.invalid_argument",
        ErrorKind::IpcStaleGeneration => "error.ipc.stale_generation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_error_maps_to_stable_code() {
        let error = map_zk_error(ZkError::BadVersion);
        assert_eq!(error.kind, ErrorKind::ZkBadVersion);
        assert_eq!(error.code, -103);
        assert_eq!(error.message_key, "error.zk.bad_version");
        assert_eq!(error.scope, ErrorScope::Zk);
    }

    #[test]
    fn unknown_error_keeps_detail() {
        let error = map_zk_error(ZkError::Unimplemented);
        assert_eq!(error.kind, ErrorKind::ZkUnimplemented);
        assert_eq!(error.code, -6);
    }
}
