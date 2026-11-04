//! Tauri 侧共享状态。

use std::sync::Arc;

use zkui_core::config::SettingsStore;
use zkui_core::connection::ConnectionManager;

/// 应用状态：设置存储与连接管理器共享同一份配置。
pub struct AppState {
    pub settings: Arc<SettingsStore>,
    pub manager: ConnectionManager,
}

impl AppState {
    pub fn new() -> Self {
        let settings = Arc::new(SettingsStore::with_default_path());
        let manager = ConnectionManager::with_settings(Some(settings.clone()));
        Self { settings, manager }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
