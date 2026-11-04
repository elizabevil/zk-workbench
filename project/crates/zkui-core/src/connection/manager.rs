//! 连接管理器：创建、关闭与快照。

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use tokio::sync::broadcast;
use zookeeper_client::{Client, SessionState};

use super::handle::ConnectionHandle;
use super::state::ConnectionState;
use crate::error::{AppResult, app};
use crate::ipc::connection::{ConnectionSnapshot, ConnectionStateEvent, CreateConnection};

struct Entry {
    handle: Arc<ConnectionHandle>,
    display_name: String,
}

/// 连接管理器。同一管理器下的连接互相隔离。
pub struct ConnectionManager {
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
        let (events, _) = broadcast::channel(256);
        Self {
            entries: RwLock::new(HashMap::new()),
            events,
            next_id: AtomicU64::new(1),
        }
    }

    /// 订阅连接状态事件。
    pub fn subscribe(&self) -> broadcast::Receiver<ConnectionStateEvent> {
        self.events.subscribe()
    }

    /// 建立连接并启动状态监听任务。
    pub async fn create(&self, request: CreateConnection) -> AppResult<ConnectionSnapshot> {
        if request.endpoint.trim().is_empty() {
            return Err(Box::new(app::invalid_argument("endpoint 不能为空")));
        }
        let client = Client::connect(&request.endpoint).await?;
        let id = format!("conn-{}", self.next_id.fetch_add(1, Ordering::SeqCst));
        let handle = Arc::new(ConnectionHandle::new(
            id.clone(),
            client,
            request.identity.clone(),
        ));

        self.spawn_state_watcher(handle.clone());

        let snapshot = ConnectionSnapshot {
            connection_id: id.clone(),
            display_name: request.display_name,
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
                },
            );

        Ok(snapshot)
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
            state: entry.handle.state(),
            generation: entry.handle.generation(),
            identity: entry.handle.identity(),
            session_timeout_ms: entry.handle.session_timeout().as_millis() as u64,
        })
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
