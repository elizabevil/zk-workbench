---
doc_id: CHG-0015
title: 命令集收窄为 17 条并将其余能力交给界面
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0015：命令集收窄为 17 条并将其余能力交给界面

## 目的

`CHG-0013` 按 3.9 文档全量提供命令。负责人指出不必实现所有命令，其中一部分用界面表达更合适。本记录收敛命令与界面的分工。

## 决策

### 命令行保留（17 条）

`ls`、`get`、`set`、`create`、`delete`、`deleteall`、`stat`、`getAcl`、`setAcl`、`addauth`、`sync`、`getAllChildrenNumber`、
`getEphemerals`、`listquota`、`version`、`whoami`、`help`。

支持的 flag：`ls -s/-w/-R`、`get -s/-w`、`stat -w`、`create -s/-e/-c/-t`、`delete -v`、`set -s/-v`、`setAcl -s/-v/-R`、
`getAcl -s`。`-w` 只对本次命令生效，不建立长期订阅；未支持 flag 明确报错。

### 交给界面

| 原命令                     | 界面承担方式                         | 首版                       |
|----------------------------|--------------------------------------|----------------------------|
| `history`、`redo`          | 命令历史面板，保留本会话命令并可重跑 | 是（`REQ-UI-HISTORY-001`） |
| `printwatches`             | 设置项或状态栏开关                   | 否                         |
| `removewatches`            | Watcher 管理面板                     | 否                         |
| `config`                   | 集群信息面板                         | 否（第三阶段）             |
| `connect`、`quit`、`close` | 连接管理                             | 已有                       |

### 首版不提供

`reconfig`、`setquota`、`delquota`：后续以界面能力单独评估。

## 语句变更

| 原语句                               | 新语句                                                  |
|--------------------------------------|---------------------------------------------------------|
| `REQ-CLI-GRAMMAR-002`（全量 3.9）    | `REQ-CLI-SCOPE-006`（17 条命令）                        |
| `REQ-CLI-FLAGS-002`（全量 flag）     | `REQ-CLI-FLAGS-003`（对应 flag，`-w` 仅本次）           |
| `REQ-CLI-DANGER-001`（六类危险命令） | `REQ-CLI-DANGER-002`（setAcl、addauth）                 |
| —                                    | `REQ-CLI-SCOPE-007`（历史、watcher、config 由界面承担） |
| —                                    | `REQ-CLI-SCOPE-008`（reconfig 与配额写入不提供）        |
| —                                    | `REQ-UI-HISTORY-001`（界面命令历史）                    |

## 影响

- 命令区更聚焦：节点、ACL、认证、排查类命令；管理类动作回归界面。
- `ver/cli.md` 三条旧用例置 `stale`，新增五条；`ver/ui.md` 新增一条。
- 兼容性：17 条命令中 `getEphemerals`（3.6+）、`getAllChildrenNumber`（3.5+）在 3.4 会报错，符合 `REQ-CONN-COMPAT-001`
  的能力探测与明确报错。

## 非目标（Non-Goals）

1. 不改变 `setAcl`/`addauth` 的确认规则。
2. 不实现 `printwatches`/`removewatches`/`config` 的界面（后续阶段）。
3. 不提供 `reconfig` 与配额写入（任何形态）。
4. 不改变脚本执行、结果隔离与写操作摘要的既有语句。
5. 不新增 `:cmd` 扩展命令。

## 候选方案与权衡（Alternatives & Trade-offs）

**保持 3.9 全量命令**：界面已有更好表达方式的管理动作被重复实现，且 `reconfig` 风险高；被拒绝。

**连 `sync`、`getAllChildrenNumber`、`getEphemerals` 一并去掉**：这些是排查命令，界面没有对等入口，去掉会削弱排查能力；被保留。

**把 `history` 也做成命令**：与结果面板、快捷键重叠，界面表达更自然；被拒绝。

## 遗留

- `printwatches`/`removewatches` 是否在第二阶段做界面，需随 Watcher 策略一起定。
- 3.4 互操作仍需 Spike A 验证（`DEC-043`）。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
