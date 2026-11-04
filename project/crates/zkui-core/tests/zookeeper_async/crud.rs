//! CRUD、CAS、顺序路径。

use zookeeper_async::{CreateMode, ZkError};

use super::common::{connect, delete_recursive, open_acls, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn create_read_cas_delete() {
    let zk = connect().await;
    let path = unique("zka-crud");

    let created = zk
        .create(&path, b"v0".to_vec(), open_acls(), CreateMode::Persistent)
        .await
        .expect("创建失败");
    assert_eq!(created, path);

    let (data, stat) = zk.get_data(&path, false).await.expect("读取失败");
    assert_eq!(data, b"v0");
    assert_eq!(stat.version, 0);

    let updated = zk
        .set_data(&path, b"v1".to_vec(), Some(0))
        .await
        .expect("按版本写入失败");
    assert_eq!(updated.version, 1);

    let conflict = zk.set_data(&path, b"v2".to_vec(), Some(0)).await;
    assert!(matches!(conflict, Err(ZkError::BadVersion)));

    zk.delete(&path, Some(1)).await.expect("删除失败");
    assert!(matches!(zk.get_data(&path, false).await, Err(ZkError::NoNode)));
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn sequential_path_has_ten_digit_suffix() {
    let zk = connect().await;
    let base = unique("zka-seq");
    zk.create(&base, Vec::new(), open_acls(), CreateMode::Persistent)
        .await
        .expect("创建失败");

    let created = zk
        .create(
            &format!("{base}/item-"),
            Vec::new(),
            open_acls(),
            CreateMode::PersistentSequential,
        )
        .await
        .expect("创建顺序节点失败");
    let suffix = created.rsplit('-').next().unwrap();
    assert_eq!(suffix.len(), 10, "顺序后缀应为 10 位，实际 {suffix}");

    delete_recursive(&zk, &base).await;
}
