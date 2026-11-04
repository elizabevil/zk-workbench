---
doc_id: CHG-0031
title: 错误契约改为 scope 表示域、kind 表示域内类型
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0031：错误契约改为 scope 表示域、kind 表示域内类型

## 变更

| 之前                                                                        | 之后                                                             |
|-----------------------------------------------------------------------------|------------------------------------------------------------------|
| `kind` 自带域前缀：`zk_no_node`、`app_invalid_path`、`ipc_invalid_argument` | `kind` 只表示类型：`no_node`、`invalid_path`、`invalid_argument` |
| 域由前缀隐含，`scope` 仅 `zk/app/ipc`                                       | `scope` 显式表示域：`zk`、`app`、`proxy`、`ipc`                  |
| 代理错误挂在 `app` 域                                                       | 代理独立为 `proxy` 域，错误码 `4001`（连接）/ `4002`（握手）     |
| `message_key` 由手写映射表维护                                              | 固定由 `error.<scope>.<kind>` 生成，映射表删除                   |

示例：

```json
{ "kind": "handshake", "scope": "proxy", "code": 4002,
  "message_key": "error.proxy.handshake", "fields": { "detail": "handshake: denied" } }
```

## 影响

- Rust：`ErrorScope` 增加 `Proxy`；`ErrorKind` 变体去掉前缀；`AppError::new` 生成 `message_key`。
- 前端：`ErrorKind` 类型重新生成；兜底错误的 `kind` 改为 `invalid_argument`；i18n key 不变。
- 文档：`DEC-063`、`02-design/implementation.md` 同步。

## 验证

```text
cargo test -p zkui-core --lib                 46 passed
vue-tsc --noEmit                              通过
```
