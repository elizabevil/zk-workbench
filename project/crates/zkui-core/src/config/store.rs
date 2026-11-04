//! 设置存储：内嵌默认值 + 用户覆盖文件。

use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::config::settings::Settings;
use crate::error::{AppResult, app};

/// 设置存储。读取失败时回退默认值并记录日志，不阻塞启动。
pub struct SettingsStore {
    path: PathBuf,
    current: RwLock<Settings>,
}

impl SettingsStore {
    /// 使用默认配置路径：`<config_dir>/zkui/app.toml`。
    pub fn with_default_path() -> Self {
        Self::with_path(default_path())
    }

    pub fn with_path(path: PathBuf) -> Self {
        let settings = load_from(&path).unwrap_or_else(|error| {
            tracing::warn!(?error, path = ?path, "读取配置失败，使用默认值");
            Settings::default()
        });
        Self {
            path,
            current: RwLock::new(settings),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn snapshot(&self) -> Settings {
        self.current.read().expect("settings lock poisoned").clone()
    }

    /// 保存并立即生效。写入使用临时文件 + 重命名，避免半截文件。
    pub fn save(&self, settings: Settings) -> AppResult<Settings> {
        validate(&settings)?;
        persist(&self.path, &settings)?;
        *self.current.write().expect("settings lock poisoned") = settings.clone();
        Ok(settings)
    }

    /// 恢复默认并删除用户覆盖文件。
    pub fn reset(&self) -> AppResult<Settings> {
        if self.path.exists() {
            std::fs::remove_file(&self.path).map_err(|error| {
                Box::new(app::invalid_argument(&format!("删除配置失败: {error}")))
            })?;
        }
        let settings = Settings::default();
        *self.current.write().expect("settings lock poisoned") = settings.clone();
        Ok(settings)
    }
}

fn default_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zkui")
        .join("app.toml")
}

fn load_from(path: &Path) -> Result<Settings, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(Settings::default());
    }
    let text = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}

fn persist(path: &Path, settings: &Settings) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            Box::new(app::invalid_argument(&format!("创建配置目录失败: {error}")))
        })?;
    }
    let text = toml::to_string_pretty(settings)
        .map_err(|error| Box::new(app::invalid_argument(&format!("序列化配置失败: {error}"))))?;
    let temp = path.with_extension("toml.tmp");
    std::fs::write(&temp, text)
        .map_err(|error| Box::new(app::invalid_argument(&format!("写入配置失败: {error}"))))?;
    std::fs::rename(&temp, path)
        .map_err(|error| Box::new(app::invalid_argument(&format!("替换配置失败: {error}"))))?;
    Ok(())
}

fn validate(settings: &Settings) -> AppResult<()> {
    if settings.retry.max_attempts == 0 {
        return Err(Box::new(app::invalid_argument("重试次数必须大于 0")));
    }
    if settings.retry.initial_interval_ms == 0 || settings.retry.max_interval_ms == 0 {
        return Err(Box::new(app::invalid_argument("重试间隔必须大于 0")));
    }
    if settings.retry.initial_interval_ms > settings.retry.max_interval_ms {
        return Err(Box::new(app::invalid_argument("初始间隔不能大于最大间隔")));
    }
    if settings.audit.retention_days == 0 {
        return Err(Box::new(app::invalid_argument("审计保留期必须大于 0")));
    }
    for profile in &settings.proxy.profiles {
        if profile.host.trim().is_empty() {
            return Err(Box::new(app::invalid_argument("代理主机不能为空")));
        }
        if profile.port == 0 {
            return Err(Box::new(app::invalid_argument("代理端口必须大于 0")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_when_file_missing() {
        let dir = std::env::temp_dir().join(format!("zkui-test-{}", std::process::id()));
        let path = dir.join("app.toml");
        let store = SettingsStore::with_path(path.clone());
        assert_eq!(store.snapshot(), Settings::default());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn save_and_reload_roundtrip() {
        let dir = std::env::temp_dir().join(format!("zkui-roundtrip-{}", std::process::id()));
        let path = dir.join("app.toml");
        let store = SettingsStore::with_path(path.clone());

        let mut settings = Settings::default();
        settings.general.locale = "en-US".to_string();
        settings.retry.max_attempts = 3;
        store.save(settings.clone()).expect("保存失败");

        let reloaded = SettingsStore::with_path(path.clone());
        assert_eq!(reloaded.snapshot(), settings);

        reloaded.reset().expect("重置失败");
        assert_eq!(reloaded.snapshot(), Settings::default());
        assert!(!path.exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn invalid_retry_is_rejected() {
        let dir = std::env::temp_dir().join(format!("zkui-invalid-{}", std::process::id()));
        let store = SettingsStore::with_path(dir.join("app.toml"));
        let mut settings = Settings::default();
        settings.retry.max_attempts = 0;
        assert!(store.save(settings).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }
}
