//! `zookeeper-client` 0.11.2 能力对照，按关注点拆分模块。
//!
//! 所有用例默认 `#[ignore]`，需要真实 ZooKeeper 环境：
//!
//! ```bash
//! CARGO_HOME=$PWD/.cargo-home ZK_SPIKE_ENDPOINT=127.0.0.1:2181 \
//! cargo test -p zkui-core --test zookeeper_client -- --ignored --test-threads=1
//! ```

mod zookeeper_client {
    pub mod acl;
    pub mod cleanup;
    pub mod common;
    pub mod concurrency;
    pub mod crud;
    pub mod errors;
    pub mod limits;
    pub mod multi;
    pub mod watch;
}
