---
doc_id: CHG-0024
title: 代码实现结构与 IPC 契约
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0024：代码实现结构与 IPC 契约

## 决策

1. 每个方向独立目录：Rust 的 `connection/`、`tree/`、`node/`、`acl/`、`command/`、`ipc/`，前端的 `pages/`、`components/`、`api/`、`stores/` 同样按方向拆分。
2. 前端补充 `pages/`，与 `components/` 并列：页面负责组合与路由，组件只做呈现；页面私有组件放 `pages/<page>/components/`。
3. IPC 分三段：DTO 在 `zkui-core/src/ipc/`，处理器在 `src-tauri/src/commands/`，调用层在 `src/api/`；每段按方向拆文件。
4. TypeScript 契约类型由 `ts-rs` 从 Rust DTO 生成（`DEC-064`），不手写。
5. 文件拆分阈值：Rust 300 行、SFC 250 行、TS 调用层 200 行，写入 `06-ai/development-workflow.md`。

## 落地

- 新增 `02-design/implementation.md`：分层、目录、并发模型、IPC 契约表、切片顺序、测试策略、非目标与被拒方案。
- `README.md` 增加实现结构入口。
- `dec/tooling.md` 增加 `DEC-064`。
- `06-ai/development-workflow.md` 增加文件拆分阈值一节。

## 影响

- 后续代码按切片 S1–S7 推进，每个方向独立目录，避免单文件膨胀。
- `src-tauri` 保持薄壳：命令处理器只做校验与转换。
- 前端不再手写 IPC 类型，生成步骤加入代码侧 justfile。

## 非目标（Non-Goals）

1. 不在本记录中实现任何业务代码。
2. 不引入 actor、消息总线或插件系统。
3. 不定义巡检、审计、导入导出的实现。
4. 不改变已确认的 IPC 数据编码与错误模型。
5. 不覆盖 ts-rs 的具体生成命令（在 S1 落地）。

## 候选方案与权衡（Alternatives & Trade-offs）

**单一 `api/index.ts` 与单一 `commands.rs`**：初期文件少，但功能增长后必然膨胀，评审困难；被拒绝。

**前端手写 TS 类型**：与 Rust 契约容易漂移；被拒绝，改用 ts-rs。

**每连接一个 actor**：当前无背压需求且客户端保证调用顺序，先不引入。

## 遗留

- `connection_create` 的 profiles 持久化位置与凭据保存范围落地方式。
- 递归删除预览 token 的失效策略。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
