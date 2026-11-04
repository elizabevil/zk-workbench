---
doc_id: CHG-0007
title: 语句加标签规范试点与节点域迁移
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0007：语句+标签规范试点与节点域迁移

## 目的

以节点创建/删除为试点，验证「单一权威 + 单向引用 + 不可变锁」能否降低文档数量与链接维护成本，并阻止实现或测试阶段改写已确认条款。

## 触发

负责人指出三个问题：文档过多且互相链接难以维护；开发过程中会破坏最初的「不可变更内容」约定；测试验证阶段可能重新选择技术架构，导致需求文档反复改动。

## 本次落地

1. 新增 `META.md`：文档规范（分层、ID、状态机、标签词表、引用规则、hash 锁、技术中立、阈值归属、测试周期、门禁）。
2. 新增 `glossary.md`：术语唯一来源，语句用 `{{名字}}` 引用。
3. 新增 `req/node.stmts.md`：15 条 `accepted` 语句；`locks.yml` 记录其正文 hash。
4. 新增 `dec/tech-route.md`、`dec/interaction.md`：技术栈、客户端、编辑器、凭据、布局与删除/创建方案，全部 `proposed`，不改变
   REQ。
5. 新增 `ver/node.md`：15 条用例，全部 `pending`，不含未执行的结果。
6. `03-specs/tree-query-connections.md` 删除「首版基础节点操作」整节，改为一行指针，避免双权威。
7. `justfile` 的 `check` 增加语句门禁，并新增 `lock` 用于生成/刷新 hash。

## 迁移映射

| 原位置                               | 原条款                                | 新 ID                                                         |
|--------------------------------------|---------------------------------------|---------------------------------------------------------------|
| `03-specs/tree-query-connections.md` | SPEC-NODE-001（创建对话框与类型行为） | REQ-NODE-CREATE-001、REQ-NODE-CREATE-005、REQ-NODE-CREATE-006 |
| `03-specs/tree-query-connections.md` | SPEC-NODE-001（父路径与错误分类）     | REQ-NODE-CREATE-003、REQ-NODE-CREATE-004、REQ-NODE-CREATE-007 |
| `03-specs/tree-query-connections.md` | SPEC-NODE-001（原始文本提交）         | REQ-NODE-CREATE-002                                           |
| `03-specs/tree-query-connections.md` | SPEC-NODE-002（确认与影响范围）       | REQ-NODE-DEL-001                                              |
| `03-specs/tree-query-connections.md` | SPEC-NODE-002（空节点与非空节点模式） | REQ-NODE-DEL-002、REQ-NODE-DEL-003                            |
| `03-specs/tree-query-connections.md` | SPEC-NODE-002（预览规则）             | REQ-NODE-DEL-004、REQ-NODE-DEL-005                            |
| `03-specs/tree-query-connections.md` | SPEC-NODE-002（分项结果与重试）       | REQ-NODE-DEL-006、REQ-NODE-DEL-007                            |
| `05-changes/CHG-0006`                | D1 方案（禁止指定子节点模式）         | REQ-NODE-DEL-008                                              |

## 影响

- 文档数量：新增 7 个规范层文件；`03-specs` 减少一节。
- 引用方式：由文件路径链接改为 ID 引用，反查用 `grep -rn "REQ-NODE-DEL-003" docs/`。
- 行为：无变化，REQ 内容与 `CHG-0006` 已确认的 D1/C1 一致。
- 仍有效：`prd.md`、`roadmap.md`、`tech-design.md`、`ui-design.md`、`test-plan.md` 保持原状，冲突时按 `META.md` 第 12 节以
  `req/` 为准。

## 未完成与后续

- 门禁 v1 项已在后续提交中补齐：VER 语法/ID/引用/覆盖、`passed` 与结果一致、依赖 superseded DEC 必须 `stale`、`waived-until`
  过期检查。
- 其余域（连接、CLI、UI）尚未迁移，旧文档继续有效。
- `PRD-003` 与 REQ-NODE 的叙述重叠已按最小去重处理：PRD-003 保留产品范围描述，删除行为细节改为指向 `REQ-NODE-DEL-001` 至
  `REQ-NODE-DEL-008`；其余 PRD 条款维持叙述层，待对应域迁移时逐条处理。

## 回退

全部改动在分支 `docs/stmts-pilot`。回退：`git checkout master`（丢弃分支现场）；已合并时 `git revert <commit>`。

## 验证

见提交信息中的门禁结果；`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
