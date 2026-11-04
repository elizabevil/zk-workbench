//! 认证身份的非秘密表示。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// ZooKeeper 支持的认证 scheme。首版只启用 `digest`，其余保留枚举值用于兼容与报错。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum AuthScheme {
    Digest,
    Sasl,
    Ip,
    X509,
}

impl AuthScheme {
    pub const fn as_str(self) -> &'static str {
        match self {
            AuthScheme::Digest => "digest",
            AuthScheme::Sasl => "sasl",
            AuthScheme::Ip => "ip",
            AuthScheme::X509 => "x509",
        }
    }

    pub fn parse_scheme(value: &str) -> Option<Self> {
        match value {
            "digest" => Some(AuthScheme::Digest),
            "sasl" => Some(AuthScheme::Sasl),
            "ip" => Some(AuthScheme::Ip),
            "x509" => Some(AuthScheme::X509),
            _ => None,
        }
    }
}

/// 当前连接实际使用的认证身份。只保留展示信息，不包含凭据。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Identity {
    /// 认证 scheme；首版只接受 `digest`。
    pub scheme: AuthScheme,
    /// 展示用标识，例如用户名；不得包含密码或摘要。
    pub display: String,
}
