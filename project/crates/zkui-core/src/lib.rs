//! ZK UI 领域核心。
//!
//! 连接、会话、generation、树缓存、命令执行与错误模型都在本 crate 内，
//! 不依赖 Tauri，便于单独测试与后续复用。
//!
//! 当前仅为骨架：具体模块随实现推进补充，接口以 doc comment 作为契约。

/// 返回 crate 版本，用于启动时自检与日志。
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
