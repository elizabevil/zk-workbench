---
doc_id: SPEC-TREE-CONNECTIONS
title: 树浏览、快速查询与多连接规格
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 树浏览、快速查询与多连接规格

## 范围

定义节点树加载、缓存、快速定位、已加载节点搜索及多连接状态隔离。全树数据内容搜索不在本规格内。

### SPEC-TREE-001 按需展开

展开路径时才读取其直接子节点；普通树浏览不得递归加载未展开的分支。

### SPEC-TREE-002 复用与刷新

同一连接、同一路径在缓存有效期内重复展开可复用结果。用户可刷新当前节点；重连、generation 变化或收到相关子节点事件时使相应条目过期。

### SPEC-TREE-003 路径直达

用户输入绝对路径后，应用直接查询路径是否存在并读取目标节点，不要求先在树上逐级点击展开。

### SPEC-TREE-004 本地范围搜索

名称或路径搜索仅覆盖当前连接已经加载的路径索引。界面必须说明搜索范围，不能把局部命中或无结果表述为全局结果。

### SPEC-CONN-001 连接隔离

已迁移至 `req/conn.stmts.md` 的 `REQ-CONN-SESSION-001`；本节不再维护。

### SPEC-CONN-002 同连接复用

已迁移至 `req/conn.stmts.md` 的 `REQ-CONN-SESSION-002`；本节不再维护。

### SPEC-CONN-003 生命周期

已迁移至 `req/conn.stmts.md` 的 `REQ-CONN-STATE-001`、`REQ-CONN-CLOSE-001`；本节不再维护。

## 待补充的验收细节

具体缓存 TTL、最大缓存规模、同时活动连接数、空闲连接策略、搜索速率与性能阈值，需通过 `04-test/test-plan.md` 的验证和产品讨论确定。

## 首版基础节点操作（已迁移）

本节的节点创建与删除条款已迁移为语句层规范，见 `req/node.stmts.md` 的 `REQ-NODE-CREATE-001` 至 `REQ-NODE-CREATE-007`、`REQ-NODE-DEL-001` 至 `REQ-NODE-DEL-008`；本节不再维护，冲突时以该文件为准。迁移记录见 `05-changes/CHG-0007`。

## JSON/YAML 查看与编辑

已迁移至 `req/data.stmts.md`（`REQ-DATA-VIEW-001` 至 `REQ-DATA-BYTES-002`）；本节不再维护。

## 不可解码的原始字节（待定策略）

已迁移至 `req/data.stmts.md`（`REQ-DATA-VIEW-001` 至 `REQ-DATA-BYTES-002`）；本节不再维护。

## ACL 首版范围（待确认）

ACL 编辑已后置。首版是否只读显示 ACL 仍待确认，候选方案见 `req/auth.stmts.md` 的 `REQ-AUTH-ACL-002`（`proposed`）；本节不再维护。

## 认证身份版本范围（已迁移）

连接级认证与路径级身份的版本边界已迁移至 `req/conn.stmts.md` 的 `REQ-CONN-AUTH-001`、`REQ-CONN-AUTH-002`，ACL 相关见 `req/auth.stmts.md`；本节不再维护。
