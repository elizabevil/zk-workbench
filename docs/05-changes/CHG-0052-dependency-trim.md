---
doc_id: CHG-0052
title: 依赖层精简
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0052：依赖层精简

## 变更

| 项 | 之前 | 之后 |
|---|---|---|
| `opentelemetry-proto` | 默认特性（`full`：trace/logs/zpages/serde + tonic channel） | `default-features = false` + `gen-tonic-messages` + `metrics` |
| `tonic` 依赖链 | 存在（channel/hyper-timeout/tonic-prost） | 已移除 |
| `tauri-plugin-opener` | 注册但未使用 | 移除（Rust/npm/capability） |
| `vue-i18n` 构建 | 默认 bundler 入口 | runtime-only 入口 |

Rust 依赖数量：**242 → 228**（`cargo tree -p zkui-core` 去重统计）。

## 保留项与原因

- `keyring` 使用默认 `v1` 特性，覆盖 Linux/Windows/macOS 系统凭据存储；
- `rusqlite` 保留 `bundled`，避免依赖系统 SQLite 版本；
- `async-http-proxy` 仍用于 HTTP CONNECT（自带 base64 0.13，属历史依赖）；
- `zookeeper-client` 内部 ahash 0.7/0.8 双版本，属上游问题。

## 验证

```text
cargo check / just check       通过
npm run build                  通过
vue-tsc / biome                通过
```
