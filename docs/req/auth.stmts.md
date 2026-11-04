---
doc_id: REQ-AUTH
title: ACL 与身份需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# ACL 与身份需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`01-product/prd.md`、`99-archive/03-specs-tree-query-connections.md`、`05-changes/CHG-0004`。

REQ-AUTH-ACL-001 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-AUTH-ACL-004
语句：ACL 编辑属于后续能力，首版不得提供 ACL 编辑。
标签：#constraint #danger
验证：VER-AUTH-ACL-001
来源：PRD-003；CHG-0004（A）

REQ-AUTH-ACL-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：首版只读显示 {{acl}}；权限不足导致不可见时必须与 ACL 内容为空区分。
标签：#capability #error
验证：VER-AUTH-ACL-002
来源：SPEC；CHG-0009

REQ-AUTH-ACL-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：权限不足必须与连接故障、节点不存在区分显示，不得合并为同一种错误。
标签：#error
验证：VER-AUTH-ACL-003
来源：SPEC

REQ-AUTH-CRED-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：凭据不得以明文持久化；无可用安全存储时不得降级为明文或写入普通配置文件。
标签：#security #constraint
验证：VER-AUTH-CRED-001
来源：PRD-010

REQ-AUTH-CRED-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：前端只接收非秘密的身份标识与展示名，秘密不得进入日志、错误、审计或脚本输出。
标签：#security #invariant
验证：VER-AUTH-CRED-002
来源：PRD-010

REQ-AUTH-CRED-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：凭据保存必须提供四种范围：永不保存、for session、until restart、forever；新建凭证时默认必须为永不保存。
标签：#security #ux
验证：VER-AUTH-CRED-003
来源：CHG-0011

REQ-AUTH-CRED-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：forever 范围必须写入系统凭据存储；其余三种范围只存于内存、不得写入磁盘；for session 在连接关闭时清除，until restart
在应用退出时清除。
标签：#security #invariant
验证：VER-AUTH-CRED-004
来源：CHG-0011

REQ-AUTH-PRIV-001 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-OPS-OBS-001
语句：首版不得外发遥测、崩溃上报或使用数据；诊断信息只写入本地日志。
标签：#security #constraint
验证：VER-AUTH-PRIV-001
来源：CHG-0011

REQ-AUTH-ACL-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：首版允许通过 setAcl 命令编辑 ACL（含 -R 递归与 -v 版本校验），界面 ACL 面板仍只读；执行前必须确认并展示目标路径、当前身份与新
ACL。
标签：#capability #danger #confirm
验证：VER-AUTH-ACL-004
来源：CHG-0013
