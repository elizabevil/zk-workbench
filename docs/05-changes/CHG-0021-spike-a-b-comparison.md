---
doc_id: CHG-0021
title: 两个 Rust ZooKeeper 客户端功能对照结果
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0021：两个 Rust ZooKeeper 客户端功能对照结果

## 环境

| 项       | 值                                                              |
|----------|-----------------------------------------------------------------|
| 服务端   | 192.168.11.5:2181，ZooKeeper 3.9.5（follower）                  |
| 候选 A   | `zookeeper-client` 0.11.2（`tokio` feature）                    |
| 候选 B   | `zookeeper-async` 5.0.0                                         |
| 测试文件 | `crates/zkui-core/tests/spike_client_a.rs`、`spike_client_b.rs` |

## 命令

```bash
cd project
CARGO_HOME=$PWD/.cargo-home ZK_SPIKE_ENDPOINT=192.168.11.5:2181 \
cargo test -p zkui-core -- --ignored --test-threads=1
```

## 结果

| 客户端             | 用例数 | 结果      |
|--------------------|--------|-----------|
| `zookeeper-client` | 13     | 13 passed |
| `zookeeper-async`  | 5      | 5 passed  |

覆盖：连接、持久节点 CRUD、CAS 与 `BadVersion`、子节点与 Stat、ACL 读取、认证与 ACL 拒绝（`NoAuth`）、一次性数据/子节点
Watcher、顺序后缀、`sync`、`check_stat`、非空删除 `NotEmpty`、重复创建 `NodeExists`、父路径缺失 `NoNode`、8 路并发 CAS
单赢家、递归清理。执行后服务端节点数回到 78，无 `spike-` / `spikeb-` 残留。

## API 形态差异

| 维度              | zookeeper-client 0.11                                                | zookeeper-async 5.0                                                  |
|-------------------|----------------------------------------------------------------------|----------------------------------------------------------------------|
| 连接              | `Client::connect(&str)`，可选 `Connector`                            | `ZooKeeper::connect(&str, Duration, watcher)`，连接必须提供 watcher  |
| 创建              | `create(path, &[u8], &CreateOptions)`，返回 `(Stat, CreateSequence)` | `create(path, Vec<u8>, Vec<Acl>, CreateMode)`，返回实际路径 `String` |
| 读数据            | `get_data(path)`                                                     | `get_data(path, watch: bool)`                                        |
| 一次性 watcher    | 返回值携带 `OneshotWatcher`                                          | `get_data_w(path, FnOnce(WatchedEvent))` 回调                        |
| 事件路径          | `WatchedEvent.path: String`                                          | `WatchedEvent.path: Option<String>`                                  |
| 会话状态          | `StateWatcher` + `SessionState`                                      | 连接 watcher 回调                                                    |
| 多操作            | `MultiReader` / `MultiWriter` / `CheckWriter`                        | 未在本次验证                                                         |
| 持久/递归 watcher | `PersistentWatcher` + `AddWatchMode`                                 | 未在本次验证                                                         |
| chroot            | `Client::chroot`                                                     | 未在本次验证                                                         |

## 初步结论（待负责人确认）

`zookeeper-client` 的功能面更贴近产品需求：会话状态 watcher、一次性 watcher 作为返回值（便于和缓存失效串起来）、持久
watcher、multi/check writer、chroot；`zookeeper-async` API 更传统（连接即 watcher 回调），代码量与维护节奏需另行评估。

建议： **首选 `zookeeper-client`，`zookeeper-async` 作为备选**；按规则选型属负责人确认事项，`DEC-002` 保持 `proposed` 直到确认。

## 未覆盖

- 3.4/3.5 兼容（按负责人决定本次不做）；
- 断线重连与会话过期（需要可停服务的环境）；
- 大节点（接近 1 MiB）读写与错误；
- 持久 watcher、multi writer 的实际行为；
- Tauri 集成与并发压力。

## 非目标（Non-Goals）

1. 不据此批准生产依赖。
2. 不评估许可与安全公告。
3. 不做性能基准对比。
4. 不覆盖服务端版本矩阵。
5. 不修改 `DEC-002` 状态。

## 验证

两套用例均已在 CI 之外的本机执行并全部通过；测试文件与锁文件已提交，命令见上。
