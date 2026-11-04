---
doc_id: CHG-0025
title: S1 连接管理器与前端连接页
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0025：S1 连接管理器与前端连接页

## 交付

| 层 | 文件 |
|---|---|
| core 错误 | `error/mod.rs`、`error/zk.rs`、`error/app.rs` |
| core 连接 | `connection/{mod,state,identity,handle,manager}.rs` |
| core 契约 | `ipc/{mod,connection,events}.rs`，ts-rs 生成到 `src/api/types/` |
| Tauri | `state.rs`、`commands/{mod,connection}.rs`、`events.rs`、`lib.rs` |
| 前端 | `api/{client,connection,events}.ts`、`stores/connection.ts`、`pages/workbench/index.vue`、`i18n/*`、`router/index.ts`、`main.ts`、`App.vue` |

## 行为

- `connection_create` 建立连接并返回快照（状态、generation、身份、会话超时）；
- 后台任务消费 `StateWatcher`，状态变化时递增 generation（重连）或广播新状态；
- Tauri 转发为 `connection.state.changed` 事件；
- 前端工作台可连接、显示状态与 generation、关闭连接；错误按 `message_key` 渲染中英文。

## 事件命名

采用 `<domain>.<entity>.<action>`（点分隔），替代最初的 `connection:state`。原因：Tauri 对冒号无限制，但点号在日志、SSE/WebSocket 桥接、URL 场景无需转义；事件名集中定义，便于后续追加新域。

## 验证

```text
cargo test -p zkui-core --lib                         15 passed
cargo test -p zkui-core --test connection -- --ignored  2 passed（192.168.11.5:2181）
vue-tsc --noEmit                                      通过
biome check src                                       0 error，4 warning
```

`just check` 待补：把 `types` 生成步骤纳入门禁（当前手动执行）。

## 未完成

- profiles 持久化（按计划延后）；
- 树、节点、命令区；
- Biome 4 条警告（`vite-env.d.ts` 的 any/bannedTypes、`App.vue` scoped 规则、页面多词组件名），实现真实页面后清理；
- 断线重连与 generation 递增只有单元测试与真实连接快照，尚未做断网注入。

## 非目标（Non-Goals）

1. 不实现树、节点编辑、命令区。
2. 不落盘连接配置与凭据。
3. 不引入 actor 或连接池。
4. 不做多窗口事件路由。
5. 不覆盖巡检与审计。

## 候选方案与权衡（Alternatives & Trade-offs）

**事件名用冒号**：直观，但跨传输层需转义；改为点号。

**前端直接调用 `invoke` 字符串**：类型分散；改为 `api/` 封装 + ts-rs 类型。

**在 Tauri 层做状态机**：业务会分散到壳层；保持 core 单一职责。

## 遗留

- 生成类型与 Rust 不一致时如何阻断：建议在 CI 加 `just types && git diff --exit-code`。
