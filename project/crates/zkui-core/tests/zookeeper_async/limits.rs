//! 大节点读写边界。

use zookeeper_async::{CreateMode, ZkError};

use super::common::{connect, open_acls, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn large_payload_boundary() {
    let zk = connect().await;
    let small_path = unique("zka-large-ok");
    let big_path = unique("zka-large-reject");

    let medium = vec![b'x'; 512 * 1024];
    zk.create(&small_path, medium.clone(), open_acls(), CreateMode::Persistent)
        .await
        .expect("512 KiB 写入应成功");
    let (data, stat) = zk.get_data(&small_path, false).await.expect("读取失败");
    assert_eq!(data, medium);
    assert_eq!(stat.data_length as usize, medium.len());
    zk.delete(&small_path, None).await.expect("清理失败");

    let oversized = vec![b'y'; 1024 * 1024 + 4096];
    let created = zk
        .create(&big_path, oversized.clone(), open_acls(), CreateMode::Persistent)
        .await;
    println!("1 MiB + 4 KiB 写入结果: {created:?}");
    match created {
        Ok(path) => {
            let (data, _) = zk.get_data(&path, false).await.expect("读取大节点失败");
            assert_eq!(data.len(), oversized.len());
            zk.delete(&path, None).await.expect("清理失败");
        }
        Err(error) => {
            if !matches!(error, ZkError::ConnectionLoss | ZkError::BadArguments) {
                println!("大节点被拒绝的错误类型: {error:?}");
            }
        }
    }
}
