//! 单个逻辑连接的句柄。

use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use tokio::sync::watch;
use zookeeper_client::Client;

use super::identity::Identity;
use super::state::ConnectionState;

/// 连接句柄：持有客户端、generation、状态广播与身份。
pub struct ConnectionHandle {
    id: String,
    client: Client,
    generation: AtomicU64,
    state: watch::Sender<ConnectionState>,
    identity: RwLock<Option<Identity>>,
    session_timeout: Duration,
}

impl ConnectionHandle {
    pub fn new(id: String, client: Client, identity: Option<Identity>) -> Self {
        let session_timeout = client.session_timeout();
        let (state, _) = watch::channel(ConnectionState::Connected);
        Self {
            id,
            client,
            generation: AtomicU64::new(0),
            state,
            identity: RwLock::new(identity),
            session_timeout,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }

    pub fn bump_generation(&self) -> u64 {
        self.generation.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn state(&self) -> ConnectionState {
        *self.state.borrow()
    }

    pub fn set_state(&self, next: ConnectionState) {
        if self.state() != next {
            let _ = self.state.send(next);
        }
    }

    pub fn subscribe_state(&self) -> watch::Receiver<ConnectionState> {
        self.state.subscribe()
    }

    pub fn identity(&self) -> Option<Identity> {
        self.identity
            .read()
            .expect("identity lock poisoned")
            .clone()
    }

    pub fn session_timeout(&self) -> Duration {
        self.session_timeout
    }
}
