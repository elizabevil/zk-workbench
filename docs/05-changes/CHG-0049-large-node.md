---
doc_id: CHG-0049
title: 大节点按需加载
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0049：大节点按需加载

- `node_data` 支持 `max_bytes`，返回预览前缀与 `truncated` 标记，`byte_len` 始终为完整长度；
- 工作区默认只请求 256 KiB 预览，显示截断提示与「加载完整内容」；
- 截断状态下禁用内联编辑，避免基于不完整内容保存；
- 记录读取延迟指标不变。

## 验证

```text
vue-tsc / biome / just check   通过
```
