---
doc_id: CHG-0061
title: 审计事件模型与 schema v2
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0061：审计事件模型与 schema v2

## 事件模型

```text
AuditEvent
├── op          set / create / delete / set_acl / clear（枚举）
├── connection_id
├── generation  记录写入时的连接 generation（0 = 未知）
├── path
├── before_version / after_version（-1 = 不适用）
├── result      success / failure（枚举）
├── error_kind  scope.kind，成功时为空串
├── detail
└── actor       scheme:display，未认证为空串
```

`ts` 与 `id` 在写入时生成，`prev_hash`/`hash` 由完整事件字段计算。

## 字段语义

- `before_version` 是执行写入前用户读到的版本（CAS 基准）；`create`/`delete`/`set_acl` 暂无版本语义时为 `-1`；
- `after_version` 是写入成功后服务端返回的新版本；未知时为 `-1`；
- 空字符串用于表示“无值”，数据库列全部 `NOT NULL`，避免前端做 null 判断；
- `actor` 只保存展示身份，不保存凭据。

## 表结构与索引

- `PRAGMA user_version = 2`，启动时执行迁移；
- v1 → v2 迁移使用 `COALESCE` 填充旧数据的 null，并在 Rust 侧重算哈希链；
- 索引：`ts`、`path`、`connection_id`、`op`。

## 覆盖范围

- 工作区写操作：`set`、`create`、`delete`；
- 命令区写操作：`set`、`create`、`delete`、`deleteall`、`setAcl`（成功与失败都记录）；
- 读取不记录；`clear` 自身写入一条 `clear` 标记。

## 未完成

- 异步写队列（当前同步 WAL 写入）；
- 自动裁剪任务；
- 节点历史按路径过滤的 UI 已在检查器时间线中提供基础版本。

## 验证

```text
cargo test -p zkui-core --lib audit   9 passed
just check                            exit 0
```
