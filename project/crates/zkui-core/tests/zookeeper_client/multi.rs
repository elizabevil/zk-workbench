//! 原子多写与版本检查写。

use zookeeper_client::{Acls, CreateMode};

use super::common::{connect, delete_recursive, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn multi_writer_creates_atomically() {
    let client = connect().await;
    let base = unique("zkc-multi");
    let options = persistent().with_acls(open_acls());
    client.create(&base, b"", &options).await.expect("创建父节点失败");

    let first = format!("{base}/a");
    let second = format!("{base}/b");
    let mut writer = client.new_multi_writer();
    let create_options = CreateMode::Persistent.with_acls(Acls::anyone_all());
    writer.add_create(&first, b"a", &create_options).expect("加入创建失败");
    writer.add_create(&second, b"b", &create_options).expect("加入创建失败");

    let results = writer.commit().await.expect("原子多写失败");
    assert_eq!(results.len(), 2);
    assert!(client.get_data(&first).await.is_ok());
    assert!(client.get_data(&second).await.is_ok());

    delete_recursive(&client, &base).await;
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn check_writer_rejects_stale_version() {
    let client = connect().await;
    let path = unique("zkc-check");
    client
        .create(&path, b"v0", &persistent().with_acls(open_acls()))
        .await
        .expect("创建失败");

    client.set_data(&path, b"v1", Some(0)).await.expect("先推进版本");

    let mut writer = client.new_check_writer(&path, Some(0)).expect("创建 check writer 失败");
    writer.add_set_data(&path, b"stale", None).expect("加入写入失败");
    let result = writer.commit().await;
    assert!(result.is_err(), "版本已过期，检查写应失败");

    let (data, stat) = client.get_data(&path).await.expect("读取失败");
    assert_eq!(data, b"v1");
    assert_eq!(stat.version, 1);
    client.delete(&path, None).await.expect("清理失败");
}
