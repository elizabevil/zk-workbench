---
doc_id: CHG-0035
title: 连接档案本地持久化
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0035：连接档案本地持久化

## 变更

| 层 | 内容 |
|---|---|
| 存储 | `<config_dir>/zkui/connections.toml`；原子写入；`id` 递增 |
| 命令 | `profile_list`、`profile_save`（新建/更新）、`profile_delete` |
| 前端 | 首页读取档案；新建连接时先保存档案再建立会话；删除需确认 |
| 状态 | 已连接档案显示「已连接」并提供「打开」，未连接显示「连接」 |

## 行为

- 重启后首页仍有档案，可再次连接；
- 档案只保存名称、主机、端口，不包含凭据；
- 同一 `host:port` 已有活动会话时，点击「打开」复用会话而不是新建。

## 验证

```text
cargo test -p zkui-core --lib   51 passed（含档案增删改与校验）
vue-tsc                         通过
just check                      exit 0
```

## 未完成

1. 编辑档案（当前只有新建与删除）；
2. 凭据、代理选择随档案保存；
3. 档案排序、分组与搜索。
