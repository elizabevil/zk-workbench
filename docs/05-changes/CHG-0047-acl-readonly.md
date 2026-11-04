---
doc_id: CHG-0047
title: ACL 只读面板
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0047：ACL 只读面板

## 交付

| 层   | 内容                                                                     |
|------|--------------------------------------------------------------------------|
| DTO  | `AclEntry { scheme, id, permissions }`，权限字母 `c/r/w/d/a`             |
| core | `ConnectionManager::acl`（get_acl，记录读指标）                          |
| 命令 | `node_acl`                                                               |
| 前端 | 数据/Stat/ACL 三标签；ACL 表格；空列表提示；权限不足按 `zk.no_auth` 显示 |

## 行为

- ACL 面板只读（`REQ-AUTH-ACL-002`、`REQ-AUTH-ACL-004`）；
- 权限不足与空 ACL 区分：错误按 `zk.no_auth` 显示，空列表显示空提示；
- 编辑 ACL 仍通过命令区 `setAcl`（命令区未实现）。

## 验证

```text
vue-tsc / biome / just check   通过
```
