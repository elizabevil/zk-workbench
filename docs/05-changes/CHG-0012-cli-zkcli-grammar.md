---
doc_id: CHG-0012
title: 命令语法改为 zkCli 兼容
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0012：命令语法改为 zkCli 兼容

## 目的

纠正当初对 CLI 需求的假设：原始需求是在 UI 命令区直接使用 ZooKeeper 官方 zkCli 语法（如 `get /node`），此前的语句把它当作「待定义的应用自研 DSL」。

## 决策

产品负责人确认 Q1–Q4 按推荐执行。

### 语法

- 语法以 ZooKeeper 官方 zkCli 为准：位置参数 + flag。
- 引号与转义只在脚本层补充（znode 名称可含空格），不改变 zkCli 语义。
- 脚本以 `#` 开头的行为注释（应用层扩展，zkCli 原生无注释）。

### 命令集（首版）

- 支持：`ls`、`get`、`set`、`create`、`delete`、`deleteall`、`stat`、`getAcl`、`sync`、`help`。
- 不支持并明确报错：`setAcl`（ACL 编辑后置）、`addauth`（认证在连接层）、配额命令、`close`/`quit`（连接由界面管理）、`history`/`redo`、四字命令（巡检属第三阶段）。

### flag

- 支持：`get -s`、`create -e`、`create -s`、`create -c`、`create -t`、`set -v`、`delete -v`。
- 不支持：`-w`（watcher 订阅）、`ls -R`（全树遍历）；未支持的 flag 必须明确报错。

### 扩展命令与 deleteall

- 不提供应用扩展命令，保持与官方 zkCli 一致的命名空间。
- `deleteall` 允许使用，执行前展示目标路径与递归删除确认；不在命令行统计后代数量（与 UI 删除的预览规则不同，已在语句中单独说明）。

## 语句变更

| 原语句 | 处理 | 新语句 |
|---|---|---|
| `REQ-CLI-SCOPE-001` | supersede | `REQ-CLI-GRAMMAR-001` |
| `REQ-CLI-SCOPE-002` | supersede | `REQ-CLI-GRAMMAR-001` |
| — | 新增 | `REQ-CLI-SCOPE-003`（命令白名单） |
| — | 新增 | `REQ-CLI-SCOPE-004`（-w / ls -R 不支持，未知 flag 报错） |
| — | 新增 | `REQ-CLI-FLAGS-001`（支持的 flag 清单） |
| — | 新增 | `REQ-CLI-SCOPE-005`（无应用扩展命令） |
| — | 新增 | `REQ-CLI-DELETEALL-001`（deleteall 确认规则） |
| — | 新增 | `REQ-CLI-SCRIPT-006`（`#` 注释） |

未受影响并继续有效：补全、保存、顺序执行、首错停止、写操作摘要、结果展示与隔离。

## 影响文件

`req/cli.stmts.md`、`ver/cli.md`（两条旧用例置 `stale`，新增七条）、新增 `dec/cli-grammar.md`、`locks.yml`、`README.md` 计数。

## 非目标（Non-Goals）

1. 不实现 `-w` 与 `ls -R`（第三阶段再评估，见 `DEC-041`）。
2. 不提供 `setAcl`/`addauth`（ACL 编辑与连接级认证的既有边界不变）。
3. 不提供任何应用扩展命令或 `:` 前缀语法。
4. 不改变脚本执行、结果隔离与写操作摘要的既有语句。
5. 不让命令绕过 Rust 侧的权限、CAS 与审计控制。

## 候选方案与权衡（Alternatives & Trade-offs）

**保留应用自研 DSL**：与用户原始需求不符，且用户需重新学习命令；被拒绝。

**全量兼容 zkCli（含 -w、ls -R、setAcl、addauth、配额、history/redo）**：与首版 ACL 后置、按需加载、缓存失效策略冲突，且部分命令能力未验证；被拒绝，改为白名单 + 明确报错。

**用 `:` 前缀提供应用扩展命令**：首版无实际需要，且会增加语法歧义；被拒绝，留作后续选项。

## 遗留

- `-w`、`ls -R` 待第三阶段评估（`DEC-041`）。
- 命令补全的候选来源（命令表、参数 schema）需在实现时与解析器共用同一份定义，避免前后端两份语法。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
