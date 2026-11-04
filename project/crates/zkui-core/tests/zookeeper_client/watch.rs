//! 一次性与持久 Watcher。

use std::time::Duration;

use zookeeper_client::AddWatchMode;

use super::common::{connect, delete_recursive, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn oneshot_data_watch_receives_change() {
    let client = connect().await;
    let path = unique("zkc-watch-data");
    client
        .create(&path, b"v0", &persistent().with_acls(open_acls()))
        .await
        .expect("创建失败");

    let (_data, _stat, watcher) = client
        .get_and_watch_data(&path)
        .await
        .expect("注册 watch 失败");
    client.set_data(&path, b"v1", None).await.expect("写入失败");

    let event = tokio::time::timeout(Duration::from_secs(10), watcher.changed())
        .await
        .expect("等待 watch 事件超时");
    assert_eq!(event.path, path);
    client.delete(&path, None).await.expect("清理失败");
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn children_watch_receives_creation() {
    let client = connect().await;
    let base = unique("zkc-watch-children");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建失败");

    let (_children, _stat, watcher) = client
        .get_and_watch_children(&base)
        .await
        .expect("注册子节点 watch 失败");
    client
        .create(&format!("{base}/child"), b"", &options)
        .await
        .expect("创建子节点失败");

    let event = tokio::time::timeout(Duration::from_secs(10), watcher.changed())
        .await
        .expect("等待子节点事件超时");
    assert_eq!(event.path, base);
    delete_recursive(&client, &base).await;
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn persistent_watch_receives_multiple_changes() {
    let client = connect().await;
    let path = unique("zkc-watch-persistent");
    client
        .create(&path, b"v0", &persistent().with_acls(open_acls()))
        .await
        .expect("创建失败");

    let mut watcher = client
        .watch(&path, AddWatchMode::Persistent)
        .await
        .expect("注册持久 watch 失败");

    client
        .set_data(&path, b"v1", None)
        .await
        .expect("第一次写入失败");
    let first = tokio::time::timeout(Duration::from_secs(10), watcher.changed())
        .await
        .expect("等待第一次事件超时");
    assert_eq!(first.path, path);

    client
        .set_data(&path, b"v2", None)
        .await
        .expect("第二次写入失败");
    let second = tokio::time::timeout(Duration::from_secs(10), watcher.changed())
        .await
        .expect("等待第二次事件超时");
    assert_eq!(second.path, path);

    client.delete(&path, None).await.expect("清理失败");
    let _ = client.get_data(&path).await;
}
