---
doc_id: IMPLEMENTATION
title: 代码实现结构与契约
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 代码实现结构与契约

本文件定义模块划分、IPC 契约、并发模型与切片顺序。行为以 `req/` 为准；技术选择见 `dec/`。

## 1. 分层

```text
Vue 前端 (src/)
   pages / components / stores / i18n / theme
        │  invoke / listen（Tauri IPC）
        ▼
src-tauri/src/            薄壳：命令处理器、事件广播、原生菜单、窗口
        │  直接调用
        ▼
crates/zkui-core/         领域核心：连接、会话、缓存、节点、ACL、命令
```

约束：

- 业务规则、ZooKeeper 协议、权限与一致性判断只在 `zkui-core`；
- `src-tauri` 的 `commands/*` 只做入参校验、DTO 转换与事件发送，不写业务分支；
- 前端不持有 ZooKeeper 协议知识，不缓存权威数据。

## 2. Rust 目录

```text
crates/zkui-core/src/
  lib.rs
  error/          mod.rs  zk.rs  app.rs
  config/         mod.rs  app_toml.rs
  connection/     mod.rs  manager.rs  handle.rs  state.rs  identity.rs
  tree/           mod.rs  cache.rs  query.rs
  node/           mod.rs  read.rs  write.rs  create.rs  delete.rs
  acl/            mod.rs  get.rs  set.rs
  command/        mod.rs  lexer.rs  parser.rs  exec.rs  complete.rs
  ipc/            mod.rs  connection.rs  tree.rs  node.rs  acl.rs  command.rs  events.rs
```

```text
src-tauri/src/
  main.rs  lib.rs
  state.rs         AppState：ConnectionManager 等共享状态
  commands/        mod.rs  connection.rs  tree.rs  node.rs  acl.rs  command.rs
  events.rs        事件广播封装
  menu.rs          原生菜单与语言同步
```

## 3. 前端目录

```text
src/
  pages/
    workbench/       主工作台；仅本页使用的组件放 pages/workbench/components/
    connections/     连接列表与新建连接
    settings/        语言、主题、日志路径
  components/        跨页面复用
    tree/  data/  inspector/  command/  common/
  api/               IPC 调用层，按方向拆分
    client.ts  connection.ts  tree.ts  node.ts  acl.ts  command.ts  events.ts
    types/           ts-rs 生成
  stores/            connection.ts  tabs.ts  editor.ts  command.ts
  i18n/  theme/  router/
```

页面负责组合与路由，组件只做呈现与局部交互；页面私有组件不下沉到顶层。

## 4. 并发模型

- `zookeeper-client` 自带重连与会话恢复，`Client: Clone + Send + Sync`，不自行实现重连循环。
- `ConnectionHandle` 持有 `Client`、`generation`（原子计数器）、状态广播通道、身份与缓存命名空间。
- 每个连接一个轻量后台任务消费 `StateWatcher`：状态变化时广播 `connection:state`，在 `Disconnected → SyncConnected` 或 `Expired` 时递增 generation。
- 不引入 actor、连接池或每操作新建客户端；只有出现真实背压或多路复用需求时再评估。

## 5. IPC 契约

### 5.1 三段职责

| 段 | 位置 | 内容 |
|---|---|---|
| DTO | `zkui-core/src/ipc/` | `serde` 结构体与错误形状，ts-rs 生成 TS 类型 |
| 处理器 | `src-tauri/src/commands/` | 入参校验、调用 core、返回 DTO |
| 调用层 | `src/api/` | `invoke` 封装、错误映射、事件订阅 |

### 5.2 命令（首版）

| 方向 | 命令 | 返回 |
|---|---|---|
| connection | `connection_create` | `{ connection_id }` |
| connection | `connection_close` | `()` |
| connection | `connection_snapshot` | `ConnectionSnapshot { state, generation, identity, session_timeout }` |
| tree | `tree_children` | `ChildrenPage { children, stat, generation }` |
| tree | `tree_locate` | `LocateResult { path, exists, generation }` |
| node | `node_get` | `NodeView { data_b64, is_text, byte_len, stat, acl, generation }` |
| node | `node_set` | `NodeStatDto` |
| node | `node_create` | `CreateResult { path, stat }` |
| node | `node_delete_preview` | `DeletePreview { paths, total, truncated, token }` |
| node | `node_delete` | `DeleteReport { done, failed, skipped }` |
| acl | `acl_get` / `acl_set` | `AclView` / `NodeStatDto` |
| command | `command_execute` | `CommandResult` |
| command | `command_complete` | `Vec<Completion>` |

命名统一 snake_case；一个方向一个文件，禁止万能 `execute(action, payload)` 通道。

### 5.3 事件

| 事件 | 载荷 |
|---|---|
| `connection.state.changed` | `{ connection_id, state, generation }` |
| `tree.node.invalidated` | `{ connection_id, paths, generation }` |
| `operation.progress.changed` | `{ request_id, done, total, message_key }` |

事件名采用 `<domain>.<entity>.<action>` 点分隔：Tauri 本身对冒号无限制，但点号在日志、SSE/WebSocket 桥接与 URL 场景不需要转义；事件名集中定义在 `zkui-core/src/ipc/events.rs` 与 `src/api/events.ts`。

