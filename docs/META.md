---
doc_id: META
title: 文档规范（语句 + 标签）
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 文档规范：语句 + 标签

## 1. 分层与权威

| 层     | 位置                                               | 内容                   | 权威性               |
|--------|----------------------------------------------------|------------------------|----------------------|
| 规范层 | `req/`、`dec/`、`ver/`、`glossary.md`、`locks.yml` | 行为、选择、证据、术语 | 受门禁约束，唯一权威 |
| 叙述层 | `changes/`、旧文档                                 | 背景、理由、调查、历史 | 无约束力，可自由演进 |

一句话： **行为只写在 `req/`，选择只写在 `dec/`，证据只写在 `ver/`，理由只写在 `changes/`。**

本规范只约束文档结构；`prompts/_common.md` 第 0 节的执行授权规则与 `06-ai/guardrails.md` 的审批边界优先级更高。

## 2. 目录

```text
docs/
  META.md          本规范
  glossary.md      术语唯一来源
  locks.yml        accepted 条款正文 hash
  req/<域>.stmts.md
  dec/<主题>.md
  ver/<域>.md
  changes/CHG-*.md
  99-archive/      冻结的旧文档
```

域取值：`CONN`（连接与会话）、`NODE`（节点读写）、`DATA`（数据格式、大节点、导入导出）、`UI`（工作台、树与状态）、`CLI`（命令与脚本）、
`AUTH`（ACL 与身份）、`OPS`（巡检与追溯）。子域按能力划分，如 `CREATE`、`DEL`、`TREE`、`STATE`。一个文件只放一个域。

## 3. ID 与状态

| 类型 | 格式                    | 状态机                                                 |
|------|-------------------------|--------------------------------------------------------|
| 语句 | `REQ-<域>-<子域>-<NNN>` | `draft → proposed → accepted → superseded / withdrawn` |
| 决策 | `DEC-<NNN>`             | `proposed → accepted → superseded`                     |
| 验证 | `VER-<域>-<子域>-<NNN>` | `pending → passed / failed → stale`                    |
| 变更 | `CHG-<NNNN>`            | 只追加，不改状态                                       |
| 术语 | `TERM <名>`             | 只增不删；废弃标 `deprecated`                          |

ID 永不复用。`superseded` 只能改状态字段与 `superseded-by`，正文一个字不动。

语句被 `superseded` 后，其对应 `VER` 用例直接删除，不保留 `stale` 标记。

## 4. 语句语法

```text
REQ-NODE-DEL-003   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：目标节点非空时，普通删除不得可用；用户必须显式勾选「删除 {{subtree}}」后才能确认执行。
  标签：#constraint #danger #confirm
  验证：VER-NODE-DEL-003
```

- 情态词只用： **必须 / 不得 / 应当 / 可以**。
- **每条语句必须写在单行**，不得换行续写；需要拆分就拆成多条。
- 一条语句只表达一个可验收断言。
- 豁免行（可选，紧跟 `验证：` 行）：
  `  豁免：waived-until=<YYYY-MM-DD> owner=<人名> reason=<一句话>`；到期必须重新评估，过期即门禁失败。
- 验证块（`ver/`）字段：`VER-<域>-<子域>-<NNN>   status: pending|passed|failed|stale`，
  其后为 `对应：`（REQ/DEC ID，可多个）、`步骤：`、`结果：`。`passed` 必须对应真实执行结果。
- `accepted` 语句必须带 `  来源：<PRD/SPEC/CHG 编号>`，指向迁移前的出处。
- 可选字段：`superseded-by`、`waived-until`（到期前必须有 owner 与原因）。

## 5. 标签词表（封闭，13 个）

`#capability` `#constraint` `#invariant` `#error` `#security` `#danger` `#confirm` `#preview` `#performance`
`#threshold` `#ux` `#deferred` `#waived`

新增标签必须单独提交并说明理由；语句出现词表外的标签即门禁失败。状态用 `status:` 字段，不是标签。

## 6. 引用规则（单向 DAG）

- `ver/` → `req/`、`dec/`；`dec/` → `req/`；`req/` → 仅 `glossary.md`。
- 引用一律写 **ID**，不写文件路径；反查用 `grep -rn "REQ-NODE-DEL-002" docs/`。
- 术语在语句中写成 `{{名字}}`，名字必须在 `glossary.md` 有 `TERM` 定义。
- `req/` 之外的任何文件不得复述需求正文。
- 叙述层（`01-product/`、`02-design/`、`03-specs/`、`04-test/`）不得出现 `REQ-`/`VER-`/`DEC-` 定义行，只能按 ID 引用。

