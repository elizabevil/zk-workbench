//! 连接管理器：创建、关闭与快照。

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use base64::Engine;
use tokio::sync::broadcast;
use zookeeper_client::{Client, SessionState};

use super::handle::ConnectionHandle;
use super::state::ConnectionState;
use crate::config::{Settings, SettingsStore};
use crate::error::{AppError, AppResult, app};
use crate::ipc::connection::{ConnectionSnapshot, ConnectionStateEvent, CreateConnection};
use crate::ipc::node::{NodeData, NodeStat};
use crate::transport::{RelayHandle, no_proxy, proxy::ProxyProfile, relay};

struct Entry {
    handle: Arc<ConnectionHandle>,
    display_name: String,
    host: String,
    port: u16,
    _relay: Option<RelayHandle>,
}

/// 连接管理器。同一管理器下的连接互相隔离。
pub struct ConnectionManager {
    settings: Option<std::sync::Arc<SettingsStore>>,
    entries: RwLock<HashMap<String, Entry>>,
    events: broadcast::Sender<ConnectionStateEvent>,
    next_id: AtomicU64,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self::with_settings(None)
    }

    /// 使用共享设置存储；传 None 时使用默认设置。
    pub fn with_settings(settings: Option<std::sync::Arc<SettingsStore>>) -> Self {
        let (events, _) = broadcast::channel(256);
        Self {
            settings,
            entries: RwLock::new(HashMap::new()),
            events,
            next_id: AtomicU64::new(1),
        }
    }

    fn settings_snapshot(&self) -> Settings {
        self.settings
            .as_ref()
            .map(|store| store.snapshot())
            .unwrap_or_default()
    }

    /// 订阅连接状态事件。
    pub fn subscribe(&self) -> broadcast::Receiver<ConnectionStateEvent> {
        self.events.subscribe()
    }

    /// 建立连接并启动状态监听任务。
    pub async fn create(&self, request: CreateConnection) -> AppResult<ConnectionSnapshot> {
        if request.host.trim().is_empty() {
            return Err(Box::new(app::invalid_argument("host must not be empty")));
        }
        if request.port == 0 {
            return Err(Box::new(app::invalid_argument(
                "port must be greater than 0",
            )));
        }
        let endpoint = format!("{}:{}", request.host.trim(), request.port);
        let settings = self.settings_snapshot();
        let proxies = resolve_proxies(&request.host, request.proxy.as_ref(), &settings);
        let attempts = if settings.retry.auto_reconnect {
            settings.retry.max_attempts.max(1)
        } else {
            1
        };

        let mut last_error: Option<Box<AppError>> = None;
        let mut established: Option<(Client, Option<RelayHandle>)> = None;
        'attempts: for attempt in 0..attempts {
            for proxy in &proxies {
                match self.try_connect(&endpoint, proxy.as_ref()).await {
                    Ok(pair) => {
                        established = Some(pair);
                        break 'attempts;
                    }
                    Err(error) => last_error = Some(error),
                }
            }
            if attempt + 1 < attempts {
                let delay = settings.retry.delay_for(attempt);
                tracing::warn!(
                    attempt = attempt + 1,
                    delay_ms = delay,
                    "connection failed, retrying"
                );
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }
        }
        let (client, relay) = established.ok_or_else(|| {
            last_error.unwrap_or_else(|| {
                Box::new(app::invalid_argument("unable to establish connection"))
            })
        })?;
        let id = format!("conn-{}", self.next_id.fetch_add(1, Ordering::SeqCst));
        let handle = Arc::new(ConnectionHandle::new(
            id.clone(),
            client,
            request.identity.clone(),
        ));

        self.spawn_state_watcher(handle.clone());

        let snapshot = ConnectionSnapshot {
            connection_id: id.clone(),
            display_name: request.display_name.clone(),
            host: request.host.clone(),
            port: request.port,
            state: handle.state(),
            generation: handle.generation(),
            identity: handle.identity(),
            session_timeout_ms: handle.session_timeout().as_millis() as u64,
        };

        self.entries
            .write()
            .expect("connections lock poisoned")
            .insert(
                id,
                Entry {
                    handle,
                    display_name: snapshot.display_name.clone(),
                    host: request.host.clone(),
                    port: request.port,
                    _relay: relay,
                },
            );

        Ok(snapshot)
    }

    async fn try_connect(
        &self,
        endpoint: &str,
        proxy: Option<&ProxyProfile>,
    ) -> AppResult<(Client, Option<RelayHandle>)> {
        match proxy {
            Some(profile) => {
                let mut profile = profile.clone();
                if profile.username.is_some() && profile.password.is_none() {
                    profile.password = crate::secrets::get_proxy_password(&profile.name)?;
                }
                let relay = relay::start(endpoint.to_string(), profile).await?;
                match Client::connect(&relay.endpoint()).await {
                    Ok(client) => Ok((client, Some(relay))),
                    Err(error) => Err(Box::new(error.into())),
                }
            }
            None => match Client::connect(endpoint).await {
                Ok(client) => Ok((client, None)),
                Err(error) => Err(Box::new(error.into())),
            },
        }
    }

    /// 关闭连接并广播关闭事件。
    pub fn close(&self, connection_id: &str) -> AppResult<()> {
        let entry = self
            .entries
            .write()
            .expect("connections lock poisoned")
            .remove(connection_id)
            .ok_or_else(|| Box::new(app::not_found(connection_id)))?;

        entry.handle.set_state(ConnectionState::Closed);
        let _ = self.events.send(ConnectionStateEvent {
            connection_id: connection_id.to_string(),
            state: ConnectionState::Closed,
            generation: entry.handle.generation(),
        });
        Ok(())
    }

    /// 读取连接快照。
    pub fn snapshot(&self, connection_id: &str) -> AppResult<ConnectionSnapshot> {
        let entries = self.entries.read().expect("connections lock poisoned");
        let entry = entries
            .get(connection_id)
            .ok_or_else(|| Box::new(app::not_found(connection_id)))?;
        Ok(ConnectionSnapshot {
            connection_id: entry.handle.id().to_string(),
            display_name: entry.display_name.clone(),
            host: entry.host.clone(),
            port: entry.port,
            state: entry.handle.state(),
            generation: entry.handle.generation(),
            identity: entry.handle.identity(),
            session_timeout_ms: entry.handle.session_timeout().as_millis() as u64,
        })
    }

    /// 读取节点原始数据与 Stat。
    pub async fn data(&self, connection_id: &str, path: &str) -> AppResult<NodeData> {
        let client = self.client_of(connection_id)?;
        let (bytes, stat) = client.get_data(path).await?;
        let is_text = std::str::from_utf8(&bytes).is_ok();
        Ok(NodeData {
            path: path.to_string(),
            data_b64: base64::engine::general_purpose::STANDARD.encode(&bytes),
            byte_len: bytes.len() as u64,
            is_text,
            stat: stat_of(path, &stat),
        })
    }

    fn client_of(&self, connection_id: &str) -> AppResult<Client> {
        let entries = self.entries.read().expect("connections lock poisoned");
        entries
            .get(connection_id)
            .map(|entry| entry.handle.client().clone())
            .ok_or_else(|| Box::new(app::not_found(connection_id)))
    }

    /// 读取直接子节点，按名称排序。
    pub async fn children(&self, connection_id: &str, path: &str) -> AppResult<Vec<String>> {
        let client = self.client_of(connection_id)?;
        let (mut children, _stat) = client.get_children(path).await?;
        children.sort();
        Ok(children)
    }

    /// 读取节点 Stat。
    pub async fn stat(&self, connection_id: &str, path: &str) -> AppResult<NodeStat> {
        let client = self.client_of(connection_id)?;
        let (_data, stat) = client.get_data(path).await?;
        Ok(NodeStat {
            path: path.to_string(),
            version: stat.version,
            cversion: stat.cversion,
            aversion: stat.aversion,
            data_length: stat.data_length,
            num_children: stat.num_children,
            ephemeral_owner: stat.ephemeral_owner,
            czxid: stat.czxid,
            mzxid: stat.mzxid,
            ctime: stat.ctime,
            mtime: stat.mtime,
        })
    }

    /// 列出全部连接快照，按连接 id 排序。
    pub fn list(&self) -> Vec<ConnectionSnapshot> {
        let entries = self.entries.read().expect("connections lock poisoned");
        let mut snapshots: Vec<ConnectionSnapshot> = entries
            .values()
            .map(|entry| ConnectionSnapshot {
                connection_id: entry.handle.id().to_string(),
                display_name: entry.display_name.clone(),
                host: entry.host.clone(),
                port: entry.port,
                state: entry.handle.state(),
                generation: entry.handle.generation(),
                identity: entry.handle.identity(),
                session_timeout_ms: entry.handle.session_timeout().as_millis() as u64,
            })
            .collect();
        snapshots.sort_by(|a, b| a.connection_id.cmp(&b.connection_id));
        snapshots
    }

    /// 校验请求携带的 generation 是否仍是最新。
    pub fn ensure_generation(&self, connection_id: &str, expected: u64) -> AppResult<()> {
        let entries = self.entries.read().expect("connections lock poisoned");
        let entry = entries
            .get(connection_id)
            .ok_or_else(|| Box::new(app::not_found(connection_id)))?;
        let actual = entry.handle.generation();
        if actual == expected {
            Ok(())
        } else {
            Err(Box::new(app::stale_generation(
                connection_id,
                expected,
                actual,
            )))
        }
    }

    fn spawn_state_watcher(&self, handle: Arc<ConnectionHandle>) {
        let events = self.events.clone();
        let mut watcher = handle.client().state_watcher();
        let mut previous = handle.state();
        tokio::spawn(async move {
            loop {
                let session_state = watcher.changed().await;
                let next = map_session_state(session_state);

                if matches!(session_state, SessionState::SyncConnected)
                    && matches!(
                        previous,
                        ConnectionState::Disconnected | ConnectionState::Expired
                    )
                {
                    let generation = handle.bump_generation();
                    handle.set_state(ConnectionState::Connected);
                    let _ = events.send(ConnectionStateEvent {
                        connection_id: handle.id().to_string(),
                        state: ConnectionState::Connected,
                        generation,
                    });
                } else if next != previous {
                    handle.set_state(next);
                    let _ = events.send(ConnectionStateEvent {
                        connection_id: handle.id().to_string(),
                        state: next,
                        generation: handle.generation(),
                    });
                }

                previous = next;
                if next.is_terminal() {
                    break;
                }
            }
        });
    }
}

