---
doc_id: DEC-OBSERVABILITY
title: 审计与可观测决策
status: accepted
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 审计与可观测决策

DEC-080 status: accepted updated: 2026-11-02
选择：审计只记录本应用发起的变更操作（创建、修改、删除、ACL 修改），不记录读取；存储使用 SQLite（WAL 模式），记录带 `prev_hash` /
`hash` 的哈希链，提供完整性校验与导出（`VACUUM INTO` 快照）；保留期默认 90 天且可配置，清理需二次确认并展示待删范围。
依据：负责人明确「只关注增删改」；哈希链满足可追溯与防篡改的最低要求。
推翻条件：审计量级或查询需求超出 SQLite 能力，或需要跨设备汇总。
影响：新增 `REQ-OPS-AUDIT-003..005`；审计写入走异步队列，失败可见。

DEC-081 status: accepted updated: 2026-11-02
选择：OpenTelemetry 导出默认关闭，由用户在应用内显式开启并可随时关闭；协议固定 http/protobuf，端点与采样率可配置；默认只保留本地日志与指标。
依据：负责人确认；与 `REQ-OPS-OBS-001` 一致。
推翻条件：企业部署要求强制导出（需另行评审隐私与合规）。
影响：supersede `REQ-AUTH-PRIV-001`；新增可选依赖（`opentelemetry-otlp`，feature 隔离）。

DEC-082 status: accepted updated: 2026-11-02
选择：本地指标固定采集请求延迟分位数、请求计数、缓存命中/失效次数与重连次数；数据仅存内存与本地滚动文件，不默认外发。
依据：`REQ-OPS-OBS-002`；性能结论需要客观数据而非主观描述。
推翻条件：指标采集本身造成可测量开销（需基准证明）。
影响：连接层与树缓存需要埋点；基准与结论写入 `05-changes/`。
