---
doc_id: CHG-0013
title: 命令集扩展为 zkCli 3.9 全量
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0013：命令集扩展为 zkCli 3.9 全量

## 目的

`CHG-0012` 把命令集限定为 10 个命令、禁用 `-w`/`ls -R` 与扩展命令。负责人指出 `setAcl` 等同样是 zkCli 原生命令，应尽量提供，并以
ZooKeeper 3.9 官方文档为准；`connect`、`quit` 由界面连接管理承担，不必提供；`:cmd` 作为后续能力，首版不加。

## 依据

- 官方文档：[ZooKeeper 3.9 CLI](https://zookeeper.apache.org/doc/current/zookeeperCLI.html)（命令表来自该页 `help` 输出）。
- 负责人决定：3.9 为准；不提供 `connect`/`quit`；首版不加 `:cmd`。

## 命令集（首版）

| 类别         | 命令                                                                                       |
|--------------|--------------------------------------------------------------------------------------------|
| 节点与数据   | `create` `delete` `deleteall` `get` `set` `ls` `stat` `sync`                               |
| ACL 与认证   | `getAcl` `setAcl` `addauth`                                                                |
| 配额         | `listquota` `setquota` `delquota`                                                          |
| 批量与元信息 | `getAllChildrenNumber` `getEphemerals`                                                     |
| 集群与客户端 | `config` `reconfig` `version` `whoami`                                                     |
| 会话内辅助   | `history` `redo` `printwatches` `removewatches` `help`                                     |
| 明确不提供   | `connect` `quit` `close`（连接由界面管理）；四字命令（不在该文档命令表内，巡检属第三阶段） |

## flag（与 3.9 文档一致）

`get -s/-w`、`ls -s/-w/-R`、`stat -w`、`create -s/-e/-c/-t`、`delete -v`、`set -s/-v`、`setAcl -s/-v/-R`、`getAcl -s`、
`config -c/-w/-s`、`setquota|delquota -n/-b/-N/-B`、`removewatches -c/-d/-a/-l`、
`reconfig -s/-v/-file/-members/-add/-remove`。

## 安全围栏

`setAcl`、`addauth`、`setquota`、`delquota`、`removewatches`、`reconfig` 执行前必须展示目标路径、参数摘要、当前连接与身份，并要求二次确认（
`REQ-CLI-DANGER-001`）。`deleteall` 仍要求目标路径与递归删除确认（`REQ-CLI-DELETEALL-001`）。

## 被取代的语句

| 原语句                                 | 新语句                                                            |
|----------------------------------------|-------------------------------------------------------------------|
| `REQ-CLI-SCOPE-003`（10 命令白名单）   | `REQ-CLI-GRAMMAR-002`（以 3.9 文档为准）                          |
| `REQ-CLI-SCOPE-004`（禁 `-w`/`ls -R`） | `REQ-CLI-FLAGS-002`（flag 与文档一致）                            |
| `REQ-CLI-FLAGS-001`（flag 白名单）     | `REQ-CLI-FLAGS-002`                                               |
| `REQ-CLI-SCOPE-005`（无扩展命令）      | `REQ-CLI-EXT-002`（首版不加，列为后续）                           |
| `REQ-AUTH-ACL-001`（ACL 编辑后置）     | `REQ-AUTH-ACL-004`（setAcl 提供 ACL 编辑，面板只读）              |
| `REQ-CONN-AUTH-001`（身份不得叠加）    | `REQ-CONN-AUTH-006`（连接级独立 session；addauth 显式叠加需确认） |

## 影响

- ACL：首版可通过 `setAcl` 修改 ACL（含 `-R`、`-v`），但界面 ACL 面板仍只读。
- 认证：`addauth` 会改变当前 session 的认证集合，必须显式确认并展示身份变化。
- 风险：`reconfig` 会改变集群成员，属高危；仅在有超级用户与 `reconfigEnabled=true` 的集群可用，失败信息必须原样呈现。
- 验证：`ver/cli.md` 新增命令表、flag、扩展命令、危险命令四组用例；两条 ACL/认证用例补入 `ver/auth.md`、`ver/conn.md`。

## 非目标（Non-Goals）

1. 不提供 `connect`、`quit`、`close`，连接生命周期由界面管理。
2. 不提供四字命令（第三阶段巡检再评估）。
3. 首版不提供 `:cmd` 扩展命令与自定义快捷键绑定。
4. 不改变写操作摘要、结果隔离、脚本执行等既有语句。
5. 不让命令绕过 Rust 侧的权限、CAS 与审计控制。

## 候选方案与权衡（Alternatives & Trade-offs）

**保持 10 命令白名单**：与「尽量提供原生命令」的诉求冲突，用户仍需另开 zkCli；被拒绝。

**全量包含 connect/quit/close**：会绕过界面连接状态与工作区恢复，`quit` 在 GUI 中语义不明；被拒绝，由界面承担。

**同时提供 `:cmd` 扩展**：首版缺少明确需求，且增加语法分流与补全复杂度；被拒绝，列为后续。

## 遗留

- 目标服务端最低版本未定（`DEC-043` 为 `proposed`）。
- 四字命令是否进入 CLI 需与第三阶段巡检一起评审。
- `reconfig` 的可用性依赖服务端配置，需要在验证环境实测。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
