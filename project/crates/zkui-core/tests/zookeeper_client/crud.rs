//! 节点 CRUD、CAS、节点类型与顺序后缀。

use zookeeper_client::{CreateMode, Error};

use super::common::{connect, delete_recursive, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn create_read_cas_delete() {
    let client = connect().await;
    let path = unique("zkc-crud");

    let options = persistent().with_acls(open_acls());
    client
        .create(&path, b"v0", &options)
        .await
        .expect("创建失败");

    let (data, stat) = client.get_data(&path).await.expect("读取失败");
    assert_eq!(data, b"v0");
    assert_eq!(stat.version, 0);

    let updated = client
        .set_data(&path, b"v1", Some(0))
        .await
        .expect("按版本写入失败");
    assert_eq!(updated.version, 1);

    let conflict = client.set_data(&path, b"v2", Some(0)).await;
    assert!(
        matches!(conflict, Err(Error::BadVersion)),
        "版本过期应返回 BadVersion，实际 {conflict:?}"
    );

    client.delete(&path, Some(1)).await.expect("删除失败");
    assert!(matches!(client.get_data(&path).await, Err(Error::NoNode)));
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn node_types_ephemeral_sequential_container() {
    let client = connect().await;
    let base = unique("zkc-types");
    let options = persistent().with_acls(open_acls());
    client
        .create(&base, b"", &options)
        .await
        .expect("创建父节点失败");

    let ephemeral = format!("{base}/ephemeral");
    let eph = CreateMode::Ephemeral.with_acls(open_acls());
    client
        .create(&ephemeral, b"e", &eph)
        .await
        .expect("创建临时节点失败");

    let seq = CreateMode::PersistentSequential.with_acls(open_acls());
    client
        .create(&format!("{base}/seq-"), b"s", &seq)
        .await
        .expect("创建顺序节点失败");

    let container = format!("{base}/container");
    let c = CreateMode::Container.with_acls(open_acls());
    client
        .create(&container, b"c", &c)
        .await
        .expect("创建容器节点失败");

    let ephemerals = client
        .list_ephemerals(&base)
        .await
        .expect("列出临时节点失败");
    assert!(ephemerals.iter().any(|p| p.ends_with("/ephemeral")));

    client
        .delete(&ephemeral, None)
        .await
        .expect("删除临时节点失败");
    client
        .delete(&container, None)
        .await
        .expect("删除容器节点失败");
    delete_recursive(&client, &base).await;
    assert!(matches!(client.get_data(&base).await, Err(Error::NoNode)));
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn sequential_suffix_sync_and_exists() {
    let client = connect().await;
    let base = unique("zkc-seq");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建失败");

    let seq = CreateMode::PersistentSequential.with_acls(open_acls());
    client
        .create(&format!("{base}/item-"), b"", &seq)
        .await
        .expect("创建顺序节点失败");
    client
        .create(&format!("{base}/item-"), b"", &seq)
        .await
        .expect("创建顺序节点失败");

    let (children, _) = client.get_children(&base).await.expect("读取子节点失败");
    assert_eq!(children.len(), 2);
    for name in &children {
        let suffix = name.rsplit('-').next().unwrap();
        assert_eq!(suffix.len(), 10, "顺序后缀应为 10 位，实际 {suffix}");
    }

    let (_, stat) = client.get_data(&base).await.expect("读取失败");
    let exists = client.check_stat(&base).await.expect("check_stat 失败");
    assert_eq!(exists.map(|s| s.version), Some(stat.version));

    client.sync(&base).await.expect("sync 失败");
    delete_recursive(&client, &base).await;
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn connect_and_session_state() {
    let client = connect().await;
    assert!(matches!(
        client.state(),
        zookeeper_client::SessionState::SyncConnected
    ));
    assert!(client.session_timeout() > std::time::Duration::ZERO);
    let _ = client.session_id();
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn count_descendants_matches_children() {
    let client = connect().await;
    let base = unique("zkc-descendants");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建失败");
    for i in 0..3 {
        client
            .create(&format!("{base}/c{i}"), b"", &options)
            .await
            .expect("创建子节点失败");
    }
    let count = client
        .count_descendants_number(&base)
        .await
        .expect("统计失败");
    assert_eq!(count, 3);
    delete_recursive(&client, &base).await;
}
