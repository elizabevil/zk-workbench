---
doc_id: CHG-0044
title: 本地性能指标
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0044：本地性能指标

## 交付

| 层   | 内容                                                           |
|------|----------------------------------------------------------------|
| core | `Metrics` 原子计数器：连接尝试/失败、重连、读/写/创建/删除次数 |
| 延迟 | 读写延迟按 1/5/10/50/100/500/1000ms 桶统计，输出 P50/P95       |
| 命令 | `metrics_snapshot`                                             |
| 前端 | 设置页「可观测」展示指标并提供刷新                             |

## 说明

- 指标仅存内存，不落盘、不外发（`REQ-OPS-OBS-002`）；
- OTel 导出开关与端点仍在配置中，实际 OTLP 导出尚未实现（`REQ-OPS-OBS-001` 未闭环）。

## 验证

```text
cargo test -p zkui-core --lib   59 passed（含分位用例）
vue-tsc / biome / just check    通过
```
