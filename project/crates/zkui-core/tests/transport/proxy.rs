//! 经真实代理建立到 ZooKeeper 的连接。

use zkui_core::transport::{ProxyKind, ProxyProfile, relay};
use zookeeper_client::Client;

fn zk_endpoint() -> String {
    std::env::var("ZK_SPIKE_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:2181".to_string())
}

fn proxy_endpoint() -> String {
    std::env::var("ZK_PROXY_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:1080".to_string())
}

fn profile(kind: ProxyKind) -> ProxyProfile {
    let endpoint = proxy_endpoint();
    let (host, port) = endpoint
        .rsplit_once(':')
        .map(|(host, port)| (host.to_string(), port.parse().expect("代理端口非法")))
        .expect("代理地址必须是 host:port");
    ProxyProfile {
        name: format!("{kind:?}"),
        kind,
        host,
        port,
        username: None,
        password: None,
    }
}

async fn assert_connect_through(kind: ProxyKind) {
    let relay = relay::start(zk_endpoint(), profile(kind))
        .await
        .expect("启动中继失败");
    let client = Client::connect(&relay.endpoint())
        .await
        .expect("经代理连接失败");
    let (_data, stat) = client.get_data("/").await.expect("读取根节点失败");
    assert!(stat.version >= 0);
    drop(client);
    drop(relay);
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 与代理环境"]
async fn socks5_proxy_reaches_zookeeper() {
    assert_connect_through(ProxyKind::Socks5).await;
}

#[tokio::test]
#[ignore = "需要 ZooKeeper 与代理环境"]
async fn http_connect_proxy_reaches_zookeeper() {
    assert_connect_through(ProxyKind::HttpConnect).await;
}
