//! Tauri 侧共享状态。

use zkui_core::connection::ConnectionManager;

/// 应用状态：S1 只持有连接管理器。
pub struct AppState {
    pub manager: ConnectionManager,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            manager: ConnectionManager::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
