//! 应用配置：内嵌默认值 + 用户覆盖。

pub mod constants;
pub mod locale;
pub mod settings;
pub mod store;

pub use locale::Locale;
pub use settings::{
    AppearanceSettings, AuditSettings, GeneralSettings, LogSettings, NotificationSettings,
    ObservabilitySettings, OtelSettings, ProxySettings, RetrySettings, Settings, TaskSettings,
};
pub use store::SettingsStore;
