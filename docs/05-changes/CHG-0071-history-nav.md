---
doc_id: CHG-0071
title: 最近节点移入右侧历史，修复审计/可观测路由
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0071

- 移除数据区上方的横向节点标签；
- 右侧检查器新增“最近节点”列表（按打开顺序倒序），点击直接查看、可关闭；
- 命令历史与时间线仍保留在同一检查器；
- 修复 `/audit` 与 `/observability` 未注册路由的问题，两个页面现在可访问。

## 验证

```text
just check  exit 0
vue-tsc/biome 通过
```
