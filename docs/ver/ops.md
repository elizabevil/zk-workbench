---
doc_id: VER-OPS
title: 巡检与追溯验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 巡检与追溯验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-OPS-AUDIT-001、REQ-OPS-AUDIT-002、REQ-OPS-INSPECT-001、REQ-OPS-INSPECT-002

VER-OPS-INSPECT-001   status: pending
  对应：REQ-OPS-INSPECT-001
  步骤：执行并观察「用户必须能按集群、实例、指标查看 {{four-letter-command}} 结果与阈值状态。」所述行为，记录实际结果。
  结果：未执行

VER-OPS-INSPECT-002   status: pending
  对应：REQ-OPS-INSPECT-002
  步骤：执行并观察「巡检不可用不得误报为 ZooKeeper 连接故障。」所述行为，记录实际结果。
  结果：未执行

VER-OPS-AUDIT-001   status: pending
  对应：REQ-OPS-AUDIT-001
  步骤：执行并观察「应用必须记录自身发起的节点与 {{acl}} 变更，包括时间、连接、路径、操作类型、版本变化与结果。」所述行为，记录实际结果。
  结果：未执行

VER-OPS-AUDIT-002   status: pending
  对应：REQ-OPS-AUDIT-002
  步骤：执行并观察「必须提供节点历史与全局时间线；不得宣称是合规审计，也不得覆盖其他客户端发起的操作。」所述行为，记录实际结果。
  结果：未执行

VER-OPS-LOG-001   status: pending
  对应：REQ-OPS-LOG-001
  步骤：检查默认日志路径位于用户文档目录同名子目录；检索日志确认无凭据。
  结果：未执行
