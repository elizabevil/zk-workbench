---
doc_id: CHG-0045
title: S3 变更审计：SQLite 哈希链
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0045：S3 变更审计

## 交付

| 层 | 内容 |
|---|---|
| 存储 | `<data_dir>/zkui/audit.db`，SQLite WAL；`audit_log` 表 |
| 完整性 | 每条记录含 `prev_hash`/`hash`（SHA-256），提供校验并定位断链位置 |
| 记录范围 | 仅本应用变更：`set`、`create`、`delete`；成功与失败都记录 |
| 命令 | `audit_list`、`audit_verify`、`audit_prune`、`audit_clear`、`audit_export` |
| 前端 | 设置页「审计」：最近 100 条、按路径过滤（命令支持）、校验、按保留期清理、清空（确认）、导出 SQLite 快照 |

## 行为

- 审计不记录读取；
- 清空写入 `audit.clear` 标记并重置链；过期裁剪保留剩余链，校验以首条为锚点；
- 导出使用 `VACUUM INTO`，生成一致性快照；
- 保留期沿用设置中的 90 天默认值。

## 已知限制

1. 写入为同步 SQLite（WAL），尚未使用异步队列，极端情况下可能短暂阻塞；
2. 未自动定时裁剪，需手动触发；
3. ACL 变更尚未实现，`setAcl` 审计待 ACL 功能；
4. 节点历史按路径过滤的 UI 输入尚未接线（命令已支持 `path_filter`）。

## 验证

```text
cargo test -p zkui-core --lib   65 passed（含追加/篡改检测/清空）
vue-tsc / biome / just check    通过
```
