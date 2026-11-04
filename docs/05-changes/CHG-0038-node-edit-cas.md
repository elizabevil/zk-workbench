---
doc_id: CHG-0038
title: S6 编辑保存与 CAS
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0038：S6 编辑保存与 CAS

## 交付

| 层 | 内容 |
|---|---|
| core | `ConnectionManager::set_data`，使用读取时 version 做 CAS |
| 命令 | `node_set`（base64 数据 + expected_version） |
| 前端 | 独立编辑缓冲、未保存状态、行级 diff 预览、确认保存、冲突提示与重新加载 |

## 行为

- 只有用户确认保存才写入（`REQ-DATA-EDIT-001`）；
- 保存前展示 diff，使用读取 version（`REQ-DATA-EDIT-002`）；
- 版本冲突返回 `zk.bad_version`，不自动覆盖、不盲目重试。

## 未完成

- 查找/替换与撤销（`REQ-DATA-EDIT-003/004`）；
- 格式校验错误位置（`REQ-DATA-EDIT-005`）；
- 二进制数据的完整替换流程（`REQ-DATA-BYTES-002`）。

## 验证

```text
vue-tsc / biome / just check   通过
cargo test -p zkui-core --lib  51 passed
```
