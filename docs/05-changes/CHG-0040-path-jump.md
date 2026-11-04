---
doc_id: CHG-0040
title: 工作区路径直达
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0040：工作区路径直达

## 变更

树顶部增加路径输入与「跳转」：输入绝对或相对路径后直接读取目标节点，不要求逐级展开（`REQ-UI-TREE-004`）。路径自动补前导 `/`
、去掉尾部 `/`，根路径保持 `/`。

## 验证

```text
vue-tsc / biome / just check   通过
```
