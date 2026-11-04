//! 并发 CAS：同一版本只允许一个赢家。

use super::common::{connect, open_acls, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn concurrent_cas_has_single_winner() {
    let client = connect().await;
    let path = unique("zkc-concurrent");
    client
        .create(&path, b"v0", &persistent().with_acls(open_acls()))
        .await
        .expect("创建失败");

    let mut handles = Vec::new();
    for i in 0..8 {
        let client = client.clone();
        let path = path.clone();
        handles.push(tokio::spawn(async move {
            client
                .set_data(&path, format!("v{i}").as_bytes(), Some(0))
                .await
                .is_ok()
        }));
    }

    let mut success = 0;
    for handle in handles {
        if handle.await.expect("任务失败") {
            success += 1;
        }
    }
    assert_eq!(success, 1, "同一版本的并发写入应只有一个成功");

    let (_, stat) = client.get_data(&path).await.expect("读取失败");
    assert_eq!(stat.version, 1);
    client.delete(&path, Some(1)).await.expect("清理失败");
}
