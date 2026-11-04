//! 应用配置：内嵌默认值 + 用户覆盖。

pub mod settings;
pub mod store;

pub use settings::{
    AuditSettings, GeneralSettings, NotificationSettings, ProxySettings, RetrySettings, Settings,
    TaskSettings,
};
pub use store::SettingsStore;
