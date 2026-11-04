//! 公共辅助：连接、唯一路径、递归删除。

use zookeeper_client::{Acls, Client, CreateMode};

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

pub async fn connect() -> Client {
    let endpoint = endpoint();
    Client::connect(&endpoint)
        .await
        .expect("连接 ZooKeeper 失败")
}

pub fn persistent() -> CreateMode {
    CreateMode::Persistent
}

pub fn open_acls() -> zookeeper_client::Acls<'static> {
    Acls::anyone_all()
}

pub async fn delete_recursive(client: &Client, path: &str) {
    let children = client
        .get_children(path)
        .await
        .map(|(children, _)| children)
        .unwrap_or_default();
    for child in children {
        let child_path = format!("{path}/{child}");
        Box::pin(delete_recursive(client, &child_path)).await;
    }
    let _ = client.delete(path, None).await;
}
