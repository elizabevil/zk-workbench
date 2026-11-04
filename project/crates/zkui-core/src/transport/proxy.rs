//! 代理档案类型。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 代理协议。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(
    rename_all = "snake_case",
    export,
    export_to = "../../../src/api/types/"
)]
pub enum ProxyKind {
    Socks5,
    HttpConnect,
}

/// 单条代理档案。`password` 只存在于内存，持久化时写入系统凭据存储。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub struct ProxyProfile {
    pub name: String,
    pub kind: ProxyKind,
    pub host: String,
    pub port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl ProxyProfile {
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
