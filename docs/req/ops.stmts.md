---
doc_id: REQ-OPS
title: 巡检与追溯需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 巡检与追溯需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`01-product/prd.md`。本域为第三阶段与第二阶段的后续能力，语句带 `#deferred` 标记，交付时间由路线图决定。

REQ-OPS-INSPECT-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：用户必须能按集群、实例、指标查看 {{four-letter-command}} 结果与阈值状态。
  标签：#capability #deferred
  验证：VER-OPS-INSPECT-001
  来源：PRD-006

REQ-OPS-INSPECT-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：巡检不可用不得误报为 ZooKeeper 连接故障。
  标签：#error #deferred
  验证：VER-OPS-INSPECT-002
  来源：PRD-006

REQ-OPS-AUDIT-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：应用必须记录自身发起的节点与 {{acl}} 变更，包括时间、连接、路径、操作类型、版本变化与结果。
  标签：#capability #deferred
  验证：VER-OPS-AUDIT-001
  来源：PRD-007

REQ-OPS-AUDIT-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：必须提供节点历史与全局时间线；不得宣称是合规审计，也不得覆盖其他客户端发起的操作。
  标签：#constraint #deferred
  验证：VER-OPS-AUDIT-002
  来源：PRD-007

REQ-OPS-LOG-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：运行日志必须默认写入当前操作系统用户文档目录下与应用同名的子目录；日志中不得出现凭据。
  标签：#capability #security
  验证：VER-OPS-LOG-001
  来源：CHG-0017
