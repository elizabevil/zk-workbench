---
doc_id: CHG-0055
title: 首页搜索与分组
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0055：首页搜索与分组

- 连接档案新增可选 `group` 字段（旧档案缺省为未分组）；
- 首页按分组展示，支持按名称、地址、`host:port` 与分组关键字过滤；
- 新建/编辑对话框可填写分组。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
