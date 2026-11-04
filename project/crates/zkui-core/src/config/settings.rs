//! 配置结构。密码不进入本文件，统一由系统凭据存储管理。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::constants::{DEFAULT_LOG_LEVEL, DEFAULT_THEME, OTEL_PROTOCOL_HTTP_PROTOBUF};
use crate::config::locale::Locale;
use crate::transport::ProxyProfile;

/// 全部应用设置。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct Settings {
    pub general: GeneralSettings,
    pub notifications: NotificationSettings,
    pub retry: RetrySettings,
    pub task: TaskSettings,
    pub proxy: ProxySettings,
    pub audit: AuditSettings,
    pub observability: ObservabilitySettings,
    pub logs: LogSettings,
    pub appearance: AppearanceSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct GeneralSettings {
    pub locale: Locale,
    pub theme: String,
    pub restore_last_connection: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            locale: Locale::ZhCn,
            theme: DEFAULT_THEME.to_string(),
            restore_last_connection: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct NotificationSettings {
    /// off | errors | warnings | all
    pub desktop: String,
    pub command_finished: bool,
    pub sound: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            desktop: crate::config::constants::NOTIFICATION_ERRORS.to_string(),
            command_finished: false,
            sound: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct RetrySettings {
    pub auto_reconnect: bool,
    pub max_attempts: u32,
    pub initial_interval_ms: u64,
    pub multiplier: f64,
    pub max_interval_ms: u64,
}

impl Default for RetrySettings {
    fn default() -> Self {
        Self {
            auto_reconnect: true,
            max_attempts: 5,
            initial_interval_ms: 3000,
            multiplier: 2.0,
            max_interval_ms: 10000,
        }
    }
}

impl RetrySettings {
    /// 第 `attempt` 次重试前的等待时间（从 0 开始计数），受上限约束。
    pub fn delay_for(&self, attempt: u32) -> u64 {
        let base = self.initial_interval_ms as f64 * self.multiplier.powi(attempt as i32);
        base.min(self.max_interval_ms as f64) as u64
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct TaskSettings {
    pub continue_on_error: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct ProxySettings {
    pub enabled: bool,
    pub no_proxy: Vec<String>,
    /// 按顺序尝试，全部失败才报错。
    pub profiles: Vec<ProxyProfile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct AppearanceSettings {
    /// 页面背景，接受任意 CSS background 值。
    pub background: String,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            background: "linear-gradient(135deg, #7c3aed, #2563eb)".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct ObservabilitySettings {
    pub otel: OtelSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct OtelSettings {
    pub enabled: bool,
    pub endpoint: String,
    /// 固定 http/protobuf，保留字段便于将来扩展。
    pub protocol: String,
    pub sample_ratio: f64,
}

impl Default for OtelSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: String::new(),
            protocol: OTEL_PROTOCOL_HTTP_PROTOBUF.to_string(),
            sample_ratio: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct LogSettings {
    pub level: String,
    /// 为空表示使用默认的文档目录下 `zkui/logs`。
    pub directory: String,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            level: DEFAULT_LOG_LEVEL.to_string(),
            directory: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(default)]
#[ts(export)]
pub struct AuditSettings {
    pub retention_days: u32,
}

impl Default for AuditSettings {
    fn default() -> Self {
        Self { retention_days: 90 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_documented_values() {
        let settings = Settings::default();
        assert_eq!(settings.retry.max_attempts, 5);
        assert_eq!(settings.retry.initial_interval_ms, 3000);
        assert_eq!(settings.retry.max_interval_ms, 10000);
        assert!(settings.retry.auto_reconnect);
        assert_eq!(settings.audit.retention_days, 90);
        assert!(!settings.proxy.enabled);
    }

    #[test]
    fn retry_delay_is_capped() {
        let retry = RetrySettings::default();
        assert_eq!(retry.delay_for(0), 3000);
        assert_eq!(retry.delay_for(1), 6000);
        assert_eq!(retry.delay_for(2), 10000);
        assert_eq!(retry.delay_for(5), 10000);
    }
}
