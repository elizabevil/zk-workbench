//! 本地中继：监听临时端口，把连接转发到目标（可经代理）。

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use super::proxy::ProxyProfile;
use super::upstream;
use crate::error::AppResult;

/// 中继句柄。`Drop` 时终止监听任务，端口随之释放。
pub struct RelayHandle {
    addr: SocketAddr,
    task: JoinHandle<()>,
}

impl RelayHandle {
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn endpoint(&self) -> String {
        self.addr.to_string()
    }
}

impl Drop for RelayHandle {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// 启动中继。每次接受新连接都会重新建立上游，便于客户端重连。
pub async fn start(target: String, proxy: ProxyProfile) -> AppResult<RelayHandle> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|error| Box::new(crate::error::app::proxy_failed("relay", &error.to_string())))?;
    let addr = listener
        .local_addr()
        .map_err(|error| Box::new(crate::error::app::proxy_failed("relay", &error.to_string())))?;

    let task = tokio::spawn(async move {
        loop {
            let (mut inbound, _) = match listener.accept().await {
                Ok(pair) => pair,
                Err(_) => break,
            };
            let target = target.clone();
            let proxy = proxy.clone();
            tokio::spawn(async move {
                match upstream::connect(&target, Some(&proxy)).await {
                    Ok(mut outbound) => {
                        let _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await;
                    }
                    Err(error) => {
                        tracing::warn!(?error, "中继上游连接失败");
                    }
                }
            });
        }
    });

    Ok(RelayHandle { addr, task })
}
