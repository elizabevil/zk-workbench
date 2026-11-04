//! ZK UI 领域核心。
//!
//! 连接、会话、generation、树缓存、节点操作与错误模型都在本 crate 内，
//! 不依赖 Tauri，便于单独测试与复用。接口以 doc comment 作为契约。

pub mod audit;
pub mod config;
pub mod connection;
pub mod error;
pub mod ipc;
pub mod metrics;
pub mod otel;
pub mod profiles;
pub mod secrets;
pub mod transport;

/// 返回 crate 版本，用于启动自检与日志。
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_not_empty() {
        assert!(!super::version().is_empty());
    }
}
