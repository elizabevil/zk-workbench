# 选型验证测试（Spike）

本目录用于 ZooKeeper 客户端的对照验证，不依赖 Tauri。

## 计划

| 测试文件 | 候选 | 覆盖 |
|---|---|---|
| `spike_client_a.rs` | 首选候选 | 连接、节点全类型、CAS、ACL、认证、Watcher、会话过期、递归删除部分失败 |
| `spike_client_b.rs` | 对照候选 | 同一矩阵，便于逐项比对 |

## 运行方式

测试默认 `#[ignore]`，避免在没有环境时误报。先启动场景，再显式运行：

```bash
just --justfile test/env/justfile up zk-39-single
ZK_SPIKE_ENDPOINT=127.0.0.1:2181 cargo test -p zkui-core --test spike_client_a -- --ignored --nocapture
just --justfile test/env/justfile down zk-39-single
```

3.4 / 3.5 场景同理替换 `ZK_SPIKE_ENDPOINT` 与场景名。结果记录到 `docs/ver/conn.md` 与对应 CHG。

## 约定

- 不在测试里写死镜像 tag 之外的假设。
- 每项能力失败时记录错误类别与原始错误，不吞错。
