---
doc_id: CHG-0033
title: 失败值使用确定联合类型，取消逐字段运行时校验
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0033：失败值使用确定联合类型

## 变更

`toAppError` 的入参从 `unknown` 改为确定的联合类型：

```ts
export type InvokeRejection = AppError | string | Error;
export function toAppError(raw: InvokeRejection): AppError;
```

- 只有两种非契约形态需要处理：字符串与 `Error`；
- 其余对象按 `AppError` 契约处理，不再逐字段 `typeof` 校验；
- `ErrorFields` 的恒定型保证由后端与生成的 TS 类型约束，前端运行时不重复判断；
- `catch` 中保留一次边界断言（TypeScript 的 catch 变量只能是 `unknown`）。

## 取舍

- 移除运行时形状校验后，若 Tauri 返回非 `AppError` 的对象，会被当作契约对象透传；
- 依据：跨进程边界由 Rust 的 `AppError` 序列化保证，Tauri 自身只会以字符串或 `Error` 拒绝；
- 若将来出现第三种失败形态，应在边界处扩展 `InvokeRejection`，而不是恢复逐字段判断。

## 验证

```text
vue-tsc --noEmit     通过
just check           exit 0
```
