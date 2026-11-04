---
doc_id: CHG-0028
title: S2a 代理传输层与真实代理验证
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0028：S2a 代理传输层与真实代理验证

## 交付

| 模块                    | 内容                                                      |
|-------------------------|-----------------------------------------------------------|
| `transport/proxy.rs`    | `ProxyKind`（socks5/http_connect）、`ProxyProfile`        |
| `transport/no_proxy.rs` | 精确主机、`.suffix`、`*.suffix`、`*` 匹配，大小写不敏感   |
| `transport/upstream.rs` | 直连、SOCKS5（含认证）、HTTP CONNECT（含 Basic 认证）     |
| `transport/relay.rs`    | `127.0.0.1:0` 本地中继，每次接受连接重建上游，`Drop` 释放 |
| `error`                 | 新增 `proxy_connect`、`proxy_handshake` 类别与文案 key    |
| `connection/manager.rs` | 连接可选代理；失败按 3s/6s/10s/10s 重试，最多 5 次        |
| `ipc/connection.rs`     | `CreateConnection.proxy` 字段，ts-rs 生成前端类型         |

## 真实环境验证

- ZooKeeper：`192.168.11.5:2181`（3.9.5）
- 代理：`192.168.10.1:10808`（SOCKS5 与 HTTP CONNECT 均可用）

```text
transport::proxy::socks5_proxy_reaches_zookeeper ... ok
transport::proxy::http_connect_proxy_reaches_zookeeper ... ok
connection::manager::create_connection_through_socks5 ... ok
connection::manager::create_snapshot_and_close ... ok
connection::manager::empty_endpoint_is_rejected ... ok
```

复现：

```bash
cd project
CARGO_HOME=$PWD/.cargo-home \
ZK_SPIKE_ENDPOINT=192.168.11.5:2181 \
ZK_PROXY_ENDPOINT=192.168.10.1:10808 \
cargo test -p zkui-core --test transport --test connection -- --ignored --test-threads=1
```

## 关键行为

- 客户端始终连接本地中继，代理协议对 `zookeeper-client` 透明；
- 中继绑定临时端口，连接关闭或句柄释放即中止；
- 代理失败返回 `proxy_connect` / `proxy_handshake`，`fields.detail` 含阶段，不误报为集群故障；
- 重试策略固定为 5 次、3s/6s/10s/10s； **尚不可从设置修改**。

## 未完成

- 设置页 S2b：代理档案有序回退（当前每个连接单条代理）、凭据从 keyring 读取、`no_proxy` 接线、重试参数可配置；
- 代理健康检查与「测试连接」按钮；
- HTTP CONNECT 认证的真实代理验证（当前代理无认证）。

## 非目标（Non-Goals）

1. 不实现 SSH Tunnel。
2. 不做代理测速或自动选优。
3. 不改变 ZooKeeper 操作语义。
4. 不把代理错误合并进 `zk_*` 类别。

## 验证

`just check` 退出码 0；上述 5 条集成用例通过；单元测试 22 条通过。
