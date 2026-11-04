//! 大节点读写边界（接近服务端默认 1 MiB 缓冲）。

use zookeeper_client::Error;

use super::common::{connect, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn large_payload_boundary() {
    let client = connect().await;
    let small_path = unique("zkc-large-ok");
    let big_path = unique("zkc-large-reject");
    let options = persistent().with_acls(open_acls());

    let medium = vec![b'x'; 512 * 1024];
    client
        .create(&small_path, &medium, &options)
        .await
        .expect("512 KiB 写入应成功");
    let (data, stat) = client.get_data(&small_path).await.expect("读取失败");
    assert_eq!(data.len(), medium.len());
    assert_eq!(data, medium);
    assert_eq!(stat.data_length as usize, medium.len());
    client.delete(&small_path, None).await.expect("清理失败");

    let oversized = vec![b'y'; 1024 * 1024 + 4096];
    let created = client.create(&big_path, &oversized, &options).await;
    println!("1 MiB + 4 KiB 写入结果: {created:?}");
    match created {
        Ok(_) => {
            let (data, _) = client.get_data(&big_path).await.expect("读取大节点失败");
            assert_eq!(data.len(), oversized.len());
            client.delete(&big_path, None).await.expect("清理失败");
        }
        Err(error) => {
            println!("大节点被拒绝的错误类型: {error:?}");
            assert!(!matches!(error, Error::NoNode), "拒绝原因不应是节点不存在");
        }
    }
}
