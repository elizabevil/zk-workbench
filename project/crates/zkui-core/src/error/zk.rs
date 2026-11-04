//! ZooKeeper 错误到应用错误的映射。
//!
//! `code` 使用 ZooKeeper 原生错误码；未知错误统一为 `zk` 域的 `other`，并保留原始文本到 `detail`。

use zookeeper_client::Error as ZkError;

use super::{AppError, ErrorFields, ErrorKind, ErrorScope};

/// ZooKeeper 原生错误码，与 Java 客户端 `KeeperException.Code` 对应。
pub mod codes {
    pub const NO_NODE: i32 = -101;
    pub const NO_AUTH: i32 = -102;
    pub const BAD_VERSION: i32 = -103;
    pub const NO_CHILDREN_FOR_EPHEMERALS: i32 = -108;
    pub const NODE_EXISTS: i32 = -110;
    pub const NOT_EMPTY: i32 = -111;
    pub const SESSION_EXPIRED: i32 = -112;
    pub const INVALID_ACL: i32 = -114;
    pub const AUTH_FAILED: i32 = -115;
    pub const QUOTA_EXCEEDED: i32 = -125;
    pub const UNIMPLEMENTED: i32 = -6;
    pub const TIMEOUT: i32 = -7;
    pub const CONNECTION_LOSS: i32 = -4;
    pub const OTHER: i32 = -1;
}

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
        ZkError::NoNode => build(ErrorKind::NoNode, codes::NO_NODE, None),
        ZkError::NoAuth => build(ErrorKind::NoAuth, codes::NO_AUTH, None),
        ZkError::BadVersion => build(ErrorKind::BadVersion, codes::BAD_VERSION, None),
        ZkError::NoChildrenForEphemerals => build(
            ErrorKind::NoChildrenForEphemerals,
            codes::NO_CHILDREN_FOR_EPHEMERALS,
            None,
        ),
        ZkError::NodeExists => build(ErrorKind::NodeExists, codes::NODE_EXISTS, None),
        ZkError::NotEmpty => build(ErrorKind::NotEmpty, codes::NOT_EMPTY, None),
        ZkError::SessionExpired => build(ErrorKind::SessionExpired, codes::SESSION_EXPIRED, None),
        ZkError::InvalidAcl => build(ErrorKind::InvalidAcl, codes::INVALID_ACL, None),
        ZkError::AuthFailed => build(ErrorKind::AuthFailed, codes::AUTH_FAILED, None),
        ZkError::QuotaExceeded => build(ErrorKind::QuotaExceeded, codes::QUOTA_EXCEEDED, None),
        ZkError::Unimplemented => build(ErrorKind::Unimplemented, codes::UNIMPLEMENTED, None),
        ZkError::Timeout => build(ErrorKind::Timeout, codes::TIMEOUT, None),
        ZkError::ConnectionLoss => build(ErrorKind::ConnectionLoss, codes::CONNECTION_LOSS, None),
        other => build(ErrorKind::Other, codes::OTHER, Some(other.to_string())),
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
