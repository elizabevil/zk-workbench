---
doc_id: CHG-0037
title: S5 节点数据查看
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0037：S5 节点数据查看

## 交付

| 层 | 内容 |
|---|---|
| DTO | `NodeData { path, data_b64, byte_len, is_text, stat }` |
| core | `ConnectionManager::data`，按原始字节返回，UTF-8 判定 `is_text` |
| 命令 | `node_data` |
| 前端 | 数据/Stat 标签页；文本原始与 JSON 格式化切换；二进制只读十六进制预览 |

## 行为

- 查看只读，格式化只改变展示，原始字节不变（`REQ-DATA-VIEW-001`）；
- 无法解码为文本时显示字节长度与十六进制预览（`REQ-DATA-BYTES-001`）；
- Stat 标签页显示版本、子节点数、数据长度、临时节点与修改时间。

## 验证

```text
vue-tsc / biome / just check   通过
cargo test -p zkui-core --lib  51 passed
```

## 未完成

- 语法高亮、YAML 与折叠（`REQ-DATA-VIEW-002` 部分）；
- 大节点按需加载（`REQ-DATA-LARGE-001`）；
- 编辑、创建、删除在后续提交实现。
