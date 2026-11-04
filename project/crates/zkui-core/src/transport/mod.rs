//! 传输层：直连、SOCKS5、HTTP CONNECT 与本地中继。
//!
//! ZooKeeper 客户端只连接 `127.0.0.1` 上的临时端口，中继负责按代理配置建立上游连接。

pub mod no_proxy;
pub mod proxy;
pub mod relay;
pub mod upstream;

pub use proxy::{ProxyKind, ProxyProfile};
pub use relay::RelayHandle;