## 7. 不可变锁

- `accepted` 语句的正文 hash 记入 `locks.yml`，格式 `REQ-NODE-DEL-002: sha256:<hex>`。
- 归一化：取 `语句：` 之后到行尾的单行文本，去除行尾空白，按 UTF-8 计算 sha256。
- 语义变更 = 新 ID + 旧条款 `superseded`；非语义修订（错别字、标点）允许单独提交，提交信息必须含 `lock-update`，并用
  `just --justfile docs/justfile lock` 重新生成锁表。
- 门禁比对失败即 FAIL，不允许「先跑通再说」。

## 8. 技术中立

`req/` 出现以下实现栈词汇即 FAIL：

```text
Rust  Tauri  Vue  Vite  WebView  npm  pnpm  cargo  SQLite
zookeeper-client  zookeeper-async  keyring  CodeMirror  Monaco  Pinia
```

例外：`JSON`、`YAML`、`HEX` 等 **用户可见的数据格式**允许出现。技术选择只写在 `dec/`。

## 9. 阈值归属

| 类型                                                 | 归属                  | 判定             |
|------------------------------------------------------|-----------------------|------------------|
| 用户可观察的数字（预览前 100 条、`≥5000`、超时提示） | `req/` + `#threshold` | 用户能看到       |
| 内部参数（缓存 TTL、连接上限、遍历并发、重试次数）   | `dec/`                | 只有开发者能看到 |

## 10. 测试周期

- 每个周期在 `ver/` 记录 `CYCLE: <YYYY-MM-DD>` 及纳入的 REQ ID 列表。
- 周期内： **REQ 冻结**（新增需负责人确认；supersede 需负责人签字并终止当前周期）、 **DEC 放开**。
- 依赖变更后执行 `grep -rn "<DEC-ID>" docs/ver/`，命中用例标 `stale`，重跑后才能恢复 `passed`。
- 同一周期 `DEC` 选型变更 ≥3 次，判定选型不稳，停止打补丁，回到 Spike 重新评估并写 CHG。
- `failed` 与修复后的 `passed` 都必须保留，不得覆盖历史结果。

## 11. 门禁

`just --justfile docs/justfile check` 至少覆盖：

v0：残留扫描、叙述层 frontmatter、本地链接、REQ 语法与 ID 唯一、标签合法、术语引用存在、`req/` 技术词扫描、accepted hash
匹配、supersede 链完整。

v1：VER 语法与 ID 唯一、引用存在、accepted 语句用例覆盖；`passed`/`failed`/`stale` 与结果文本一致；依赖已 `superseded` DEC
的用例必须 `stale`；`waived-until` 过期与豁免缺 owner/reason。

v2：`req/` 域单一性（文件名域 = 语句域）、叙述层不得定义语句/用例/决策、`accepted` 语句必须有来源。

v0、v1、v2 均已落地，共 15 项门禁。

## 12. 过渡期

- 已迁移的域：旧文档对应章节删除并留一行指针，旧文档其余部分停止维护。
- 未迁移的域：旧文档继续有效；技术类改动只落 `02-design/` 与 `05-changes/`。
- 旧文档与 `req/` 冲突时，以 `req/` 为准。

## 13. 非目标（Non-Goals）

1. 不引入 YAML/JSON 需求库或 BDD 工具链。
2. 不自动生成文档渲染站点或 ID 站内链接。
3. 不迁移 `changes/` 历史记录，历史保持原样。
4. 不覆盖 `_common.md` 的执行授权规则与 `guardrails.md` 的审批边界。
5. 不在本规范中定义产品行为本身。

## 14. 候选方案与权衡（Alternatives & Trade-offs）

**全产品单一语句文件**：评审 diff 冲突大、并发编辑互相覆盖、grep 噪音高，改为按域拆分。

**纯 YAML/JSON 需求库**：机器友好但产品负责人无法直接评审，改为 Markdown 内嵌语句。

**只靠流程约定、不锁 hash**：无法阻止开发中途改写已确认条款，已被实际发生的问题证伪，故保留 hash 锁。
