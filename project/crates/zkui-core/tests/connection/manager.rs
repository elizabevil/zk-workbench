//! 连接管理器：创建、快照、generation 校验与关闭。

use zkui_core::connection::{ConnectionManager, ConnectionState, CreateConnection};
use zkui_core::ipc::connection::ConnectionSnapshot;

fn endpoint() -> String {
    std::env::var("ZK_SPIKE_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:2181".to_string())
}

fn request() -> CreateConnection {
    CreateConnection {
        endpoint: endpoint(),
        display_name: "spike".to_string(),
        identity: None,
    }
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
    request.endpoint = "  ".to_string();
    let error = manager.create(request).await.expect_err("空地址应失败");
    assert_eq!(error.kind, zkui_core::error::ErrorKind::IpcInvalidArgument);
}