fn map_session_state(state: SessionState) -> ConnectionState {
    match state {
        SessionState::Disconnected => ConnectionState::Disconnected,
        SessionState::SyncConnected | SessionState::ConnectedReadOnly => ConnectionState::Connected,
        SessionState::Expired => ConnectionState::Expired,
        SessionState::Closed => ConnectionState::Closed,
        SessionState::AuthFailed => ConnectionState::Expired,
    }
}

fn resolve_proxies(
    host: &str,
    override_proxy: Option<&ProxyProfile>,
    settings: &Settings,
) -> Vec<Option<ProxyProfile>> {
    if let Some(profile) = override_proxy {
        return vec![Some(profile.clone())];
    }
    if !settings.proxy.enabled {
        return vec![None];
    }
    if no_proxy::matches(host, &settings.proxy.no_proxy) {
        return vec![None];
    }
    if settings.proxy.profiles.is_empty() {
        return vec![None];
    }
    // 代理启用时只尝试配置的档案：全部失败即报错，不静默回退直连。
    settings
        .proxy
        .profiles
        .iter()
        .map(|profile| Some(profile.clone()))
        .collect()
}

fn stat_of(path: &str, stat: &zookeeper_client::Stat) -> NodeStat {
    NodeStat {
        path: path.to_string(),
        version: stat.version,
        cversion: stat.cversion,
        aversion: stat.aversion,
        data_length: stat.data_length,
        num_children: stat.num_children,
        ephemeral_owner: stat.ephemeral_owner,
        czxid: stat.czxid,
        mzxid: stat.mzxid,
        ctime: stat.ctime,
        mtime: stat.mtime,
    }
}
