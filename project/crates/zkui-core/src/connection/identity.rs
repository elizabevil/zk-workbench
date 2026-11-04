//! 认证身份的非秘密表示。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 当前连接实际使用的认证身份。只保留展示信息，不包含凭据。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Identity {
    /// 认证 scheme，例如 `digest`。
    pub scheme: String,
    /// 展示用标识，例如用户名；不得包含密码或摘要。
    pub display: String,
}
