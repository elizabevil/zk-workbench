---
doc_id: CHG-0048
title: JSON/YAML 格式化与语法高亮
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0048：JSON/YAML 格式化与语法高亮

- 数据视图提供「原始 / JSON / YAML」格式切换；JSON 可解析时提供 JSON 视图，YAML 可解析时提供 YAML 视图；
- 使用 highlight.js 对 JSON/YAML 只读渲染做语法高亮；
- YAML 格式化使用 `yaml` 包的 Document API（YAML 1.2，保留注释），且仅在解析结果为映射/序列时提供，避免把标量文本误当 YAML；
- 格式化只影响展示，不修改原始字节，也不触发写入（`REQ-DATA-VIEW-001`）。

## 未完成

- 折叠/展开与完整 YAML 高亮主题定制（`REQ-DATA-VIEW-002` 的折叠部分）。

## 验证

```text
vue-tsc / biome / just check   通过
```
