//! 连接管理器：创建、快照、generation 校验与关闭。

use zkui_core::connection::{ConnectionManager, ConnectionState, CreateConnection};
use zkui_core::ipc::connection::ConnectionSnapshot;

fn endpoint() -> String {
    std::env::var("ZK_SPIKE_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:2181".to_string())
}

fn request() -> CreateConnection {
    let endpoint = endpoint();
    let (host, port) = endpoint
        .rsplit_once(':')
        .expect("endpoint must be host:port");
    CreateConnection {
        host: host.to_string(),
        port: port.parse().expect("port must be numeric"),
        display_name: "spike".to_string(),
        identity: None,
        proxy: None,
    }
}

fn proxy_profile() -> Option<zkui_core::transport::ProxyProfile> {
    let endpoint = std::env::var("ZK_PROXY_ENDPOINT").ok()?;
    let (host, port) = endpoint.rsplit_once(':')?;
    Some(zkui_core::transport::ProxyProfile {
        name: "env".to_string(),
        kind: zkui_core::transport::ProxyKind::Socks5,
        host: host.to_string(),
        port: port.parse().ok()?,
        username: None,
        password: None,
    })
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn create_snapshot_and_close() {
    let manager = ConnectionManager::new();
    let snapshot: ConnectionSnapshot = manager.create(request()).await.expect("创建连接失败");

    assert_eq!(snapshot.state, ConnectionState::Connected);
    assert_eq!(snapshot.generation, 0);
    assert!(snapshot.session_timeout_ms > 0);

    let again = manager
        .snapshot(&snapshot.connection_id)
        .expect("读取快照失败");
    assert_eq!(again.connection_id, snapshot.connection_id);
    assert_eq!(again.generation, 0);

    manager
        .ensure_generation(&snapshot.connection_id, 0)
        .expect("generation 应匹配");
    assert!(
        manager
            .ensure_generation(&snapshot.connection_id, 99)
            .is_err()
    );

    manager.close(&snapshot.connection_id).expect("关闭失败");
    assert!(manager.snapshot(&snapshot.connection_id).is_err());
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn empty_endpoint_is_rejected() {
    let manager = ConnectionManager::new();
    let mut request = request();
    request.host = "  ".to_string();
    let error = manager.create(request).await.expect_err("空地址应失败");
    assert_eq!(error.kind, zkui_core::error::ErrorKind::InvalidArgument);
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 与代理环境"]
async fn create_connection_through_socks5() {
    let Some(proxy) = proxy_profile() else {
        eprintln!("未设置 ZK_PROXY_ENDPOINT，跳过");
        return;
    };
    let manager = ConnectionManager::new();
    let mut request = request();
    request.proxy = Some(proxy);
    let snapshot = manager.create(request).await.expect("经代理创建连接失败");
    assert_eq!(snapshot.state, ConnectionState::Connected);
    manager.close(&snapshot.connection_id).expect("关闭失败");
}

fn settings_store(
    settings: zkui_core::config::Settings,
) -> (
    std::sync::Arc<zkui_core::config::SettingsStore>,
    std::path::PathBuf,
) {
    let dir = std::env::temp_dir().join(format!("zkui-test-{}", std::process::id()));
    let path = dir.join("app.toml");
    let store = zkui_core::config::SettingsStore::with_path(path.clone());
    store.save(settings).expect("保存测试设置失败");
    (std::sync::Arc::new(store), dir)
}

fn proxy_at(
    endpoint: &str,
    kind: zkui_core::transport::ProxyKind,
) -> zkui_core::transport::ProxyProfile {
    let (host, port) = endpoint.rsplit_once(':').expect("代理地址必须是 host:port");
    zkui_core::transport::ProxyProfile {
        name: format!("{kind:?}"),
        kind,
        host: host.to_string(),
        port: port.parse().expect("代理端口非法"),
        username: None,
        password: None,
    }
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 与代理环境"]
async fn proxy_profiles_fallback_in_order() {
    let Some(proxy) = std::env::var("ZK_PROXY_ENDPOINT").ok() else {
        eprintln!("未设置 ZK_PROXY_ENDPOINT，跳过");
        return;
    };
    let mut settings = zkui_core::config::Settings::default();
    settings.proxy.enabled = true;
    settings.proxy.profiles = vec![
        proxy_at("127.0.0.1:1", zkui_core::transport::ProxyKind::Socks5),
        proxy_at(&proxy, zkui_core::transport::ProxyKind::Socks5),
    ];
    let (store, dir) = settings_store(settings);
    let manager = ConnectionManager::with_settings(Some(store));
    let snapshot = manager
        .create(request())
        .await
        .expect("应按顺序回退到第二条代理");
    assert_eq!(snapshot.state, ConnectionState::Connected);
    manager.close(&snapshot.connection_id).expect("关闭失败");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn no_proxy_bypasses_configured_proxy() {
    let mut settings = zkui_core::config::Settings::default();
    settings.proxy.enabled = true;
    settings.proxy.profiles = vec![proxy_at(
        "127.0.0.1:1",
        zkui_core::transport::ProxyKind::Socks5,
    )];
    let host = endpoint()
        .rsplit_once(':')
        .map(|(host, _)| host.to_string())
        .unwrap();
    settings.proxy.no_proxy = vec![host];
    let (store, dir) = settings_store(settings);
    let manager = ConnectionManager::with_settings(Some(store));
    let snapshot = manager
        .create(request())
        .await
        .expect("no_proxy 应绕过代理直连");
    assert_eq!(snapshot.state, ConnectionState::Connected);
    manager.close(&snapshot.connection_id).expect("关闭失败");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn children_and_stat_read_root() {
    let manager = ConnectionManager::new();
    let snapshot = manager.create(request()).await.expect("连接失败");
    let children = manager
        .children(&snapshot.connection_id, "/")
        .await
        .expect("读取子节点失败");
    let stat = manager
        .stat(&snapshot.connection_id, "/")
        .await
        .expect("读取 Stat 失败");
    assert_eq!(stat.path, "/");
    assert!(stat.num_children >= 0);
    assert!(stat.version >= 0);
    println!(
        "root children: {}, stat.version: {}",
        children.len(),
        stat.version
    );
    manager.close(&snapshot.connection_id).expect("关闭失败");
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 环境"]
async fn create_set_read_delete_cycle() {
    use zkui_core::error::ErrorKind;
    use zkui_core::ipc::node::NodeCreateMode;

    let manager = ConnectionManager::new();
    let snapshot = manager.create(request()).await.expect("连接失败");
    let id = snapshot.connection_id.clone();
    let base = format!("/zkui-it-{}", std::process::id());
    let leaf = format!("{base}/a");

    manager
        .create_node(
            &id,
            &leaf,
            b"one".to_vec(),
            NodeCreateMode::Persistent,
            true,
        )
        .await
        .expect("创建失败");

    let data = manager.data(&id, &leaf).await.expect("读取数据失败");
    assert!(data.is_text);
    assert_eq!(data.byte_len, 3);

    let updated = manager
        .set_data(&id, &leaf, b"two".to_vec(), data.stat.version)
        .await
        .expect("写入失败");
    assert_eq!(updated.version, data.stat.version + 1);

    let conflict = manager
        .set_data(&id, &leaf, b"x".to_vec(), data.stat.version)
        .await
        .expect_err("旧版本应冲突");
    assert_eq!(conflict.kind, ErrorKind::BadVersion);

    let preview = manager
        .delete_preview(&id, &base, true)
        .await
        .expect("预览失败");
    assert!(preview.total >= 2);

    let outcome = manager
        .delete_node(&id, &base, true)
        .await
        .expect("删除失败");
    assert!(outcome.deleted >= 2);
    assert!(outcome.failed_path.is_none());

    manager.close(&id).expect("关闭失败");
}
