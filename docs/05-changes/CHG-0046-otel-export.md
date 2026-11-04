---
doc_id: CHG-0046
title: OpenTelemetry OTLP 导出
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0046：OpenTelemetry OTLP 导出

## 交付

| 项       | 内容                                                                  |
|----------|-----------------------------------------------------------------------|
| 协议     | OTLP/HTTP + protobuf，`POST {endpoint}/v1/metrics`                    |
| 指标     | 连接尝试/失败、重连、读写/创建/删除次数、读/写 P50/P95，全部为 Gauge  |
| 资源属性 | `service.name=zkui`、`service.version`                                |
| 触发     | 用户显式启用后每 30s 自动导出；设置页「立即导出」手动触发             |
| 状态     | 最近导出时间与最近错误显示在设置页；失败返回 `app.otel_export_failed` |

## 行为

- 默认关闭；未启用或端点为空的定时任务直接跳过；
- 手动导出在未启用时返回可分类错误；
- 指标仅来自进程内 `Metrics`，不包含用户数据或路径。

## 验证

```text
cargo test -p zkui-core --lib otel   4 passed（含本地 HTTP 服务接收 protobuf 的端到端用例）
just check                           exit 0
vue-tsc / biome                      通过
```

## 未完成

- 未实现 OTLP/gRPC；
- 未做批量重试/退避，失败仅记录并在下个周期重试；
- 采样率配置尚未作用到指标（指标为全量 Gauge）。
