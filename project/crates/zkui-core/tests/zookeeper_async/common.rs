//! 公共辅助。

use std::time::Duration;

use zookeeper_async::{Acl, ZooKeeper};

pub fn endpoint() -> String {
    std::env::var("ZK_SPIKE_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:2181".to_string())
}

pub fn unique(prefix: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("/{prefix}-{nanos}")
}

pub async fn connect() -> ZooKeeper {
    let endpoint = endpoint();
    ZooKeeper::connect(&endpoint, Duration::from_secs(10), |_| {})
        .await
        .expect("连接 ZooKeeper 失败")
}

pub fn open_acls() -> Vec<Acl> {
    Acl::open_unsafe().to_vec()
}

pub async fn delete_recursive(zk: &ZooKeeper, path: &str) {
    let children = zk.get_children(path, false).await.unwrap_or_default();
    for child in children {
        let child_path = format!("{path}/{child}");
        Box::pin(delete_recursive(zk, &child_path)).await;
    }
    let _ = zk.delete(path, None).await;
}
