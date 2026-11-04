//! Tauri 事件名常量。
//!
//! 命名规则：`<domain>.<entity>.<action>`，小写、点分隔，避免将来跨传输层时
//! 冒号带来的转义问题；清单集中在此，前后端各自引用同一组名字。

/// 连接状态变化。
pub const CONNECTION_STATE_CHANGED: &str = "connection.state.changed";

/// 树节点缓存失效。
pub const TREE_NODE_INVALIDATED: &str = "tree.node.invalidated";

/// 长操作进度。
pub const OPERATION_PROGRESS: &str = "operation.progress.changed";
