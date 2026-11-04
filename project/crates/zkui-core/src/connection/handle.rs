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
    /// 逻辑连接 ID，仅在当前进程内唯一，用于前端引用与日志关联。
    id: String,
    /// ZooKeeper 客户端（可克隆）；连接关闭后不可再用。
    client: Client,
    /// 会话代数：每次断线重连或会话过期后递增，用于树/数据缓存失效。
    generation: AtomicU64,
    /// 状态广播发送端；订阅者通过 `subscribe_state` 获取变化。
    state: watch::Sender<ConnectionState>,
    /// 非秘密认证身份（仅展示）；未认证时为 None。
    identity: RwLock<Option<Identity>>,
    /// 与服务端协商后的会话超时时间。
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
