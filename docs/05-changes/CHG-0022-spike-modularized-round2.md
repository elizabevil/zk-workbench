---
doc_id: CHG-0022
title: 客户端对照测试模块化与第二轮执行
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0022：客户端对照测试模块化与第二轮执行

## 目的

按库拆分测试文件，避免两个客户端混在同一文件里；补齐大节点、持久 watcher、Multi/Check writer 与 Tauri 链接验证。

## 结构

```text
crates/zkui-core/tests/
  zookeeper_client.rs        入口，按关注点挂载模块
  zookeeper_client/
    common.rs crud.rs acl.rs watch.rs errors.rs concurrency.rs multi.rs limits.rs cleanup.rs
  zookeeper_async.rs         入口
  zookeeper_async/
    common.rs crud.rs acl.rs watch.rs errors.rs limits.rs
src-tauri/tests/
  core_link.rs               验证 app crate 能依赖并使用 zkui-core
```

## 执行结果（192.168.11.5:2181，ZooKeeper 3.9.5）

| 套件 | 用例 | 结果 |
|---|---|---|
| `zookeeper_client` | 16 | 16 passed |
| `zookeeper_async` | 6 | 6 passed |

命令：

```bash
cd project
CARGO_HOME=$PWD/.cargo-home ZK_SPIKE_ENDPOINT=192.168.11.5:2181 \
cargo test -p zkui-core -- --ignored --test-threads=1
```

新增覆盖：持久 watcher 连续两次事件、`MultiWriter` 原子创建两个节点、`CheckWriter` 版本过期被拒绝、512 KiB 读写成功、1 MiB + 4 KiB 被拒绝、清理用例确认无残留。

## 关键发现：超大写入表现为 ConnectionLoss

两个客户端写入 `1 MiB + 4 KiB` 时都返回 `ConnectionLoss`，而不是参数错误或长度错误：

```text
1 MiB + 4 KiB 写入结果: Err(ConnectionLoss)
```

含义：服务端在超过 jute 缓冲上限时直接断开连接，客户端只能看到连接丢失。产品侧必须：

1. 在写入前按配置上限做客户端预检，避免把超限数据发出去；
2. 把这类断连与会话过期、网络中断区分开，避免误报为连接故障；
3. 重连后不影响其他标签与缓存（`REQ-CONN-GEN-001/002`）。

已作为风险记入 `REQ-DATA-LARGE-002` 的验证关注点。

## Tauri 链接测试

`src-tauri` 增加对 `zkui-core` 的路径依赖，`src-tauri/tests/core_link.rs` 断言版本可用。本机缺少 `pkg-config`/`glib-2.0` 开发包，`cargo test --workspace` 无法编译 Tauri 依赖；在具备 Tauri 开发环境的机器上运行：

```bash
cargo test -p zkui --test core_link
```

## 未覆盖

- 断线重连与会话过期（需可停服务的环境）；
- 持久 watcher 在重连后的续订行为；
- Tauri command/event 的真实 IPC（命令尚未实现）。

## 非目标（Non-Goals）

1. 不据此批准生产依赖或修改 `DEC-002`。
2. 不做 3.4/3.5 兼容（负责人已决定跳过）。
3. 不做吞吐与延迟基准。
4. 不验证 TLS/SASL。

## 验证

命令与输出见上；执行后服务端节点数 78，无 `zkc-`/`zka-` 残留。
