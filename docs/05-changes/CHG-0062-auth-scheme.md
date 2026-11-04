---
doc_id: CHG-0062
title: 认证 scheme 类型化与 digest-only 约束
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0062：认证 scheme 类型化

- `Identity.scheme` 从字符串改为 `AuthScheme` 枚举（digest/sasl/ip/x509）；
- 首版只启用 digest：连接身份与 `addauth` 遇到其他 scheme 明确报错；
- `Identity` 仍只保存展示信息，不保存凭据；
- 命令区 `addauth` 使用同一枚举解析。

## 决策

ZooKeeper 认证类型包括 digest、SASL/Kerberos、IP、X509。首版仅支持 digest；SASL 需要 JAAS/Kerberos 配置与额外测试环境，后续单独评估。

## 验证

```text
cargo test -p zkui-core --lib   通过
vue-tsc                         通过
```