长操作（递归删除预览、将来全树搜索）带 `request_id`，提供 `operation_cancel`。

### 5.4 数据与错误

- 节点数据：`data_b64` + `is_text` + `byte_len`；文本视图由前端按 `is_text` 解码，原始字节始终保留。
- 错误：`{ kind, code, scope, message_key, fields }`；`kind` 用 `zk_*` / `app_*` / `ipc_*` 前缀，`message_key` 由前端 i18n 渲染（`DEC-063`）。
- generation：依赖旧缓存状态的请求必须携带，服务端不匹配即拒绝（`REQ-CONN-GEN-001`）。

## 6. 文件拆分阈值

| 类型 | 阈值 | 处理 |
|---|---|---|
| Rust 模块文件 | 300 行或出现第二个职责 | 拆子模块，保持一个方向一个目录 |
| Vue SFC | 250 行 | 拆子组件；页面只留布局与数据编排 |
| TS 调用层 | 200 行 | 按方向拆文件，不堆到 `index.ts` |
| 测试 | 按场景分文件 | 一个关注点一个文件，两套客户端不混放 |

## 7. 垂直切片

| 切片 | 交付 | 关联语句 |
|---|---|---|
| S1 | 连接管理器、状态事件、快照；前端显示连接状态 | `REQ-CONN-STATE-001/002`、`REQ-CONN-GEN-001` |
| S2 | 代理传输：SOCKS5、HTTP CONNECT、本地中继与错误分类 | `REQ-CONN-PROXY-001..003` |
| S3 | 审计基础：SQLite schema、异步写入、哈希链、完整性校验 | `REQ-OPS-AUDIT-003..005` |
| S4 | 树按需展开、路径直达、缓存失效事件 | `REQ-UI-TREE-001..005` |
| S5 | 节点读取、Stat 与 ACL 只读展示 | `REQ-DATA-VIEW-001/002`、`REQ-AUTH-ACL-002` |
| S6 | 编辑、diff、CAS 保存、冲突提示（接入变更审计） | `REQ-DATA-EDIT-001..005` |
| S7 | 创建、删除预览与分项结果（接入变更审计） | `REQ-NODE-CREATE-*`、`REQ-NODE-DEL-*` |
| S8 | 本地指标与可观测面板；OTel 导出开关 | `REQ-OPS-OBS-001/002` |
| S9 | 命令区：词法、解析、执行、结果 | `REQ-CLI-SCOPE-006`、`REQ-CLI-FLAGS-003` |
| S10 | 脚本、历史、补全 | `REQ-CLI-SCRIPT-*`、`REQ-UI-HISTORY-001` |

每个切片完成时：`just check` 通过，相关 `ver/` 用例从 `pending` 更新为真实结果。

## 8. 测试策略

- 单元测试：`zkui-core` 模块内 `#[cfg(test)]`，覆盖错误映射、generation 判定、缓存键与失效、命令行词法/解析。
- 集成测试：`crates/zkui-core/tests/<方向>/`，连接真实 ZooKeeper；客户端对照测试已按库拆分。
- Tauri 层：命令落地后补 `src-tauri/tests/` 的命令级测试；当前只有链接测试。
- 所有需要环境的用例默认 `#[ignore]`，通过 `just --justfile project/justfile spike <endpoint>` 显式执行。

## 9. 配置模型

内嵌默认（随二进制）+ 用户覆盖（用户数据目录）。密码只进 keyring，配置文件不出现。

```toml
[general]
locale = "zh-CN"
theme = "compact"
restore_last_connection = false

[notifications]
desktop = "errors"        # off | errors | warnings | all
command_finished = false
sound = false

[retry]
auto_reconnect = true
max_attempts = 5
initial_interval_ms = 3000
multiplier = 2.0
max_interval_ms = 10000
jitter = 0.2

[task]
continue_on_error = false

[proxy]
enabled = false
no_proxy = []
# 按顺序尝试，全部失败才报错
[[proxy.profiles]]
name = "default"
kind = "socks5"           # socks5 | http_connect
host = ""
port = 1080
username = ""

[observability.otel]
enabled = false
endpoint = ""
protocol = "http/protobuf"
sample_ratio = 1.0

[audit]
retention_days = 90
```

设置页为独立路由（`pages/settings/`），提供恢复默认；恢复默认不删除凭据与审计记录。

## 10. 非目标（Non-Goals）

1. 不引入 actor 框架、消息总线或微服务拆分。
2. 不做多窗口与多进程共享会话。
3. 不实现插件系统或第三方扩展点。
4. 不定义巡检、审计与导入导出的实现细节。
5. 不做性能基准与压测方案。

## 11. 候选方案与权衡（Alternatives & Trade-offs）

**每连接一个 actor 串行化命令**：可简化并发推理，但当前客户端已保证调用顺序且无背压需求，先不引入。

**单一 `api/index.ts` / 单一 `commands.rs`**：初期省事，后期必然膨胀；按方向拆分的成本仅多几个文件。

**手写 TS 类型**：与 Rust 契约易漂移；改为 ts-rs 生成，前端类型由构建步骤产出。

## 12. 待确认

- `connection_create` 的凭据保存范围与 profiles 持久化位置（`REQ-AUTH-CRED-003/004` 的落地方式）。
- 递归删除预览的 `token` 失效策略（预览后树变化时如何重新确认）。
