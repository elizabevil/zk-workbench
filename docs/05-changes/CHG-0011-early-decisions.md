---
doc_id: CHG-0011
title: 首版早期决策包
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0011：首版早期决策包

## 目的

在实现前把不依赖实测、但会影响实现与验收口径的决策一次收敛，减少反复。

## 决策

### A1 凭据保存范围

产品负责人确定为四档，新建时默认「永不保存」：

| 范围          | 语义                          | 存储         |
|---------------|-------------------------------|--------------|
| 永不保存      | 仅本次连接有效，断开即失效    | 内存         |
| for session   | 保留到当前 ZooKeeper 会话关闭 | 内存         |
| until restart | 保留到应用退出                | 内存         |
| forever       | 跨重启可用                    | 系统凭据存储 |

除 forever 外不得写入磁盘；无可用系统凭据存储时只能使用前三档。对应 `REQ-AUTH-CRED-003`、`REQ-AUTH-CRED-004`。

### A2 首版发布判据

以五条端到端场景为出口：连接与工作区恢复、路径直达、diff + CAS 保存、并发冲突拒绝、重连后标记过期。用例见 `ver/release.md`。

### A3 脚本文件范围

脚本存于应用私有目录；可显式打开/保存普通文本；不做通用文件管理；脚本不得内嵌凭据。对应 `REQ-CLI-SCRIPT-005`。

### A4 查询结果持久性

不跨重启保留。对应 `REQ-CLI-RESULT-004`。

### A5 大节点阈值

预览 4 KiB、超过 1 MiB 提示、完整加载显式触发；标为实验值。因数值用户可见，按 `META.md` 第 9 节写入 `REQ-DATA-LARGE-002` 而非
`dec/`。

### A6 平台最低版本

Windows 10 1809+、macOS 12+、Ubuntu 22.04+（WebKitGTK 2.40+），作为验证目标，见 `dec/platform.md` 的 `DEC-030`。

### A7 遥测

首版不外发遥测、崩溃上报或使用数据。对应 `REQ-AUTH-PRIV-001`。

### A8 SSH Tunnel

不进入首版，归入第二阶段。对应 `REQ-CONN-SSH-001`。

## 影响文件

`req/auth.stmts.md`、`req/conn.stmts.md`、`req/data.stmts.md`、`req/cli.stmts.md` 新增语句；`ver/` 补对应用例；新增
`ver/release.md`、`dec/platform.md`；`dec/tech-route.md` 的 `DEC-004` 补四档范围。

## 非目标（Non-Goals）

1. 不新增未在决策中列出的产品能力。
2. 不改变已确认的删除、创建、ACL 与二进制替换行为。
3. 不把实验值写成最终承诺。
4. 不决定 CLI 语法（`REQ-CLI-SCOPE-*` 的语法基础待单独确认）。
5. 不涉及发布签名与自动更新（见 `DEC-031`）。

## 候选方案与权衡（Alternatives & Trade-offs）

**凭据只提供「永不保存 / forever」两档**：用户在不同设备与共享环境下的风险偏好差异大，两档不足以覆盖；改为四档。

**大节点阈值写进 `dec/`**：与 `META.md` 第 9 节「用户可观察数字属于 REQ」冲突，且会绕开验收；改为写入 REQ 并标注实验值。

## 遗留

- CLI 语法已在 `CHG-0012` 收敛为 zkCli 兼容语法，`REQ-CLI-SCOPE-001`、`REQ-CLI-SCOPE-002` 已 supersede。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
