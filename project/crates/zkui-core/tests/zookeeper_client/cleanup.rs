//! 清理用：删除根下 `zkc-` 前缀的遗留节点。

use zookeeper_client::Error;

use super::common::{connect, delete_recursive};

#[tokio::test]
#[ignore = "清理用"]
async fn cleanup_leftovers() {
    let client = connect().await;
    let (children, _) = client.get_children("/").await.expect("读取根节点失败");
    let leftovers: Vec<String> = children
        .into_iter()
        .filter(|c| c.starts_with("zkc-"))
        .collect();
    println!("发现遗留节点 {} 个: {leftovers:?}", leftovers.len());
    for child in &leftovers {
        delete_recursive(&client, &format!("/{child}")).await;
    }
    let (remaining, _) = client.get_children("/").await.expect("读取根节点失败");
    let still: Vec<&String> = remaining.iter().filter(|c| c.starts_with("zkc-")).collect();
    assert!(still.is_empty(), "仍有遗留节点: {still:?}");
    let _ = Error::NoNode;
}
