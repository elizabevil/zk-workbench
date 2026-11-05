---
doc_id: CHG-0082
title: ACL 编辑
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0082

- 新增 `node_set_acl` Tauri 命令，使用 ACL version 做 CAS 保存。
- `AclEntry` 支持反序列化，作为 ACL 写入入参。
- 后端校验 scheme、identity 与权限字母 `c/r/w/d/a`；`auth` scheme 允许空 identity。
- ACL 保存写入 `set_acl` 审计事件，记录前后 ACL version 与条目数量。
- 工作区 ACL 面板增加编辑模式：
  - 添加/删除 ACL 条目；
  - 编辑 scheme、identity；
  - 用 c/r/w/d/a 复选框编辑权限；
  - 保存、取消与校验错误提示；
  - 显示当前 ACL version。
- 保存成功后更新当前节点 Stat 与 ACL 列表，不重新加载完整数据。

## 验证

```text
vue-tsc --noEmit 通过
cargo check --lib 通过
```

界面行为由用户手动验证。
