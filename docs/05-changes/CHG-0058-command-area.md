---
doc_id: CHG-0058
title: 命令区与认证命令
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0058：命令区与认证命令

## 交付

| 项 | 内容 |
|---|---|
| 解析 | zkCli 风格词法（引号/转义）、17 个首版命令、未知命令与 `-w` 明确报错 |
| 执行 | `ls`（含 `-R/-s`）、`get`、`set`、`create`、`delete`、`deleteall`、`stat`、`getAcl`、`setAcl`（含 `-R/-v`）、`addauth`、`sync`、`getAllChildrenNumber`、`getEphemerals`、`listquota`、`version`、`whoami`、`help` |
| 认证 | `addauth scheme auth` 调用客户端 `auth`；`whoami` 列出当前认证身份 |
| 预览 | 写命令执行前展示连接名与摘要，取消则不执行 |
| 历史 | 会话内历史，`addauth` 的凭据在历史中掩码 |
| UI | 工作区底部命令面板：输出、输入、执行、历史重跑 |

## 已知限制

- `get/auth` 的凭据仍以命令文本传入，历史已掩码，但进程内仍存在明文；
- 未实现四档凭据保存范围（`REQ-AUTH-CRED-003/004`）；
- `setAcl -R` 暂不对每个子节点做版本校验；
- 脚本与历史持久化未实现（S10）。

## 验证

```text
cargo test -p zkui-core --lib command  4 passed
just check                             exit 0
vue-tsc / biome                        通过
```
