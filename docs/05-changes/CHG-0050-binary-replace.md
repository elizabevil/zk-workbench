---
doc_id: CHG-0050
title: 二进制数据完整替换
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0050：二进制数据完整替换

- 非文本节点提供「替换数据」，通过文件或十六进制输入完整替换；
- 保存前展示字节数变化、长度差与差异字节数，并使用读取版本做 CAS；
- 不提供逐字节内联编辑（`REQ-DATA-BYTES-002`）；
- 大节点截断时先加载完整内容再允许替换，避免基于前缀覆盖。

## 验证

```text
vue-tsc / biome / just check   通过
```
