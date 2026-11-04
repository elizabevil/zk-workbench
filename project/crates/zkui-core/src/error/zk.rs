//! ZooKeeper 错误到应用错误的映射。
//!
//! `code` 使用 ZooKeeper 原生错误码；未知错误统一为 `zk` 域的 `other`，并保留原始文本到 `detail`。

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
        ZkError::NoNode => build(ErrorKind::NoNode, -101, None),
        ZkError::NoAuth => build(ErrorKind::NoAuth, -102, None),
        ZkError::BadVersion => build(ErrorKind::BadVersion, -103, None),
        ZkError::NoChildrenForEphemerals => build(ErrorKind::NoChildrenForEphemerals, -108, None),
        ZkError::NodeExists => build(ErrorKind::NodeExists, -110, None),
        ZkError::NotEmpty => build(ErrorKind::NotEmpty, -111, None),
        ZkError::SessionExpired => build(ErrorKind::SessionExpired, -112, None),
        ZkError::InvalidAcl => build(ErrorKind::InvalidAcl, -114, None),
        ZkError::AuthFailed => build(ErrorKind::AuthFailed, -115, None),
        ZkError::QuotaExceeded => build(ErrorKind::QuotaExceeded, -125, None),
        ZkError::Unimplemented => build(ErrorKind::Unimplemented, -6, None),
        ZkError::Timeout => build(ErrorKind::Timeout, -7, None),
        ZkError::ConnectionLoss => build(ErrorKind::ConnectionLoss, -4, None),
        other => build(ErrorKind::Other, -1, Some(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_error_maps_to_stable_code() {
        let error = map_zk_error(ZkError::BadVersion);
        assert_eq!(error.kind, ErrorKind::BadVersion);
        assert_eq!(error.code, -103);
        assert_eq!(error.message_key, "error.zk.bad_version");
        assert_eq!(error.scope, ErrorScope::Zk);
    }

    #[test]
    fn unknown_error_keeps_detail() {
        let error = map_zk_error(ZkError::Unimplemented);
        assert_eq!(error.kind, ErrorKind::Unimplemented);
        assert_eq!(error.code, -6);
    }
}
