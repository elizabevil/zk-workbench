//! 认证与 ACL 拒绝。

use zookeeper_client::{Acl, Acls, Error, Permission};

use super::common::{connect, persistent, unique};

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn auth_scheme_acl_requires_authentication() {
    let anonymous = connect().await;
    let authenticated = connect().await;
    authenticated
        .auth("digest", b"spike:spike")
        .await
        .expect("认证失败");

    let path = unique("zkc-auth");
    let acl_list = [Acl::new_const(Permission::ALL, "auth", "")];
    let acls = Acls::new(&acl_list);
    let options = persistent().with_acls(acls);
    authenticated.create(&path, b"secret", &options).await.expect("创建失败");

    let denied = anonymous.get_data(&path).await;
    assert!(matches!(denied, Err(Error::NoAuth)), "未认证应被拒绝，实际 {denied:?}");

    let denied_write = anonymous.set_data(&path, b"x", None).await;
    assert!(matches!(denied_write, Err(Error::NoAuth)));

    let (data, _) = authenticated.get_data(&path).await.expect("认证读取失败");
    assert_eq!(data, b"secret");

    let (acls, _) = authenticated.get_acl(&path).await.expect("读取 ACL 失败");
    assert_eq!(acls.len(), 1);

    authenticated.delete(&path, None).await.expect("清理失败");
}
