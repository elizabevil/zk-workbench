//! `zookeeper-async` 5.0.0 能力对照，与 `zookeeper_client` 模块保持同一组场景。

mod zookeeper_async {
    pub mod acl;
    pub mod common;
    pub mod crud;
    pub mod errors;
    pub mod limits;
    pub mod watch;
}
