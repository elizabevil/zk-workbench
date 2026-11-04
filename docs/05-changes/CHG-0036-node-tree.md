---
doc_id: CHG-0036
title: 工作区节点树与 Stat 面板
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0036：工作区节点树与 Stat 面板

## 变更

| 层   | 内容                                                                                                                     |
|------|--------------------------------------------------------------------------------------------------------------------------|
| core | `ConnectionManager::children`（直接子节点、排序）、`stat`（Stat 子集）                                                   |
| DTO  | `NodeStat { path, version, cversion, aversion, data_length, num_children, ephemeral_owner, czxid, mzxid, ctime, mtime }` |
| 命令 | `node_children`、`node_stat`                                                                                             |
| 前端 | 可展开树（按需加载、子节点缓存）、路径选中、Stat 面板、刷新                                                              |

## 行为

- 展开节点只读取直接子节点，未展开分支不加载（`REQ-UI-TREE-001`）；
- 同一节点展开后复用子节点结果，直到刷新或 generation 变化（`REQ-UI-TREE-002`）；
- 连接切换或 generation 变化时树重新挂载，缓存失效（`REQ-UI-TREE-003`）；
- 根节点默认展开并选中，Stat 面板显示版本、子节点数、数据长度、临时节点与修改时间。

## 验证

```text
live：192.168.11.5:2181
children_and_stat_read_root ... ok（root children: 2, stat.version: 0）
cargo test -p zkui-core --lib   51 passed
vue-tsc / biome / just check    通过
```

## 未完成

1. 路径直达（`REQ-UI-TREE-004`）与搜索；
2. 节点数据的查看与编辑（S5/S6）；
3. 创建、删除与 ACL（S6/S7）；
4. 树节点的子节点事件推送与局部失效（当前整树刷新）。
