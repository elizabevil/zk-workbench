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

VER-OPS-AUDIT-003   status: pending
  对应：REQ-OPS-AUDIT-003
  步骤：执行 create/set/delete/setAcl 与读取，确认仅变更操作入审计且字段完整。
  结果：未执行

VER-OPS-AUDIT-004   status: pending
  对应：REQ-OPS-AUDIT-004
  步骤：分批写入后校验哈希链；篡改一条记录后校验应失败；导出后节点历史与时间线一致。
  结果：未执行

VER-OPS-AUDIT-005   status: pending
  对应：REQ-OPS-AUDIT-005
  步骤：模拟磁盘只读或写满，确认操作不被阻塞且界面出现写入失败提示。
  结果：未执行

VER-OPS-OBS-001   status: pending
  对应：REQ-OPS-OBS-001
  步骤：默认启动检查无外发连接；开启 OTel 导出后能看到导出请求，关闭后停止。
  结果：未执行

VER-OPS-OBS-002   status: pending
  对应：REQ-OPS-OBS-002
  步骤：执行一组操作后检查指标：延迟分位、计数、缓存命中失效、重连次数均有数据且可复算。
  结果：未执行

VER-OPS-AUDIT-006   status: pending
  对应：REQ-OPS-AUDIT-006
  步骤：修改保留期并触发清理，确认二次确认展示待删范围；清理后保留期生效。
  结果：未执行
