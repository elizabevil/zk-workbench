---
doc_id: DEC-IPC
title: IPC 契约与文档同步策略
status: accepted
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# IPC 契约与文档同步策略

DEC-050   status: accepted   updated: 2026-11-02
  选择：IPC 的 command/event、payload 与错误类型以 Rust 类型定义为唯一来源，契约写在 Rust doc comment 中；文档由 `cargo doc` 生成，不另行维护独立的 schema 文档。前端类型由同一份 Rust 类型生成（候选 ts-rs/specta，Spike 后定），禁止手写两套。
  依据：负责人要求文档与代码同步、代码注释即文档；避免 schema 文档与实现漂移。
  推翻条件：生成工具无法表达前端需要的类型（如联合类型、品牌类型）。
  影响：`DESIGN-002` 的 IPC 契约改为以 Rust 类型定义为准；新增命令必须同时写 doc comment 与生成前端类型。

DEC-051   status: proposed   updated: 2026-11-02
  选择：错误分类以稳定错误码 + 结构化字段表达（kind、path、version、连接、generation），人类可读文案不作为机器协议。
  依据：`REQ-DATA-EDIT-002`、`REQ-NODE-CREATE-007` 等要求错误可分类；`code-engineering.md` 要求错误码稳定。
  推翻条件：前端需要按文案分支（不应发生）。
  影响：待 IPC 类型设计时落地。
