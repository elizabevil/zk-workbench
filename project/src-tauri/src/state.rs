//! Tauri 侧共享状态。

use std::sync::{Arc, Mutex};

use zkui_core::audit::AuditStore;
use zkui_core::config::SettingsStore;
use zkui_core::connection::ConnectionManager;
use zkui_core::profiles::ProfileStore;

/// 应用状态：设置存储与连接管理器共享同一份配置。
#[derive(Default)]
pub struct OtelRuntime {
    pub last_export_ts: Option<i64>,
    pub last_error: Option<String>,
}

pub struct AppState {
    pub settings: Arc<SettingsStore>,
    pub audit: Arc<AuditStore>,
    pub profiles: Arc<ProfileStore>,
    pub manager: ConnectionManager,
    pub otel: Mutex<OtelRuntime>,
}

impl AppState {
    pub fn new() -> Self {
        let settings = Arc::new(SettingsStore::with_default_path());
        let profiles = Arc::new(ProfileStore::with_default_path());
        let audit_path = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("zkui")
            .join("audit.db");
        let audit = Arc::new(AuditStore::open(&audit_path).expect("failed to open audit database"));
        let manager = ConnectionManager::with_settings(Some(settings.clone()));
        Self {
            otel: Mutex::new(OtelRuntime::default()),
            settings,
            audit,
            profiles,
            manager,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
