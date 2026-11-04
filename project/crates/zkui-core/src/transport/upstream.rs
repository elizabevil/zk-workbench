//! 建立到目标的上游连接：直连、SOCKS5 或 HTTP CONNECT。
//!
//! HTTP CONNECT 使用 `async-http-proxy`，不自行解析协议。

use async_http_proxy::{http_connect_tokio, http_connect_tokio_with_basic_auth};
use tokio::net::TcpStream;

use super::proxy::{ProxyKind, ProxyProfile};
use crate::error::{AppResult, app};

/// 连接目标；`proxy` 为 None 时直连。
pub async fn connect(target: &str, proxy: Option<&ProxyProfile>) -> AppResult<TcpStream> {
    match proxy {
        None => direct(target).await,
        Some(profile) => match profile.kind {
            ProxyKind::Socks5 => socks5(target, profile).await,
            ProxyKind::HttpConnect => http_connect(target, profile).await,
        },
    }
}

async fn direct(target: &str) -> AppResult<TcpStream> {
    TcpStream::connect(target)
        .await
        .map_err(|error| Box::new(proxy_error("connect", &error.to_string())))
}

async fn socks5(target: &str, profile: &ProxyProfile) -> AppResult<TcpStream> {
    use tokio_socks::tcp::Socks5Stream;

    let proxy_addr = profile.endpoint();
    let stream = match (&profile.username, &profile.password) {
        (Some(username), Some(password)) => {
            Socks5Stream::connect_with_password(proxy_addr.as_str(), target, username, password)
                .await
                .map(Socks5Stream::into_inner)
        }
        _ => Socks5Stream::connect(proxy_addr.as_str(), target)
            .await
            .map(Socks5Stream::into_inner),
    };
    stream.map_err(|error| Box::new(proxy_error("socks5", &error.to_string())))
}

async fn http_connect(target: &str, profile: &ProxyProfile) -> AppResult<TcpStream> {
    let (host, port) = split_target(target)?;
    let mut stream = TcpStream::connect(profile.endpoint())
        .await
        .map_err(|error| Box::new(proxy_error("connect", &error.to_string())))?;

    let result = match (&profile.username, &profile.password) {
        (Some(username), Some(password)) => {
            http_connect_tokio_with_basic_auth(&mut stream, &host, port, username, password).await
        }
        _ => http_connect_tokio(&mut stream, &host, port).await,
    };
    result.map_err(|error| Box::new(proxy_error("handshake", &error.to_string())))?;
    Ok(stream)
}

/// 拆分 `host:port`，端口缺失时按 2181 处理。
fn split_target(target: &str) -> AppResult<(String, u16)> {
    match target.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() => port
            .parse::<u16>()
            .map(|port| (host.to_string(), port))
            .map_err(|_| Box::new(app::invalid_argument("invalid target port"))),
        Some(_) => Err(Box::new(app::invalid_argument("invalid target address"))),
        None => Ok((target.to_string(), 2181)),
    }
}

fn proxy_error(stage: &str, detail: &str) -> crate::error::AppError {
    app::proxy_failed(stage, detail)
}
