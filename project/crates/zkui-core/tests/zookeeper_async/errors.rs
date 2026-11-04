//! 错误分类。

use zookeeper_async::{CreateMode, ZkError};

use super::common::{connect, open_acls, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn error_paths_are_classified() {
    let zk = connect().await;
    let base = unique("zka-errors");
    zk.create(&base, Vec::new(), open_acls(), CreateMode::Persistent)
        .await
        .expect("创建失败");

    let duplicate = zk
        .create(&base, Vec::new(), open_acls(), CreateMode::Persistent)
        .await;
    assert!(matches!(duplicate, Err(ZkError::NodeExists)));

    let missing_parent = zk
        .create(
            "/zka-missing/child",
            Vec::new(),
            open_acls(),
            CreateMode::Persistent,
        )
        .await;
    assert!(matches!(missing_parent, Err(ZkError::NoNode)));

    let delete_missing = zk.delete("/zka-does-not-exist", None).await;
    assert!(matches!(delete_missing, Err(ZkError::NoNode)));

    zk.delete(&base, None).await.expect("清理失败");
}
