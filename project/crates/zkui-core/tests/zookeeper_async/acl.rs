//! 认证与 ACL 拒绝。

use zookeeper_async::{Acl, CreateMode, ZkError};

use super::common::{connect, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn creator_all_acl_requires_authentication() {
    let anonymous = connect().await;
    let authenticated = connect().await;
    authenticated
        .add_auth("digest", b"spike:spike".to_vec())
        .await
        .expect("认证失败");

    let path = unique("zka-auth");
    authenticated
        .create(
            &path,
            b"secret".to_vec(),
            Acl::creator_all().to_vec(),
            CreateMode::Persistent,
        )
        .await
        .expect("创建失败");

    let denied = anonymous.get_data(&path, false).await;
    assert!(matches!(denied, Err(ZkError::NoAuth)), "实际 {denied:?}");

    let (data, _) = authenticated.get_data(&path, false).await.expect("读取失败");
    assert_eq!(data, b"secret");

    authenticated.delete(&path, None).await.expect("清理失败");
}
