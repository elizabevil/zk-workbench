//! 连接档案：本地持久化，重启后仍可列出并连接。

use std::path::{Path, PathBuf};
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::error::{AppResult, app};

/// 已保存的连接档案。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub group: Option<String>,
}

/// 新建或更新档案的入参；`id` 为空表示新建。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, TS)]
#[ts(export)]
pub struct ProfileInput {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String,
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub group: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ProfilesFile {
    #[serde(default)]
    next_id: u64,
    #[serde(default)]
    profiles: Vec<ConnectionProfile>,
}

/// 档案存储。文件位于 `<config_dir>/zkui/connections.toml`。
pub struct ProfileStore {
    path: PathBuf,
    inner: RwLock<ProfilesFile>,
}

impl ProfileStore {
    pub fn with_default_path() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zkui")
            .join("connections.toml");
        Self::with_path(path)
    }

    pub fn with_path(path: PathBuf) -> Self {
        let inner = load_from(&path).unwrap_or_else(|error| {
            tracing::warn!(?error, path = ?path, "failed to load connection profiles, using defaults");
            ProfilesFile::default()
        });
        Self {
            path,
            inner: RwLock::new(inner),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn list(&self) -> Vec<ConnectionProfile> {
        self.inner
            .read()
            .expect("profiles lock poisoned")
            .profiles
            .clone()
    }

    /// 新建或更新档案。更新时保留原 id。
    pub fn save(&self, input: ProfileInput) -> AppResult<ConnectionProfile> {
        validate(&input)?;
        let mut inner = self.inner.write().expect("profiles lock poisoned");
        let profile = match input.id.as_deref() {
            Some(id) => {
                let existing = inner
                    .profiles
                    .iter_mut()
                    .find(|profile| profile.id == id)
                    .ok_or_else(|| Box::new(app::not_found(id)))?;
                existing.name = input.name;
                existing.host = input.host;
                existing.port = input.port;
                existing.group = input.group;
                existing.clone()
            }
            None => {
                inner.next_id += 1;
                let profile = ConnectionProfile {
                    id: format!("profile-{}", inner.next_id),
                    name: input.name,
                    host: input.host,
                    port: input.port,
                    group: input.group,
                };
                inner.profiles.push(profile.clone());
                profile
            }
        };
        persist(&self.path, &inner)?;
        Ok(profile)
    }

    pub fn delete(&self, id: &str) -> AppResult<()> {
        let mut inner = self.inner.write().expect("profiles lock poisoned");
        let before = inner.profiles.len();
        inner.profiles.retain(|profile| profile.id != id);
        if inner.profiles.len() == before {
            return Err(Box::new(app::not_found(id)));
        }
        persist(&self.path, &inner)?;
        Ok(())
    }
}

fn validate(input: &ProfileInput) -> AppResult<()> {
    if input.host.trim().is_empty() {
        return Err(Box::new(app::invalid_argument("host must not be empty")));
    }
    if input.port == 0 {
        return Err(Box::new(app::invalid_argument(
            "port must be greater than 0",
        )));
    }
    if input.name.trim().is_empty() {
        return Err(Box::new(app::invalid_argument("name must not be empty")));
    }
    Ok(())
}

fn load_from(path: &Path) -> Result<ProfilesFile, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(ProfilesFile::default());
    }
    let text = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}

fn persist(path: &Path, inner: &ProfilesFile) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            Box::new(app::invalid_argument(&format!(
                "failed to create config directory: {error}"
            )))
        })?;
    }
    let text = toml::to_string_pretty(inner).map_err(|error| {
        Box::new(app::invalid_argument(&format!(
            "failed to serialize profiles: {error}"
        )))
    })?;
    let temp = path.with_extension("toml.tmp");
    std::fs::write(&temp, text).map_err(|error| {
        Box::new(app::invalid_argument(&format!(
            "failed to write profiles: {error}"
        )))
    })?;
    std::fs::rename(&temp, path).map_err(|error| {
        Box::new(app::invalid_argument(&format!(
            "failed to replace profiles: {error}"
        )))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(host: &str, port: u16) -> ProfileInput {
        ProfileInput {
            id: None,
            name: "local".to_string(),
            host: host.to_string(),
            port,
            group: None,
        }
    }

    #[test]
    fn create_update_delete_roundtrip() {
        let dir = std::env::temp_dir().join(format!("zkui-profiles-{}", std::process::id()));
        let path = dir.join("connections.toml");
        let store = ProfileStore::with_path(path.clone());
        assert!(store.list().is_empty());

        let created = store.save(input("127.0.0.1", 2181)).expect("create");
        assert_eq!(created.id, "profile-1");
        assert_eq!(store.list().len(), 1);

        let updated = store
            .save(ProfileInput {
                id: Some(created.id.clone()),
                name: "renamed".to_string(),
                host: "192.168.11.5".to_string(),
                port: 2182,
                group: Some("prod".to_string()),
            })
            .expect("update");
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "renamed");

        let reloaded = ProfileStore::with_path(path.clone());
        assert_eq!(reloaded.list(), vec![updated.clone()]);

        reloaded.delete(&created.id).expect("delete");
        assert!(reloaded.list().is_empty());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn invalid_port_is_rejected() {
        let dir = std::env::temp_dir().join(format!("zkui-profiles-bad-{}", std::process::id()));
        let store = ProfileStore::with_path(dir.join("connections.toml"));
        assert!(store.save(input("127.0.0.1", 0)).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }
}
