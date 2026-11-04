//! 错误分类与递归删除的非原子性。

use zookeeper_client::Error;

use super::common::{connect, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn error_paths_are_classified() {
    let client = connect().await;
    let base = unique("zkc-errors");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建失败");

    let duplicate = client.create(&base, b"", &options).await;
    assert!(matches!(duplicate, Err(Error::NodeExists)));

    let missing_parent = client.create("/zkc-missing-parent/child", b"", &options).await;
    assert!(matches!(missing_parent, Err(Error::NoNode)));

    let delete_missing = client.delete("/zkc-does-not-exist", None).await;
    assert!(matches!(delete_missing, Err(Error::NoNode)));

    client.delete(&base, None).await.expect("清理失败");
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn recursive_delete_is_not_atomic() {
    let client = connect().await;
    let base = unique("zkc-recursive");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建失败");
    let child = format!("{base}/child");
    client.create(&child, b"", &options).await.expect("创建子节点失败");

    let not_empty = client.delete(&base, None).await;
    assert!(matches!(not_empty, Err(Error::NotEmpty)));

    client.delete(&child, None).await.expect("删除子节点失败");
    client.delete(&base, None).await.expect("删除父节点失败");
}
