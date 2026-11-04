---
doc_id: CHG-0032
title: 错误字段恒定型与前端错误归一
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0032：错误字段恒定型与前端错误归一

## 变更

| 位置 | 变化 |
|---|---|
| `ErrorFields` | 不再按需省略字段，始终输出 `path/version/connection_id/generation/detail`（未使用为 `null`），与生成的 TS 类型一致 |
| `ErrorKind` | 新增 `ipc.unknown`，用于非契约失败的兜底 |
| `api/client.ts` | 单一入口 `toAppError(raw)`：严格校验 AppError 形状，否则归为 `ipc.unknown` 并保留英文 detail |
| 非 Tauri 环境 | 只保留一处判断，返回固定的 `ipc.not_tauri` 错误，不再临时拼对象 |

删除的中间态处理：

- 不再对失败值做 `JSON.stringify` 猜测；
- 不再用 `raw as AppError` 断言；
- 调用方（store、页面）不再判断错误形状。

## 验证

```text
cargo test -p zkui-core --lib     47 passed（含字段恒定输出用例）
vue-tsc --noEmit                  通过
just check                        exit 0
```
