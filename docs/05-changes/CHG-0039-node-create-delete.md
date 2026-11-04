---
doc_id: CHG-0039
title: S7 创建与删除
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0039：S7 创建与删除

## 交付

| 层 | 内容 |
|---|---|
| core | `create_node`（四种类型、可选父链）、`delete_preview`、`delete_node`（递归子先删） |
| 命令 | `node_create`、`node_delete_preview`、`node_delete` |
| 前端 | 创建对话框、删除确认与影响预览、顺序节点定位实际路径、结果摘要 |

## 行为

- 创建支持持久、临时、顺序、临时顺序；顺序节点返回实际绝对路径并选中（`REQ-NODE-CREATE-006/007`）；
- 父链只有显式勾选才创建，中间节点为持久（`REQ-NODE-CREATE-004`）；
- 非空节点必须勾选「删除子树」才能确认（`REQ-NODE-DEL-002/003`）；
- 预览列出前 100 条与总数，超过 5000 显示 `≥5000`（`REQ-NODE-DEL-004`）；
- 递归删除子节点优先，遇失败停止并报告完成、失败路径与未执行数量（`REQ-NODE-DEL-006`）。

## 验证

```text
live 192.168.11.5:2181
create_set_read_delete_cycle ... ok（创建、读取、CAS、冲突、预览、递归删除）

vue-tsc / biome / just check   通过
cargo test -p zkui-core --lib  51 passed
```

## 未完成

- 预览的后端可取消（`REQ-NODE-DEL-005`）；
- 「仅重试失败子树」（`REQ-NODE-DEL-007`）；
- ACL 与节点类型展示（后续）。
