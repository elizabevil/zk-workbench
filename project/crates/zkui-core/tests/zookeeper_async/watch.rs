//! 一次性数据 watcher。

use std::time::Duration;

use zookeeper_async::{CreateMode, ZkError};

use super::common::{connect, open_acls, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn data_watch_receives_change() {
    let zk = connect().await;
    let path = unique("zka-watch");
    zk.create(&path, b"v0".to_vec(), open_acls(), CreateMode::Persistent)
        .await
        .expect("创建失败");

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    zk.get_data_w(&path, move |event| {
        let _ = tx.try_send(event);
    })
    .await
    .expect("注册 watch 失败");

    zk.set_data(&path, b"v1".to_vec(), None)
        .await
        .expect("写入失败");

    let event = tokio::time::timeout(Duration::from_secs(10), rx.recv())
        .await
        .expect("等待 watch 超时")
        .expect("通道关闭");
    assert_eq!(event.path.as_deref(), Some(path.as_str()));

    zk.delete(&path, None).await.expect("清理失败");
    let _ = ZkError::NoNode;
}
