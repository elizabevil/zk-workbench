---
doc_id: CHG-0008
title: 全域规范化与旧文档收敛
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0008：全域规范化与旧文档收敛

## 目的

把此前只覆盖节点域的语句层规范扩展到全部方向，消除「同一文档内多个方向的需求描述」，并让叙述层不再承载可验收行为。

## 触发

负责人反馈：规范只完成了一部分，`prd.md`、`03-specs`、`02-design`、`04-test` 中仍有跨方向的行为描述。

## 落地内容

1. 语句层按域拆分：`conn`、`node`、`data`、`ui`、`cli`、`auth`、`ops` 七个文件，共 71 条语句（原 68 条 `accepted`、3 条 `proposed`；其中两条已由 `CHG-0009` 转为 accepted）。
2. 验证层同步：`ver/` 七个文件，覆盖全部 accepted 语句，全部 `pending`。
3. 决策层补充：`dec/data-strategy.md`（缓存、查询、Watcher）、`dec/tech-route.md` 增 `DEC-005`（Tauri 权限）。
4. 门禁从 12 项扩到 15 项：域单一性、叙述层不得定义语句、accepted 语句来源追溯。
5. 叙述层收敛：
   - `prd.md` 只保留意图与方向索引，逐条指向 `req/`；
   - `03-specs/tree-query-connections.md` 归档至 `99-archive/`，内容已全部迁出；
   - `02-design/tech-design.md` 的 `DESIGN-001..005` 改为指针，选择移入 `dec/`；
   - `04-test/test-plan.md` 只保留验证问题与故障场景，用例与结果在 `ver/`。

## 迁移映射（域 → 来源）

| 域 | 语句文件 | 主要来源 |
|---|---|---|
| 连接与会话 | `req/conn.stmts.md` | PRD-001/005/014/015、SPEC-CONN-001..003、DESIGN-001/002/005、CHG-0004 |
| 节点 | `req/node.stmts.md` | SPEC-NODE-001/002、CHG-0006 |
| 数据格式 | `req/data.stmts.md` | PRD-003/004/008/011、规格 JSON/YAML 与不可解码章节 |
| 工作台与树 | `req/ui.stmts.md` | SPEC-TREE-001..004、ui-design、CHG-0004 F |
| 命令行与脚本 | `req/cli.stmts.md` | PRD-012/013、规格 CLI 章节、CHG-0004 E |
| ACL 与身份 | `req/auth.stmts.md` | PRD-003/010、规格 ACL 与认证章节 |
| 巡检与追溯 | `req/ops.stmts.md` | PRD-006/007 |

## 影响

- 文档结构：`03-specs/` 目录消失（内容归档），新增 `99-archive/`。
- 维护方式：改行为只改 `req/`；改技术只改 `dec/`；测试结果只改 `ver/`；理由只写 `changes/`。
- 产品行为：无变化，语句逐条来自既有已确认文档。

## 非目标（Non-Goals）

1. 不新增任何产品能力或验收承诺。
2. 不迁移 `changes/` 中的历史记录。
3. 不引入自动化文档站点或 ID 渲染。
4. 不改变 `META.md` 之外的 AI 执行授权规则。
5. 不在本记录中确定尚未决策的产品边界（如 ACL 只读展示）。

## 候选方案与权衡（Alternatives & Trade-offs）

**保留 `03-specs/` 作为规格层**：会长期维持两份权威（规格与 req），门禁无法判断谁优先，已被本次问题证伪，故归档。

**把 `04-test/test-plan.md` 也迁成 VER**：验证问题与用例是不同粒度，强行合并会让「待回答的问题」与「已验证结果」混在一起，故保留分层。

**一次提交完成全部迁移**：diff 过大难以评审，改为按域分批提交，每批一次门禁。

## 遗留

- `REQ-DATA-BYTES-002`、`REQ-AUTH-ACL-002` 已由 `CHG-0009` 转为 accepted；`REQ-CONN-AUTH-003` 已由 `CHG-0010` 转为 accepted（`#deferred`，第二版）。
- 连接、CLI、UI 域语句尚未经过实现验证，`ver/` 全部为 `pending`。
- `docs/06-ai` 中仍有指向 `00-meta/docs-dir.md` 的历史表述，待单独清理。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`（15 项）。
