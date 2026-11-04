//! 连接与会话管理。

pub mod handle;
pub mod identity;
pub mod manager;
pub mod state;

pub use crate::ipc::connection::CreateConnection;
pub use handle::ConnectionHandle;
pub use identity::{AuthScheme, Identity};
pub use manager::ConnectionManager;
pub use state::ConnectionState;
