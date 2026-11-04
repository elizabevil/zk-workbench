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

REQ-OPS-AUDIT-001   status: superseded   since: 2026-11-02   owner: 产品负责人   superseded-by: REQ-OPS-AUDIT-003
  语句：应用必须记录自身发起的节点与 {{acl}} 变更，包括时间、连接、路径、操作类型、版本变化与结果。
  标签：#capability #deferred
  验证：VER-OPS-AUDIT-001
  来源：PRD-007

REQ-OPS-AUDIT-002   status: superseded   since: 2026-11-02   owner: 产品负责人   superseded-by: REQ-OPS-AUDIT-004
  语句：必须提供节点历史与全局时间线；不得宣称是合规审计，也不得覆盖其他客户端发起的操作。
  标签：#constraint #deferred
  验证：VER-OPS-AUDIT-002
  来源：PRD-007

REQ-OPS-LOG-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：运行日志必须默认写入当前操作系统用户文档目录下与应用同名的子目录；日志中不得出现凭据。
  标签：#capability #security
  验证：VER-OPS-LOG-001
  来源：CHG-0017

REQ-OPS-AUDIT-003   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：审计只记录本应用发起的变更操作（创建、修改、删除、ACL 修改），不记录读取；每条记录必须包含时间、连接、路径、操作类型、变更前后版本与结果。
  标签：#capability
  验证：VER-OPS-AUDIT-003
  来源：CHG-0026

REQ-OPS-AUDIT-004   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：审计记录必须本地持久保存，并提供节点历史与全局时间线；记录以哈希链保证可校验的完整性，支持完整性校验与导出；不得宣称是合规审计，也不得覆盖其他客户端发起的操作。
  标签：#capability #security
  验证：VER-OPS-AUDIT-004
  来源：CHG-0026

REQ-OPS-AUDIT-005   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：审计写入不得阻塞用户操作；写入失败必须在界面可见，不得静默丢弃记录。
  标签：#invariant #error
  验证：VER-OPS-AUDIT-005
  来源：CHG-0026

REQ-OPS-OBS-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：默认不得外发任何数据；OpenTelemetry 导出必须由用户在应用内显式开启，可随时关闭，配置本地保存。
  标签：#security #constraint
  验证：VER-OPS-OBS-001
  来源：CHG-0026

REQ-OPS-OBS-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：必须采集客观性能指标：请求延迟分位数、请求计数、缓存命中与失效次数、重连次数；指标仅本地保存并在应用内展示。
  标签：#performance #capability
  验证：VER-OPS-OBS-002
  来源：CHG-0026

REQ-OPS-AUDIT-006   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：审计保留期默认 90 天且必须可配置；清理前必须二次确认并展示将删除的记录范围。
  标签：#capability #confirm
  验证：VER-OPS-AUDIT-006
  来源：CHG-0027
