# ZK UI 文档索引

## 规范层（唯一权威，受 15 项门禁约束）

| 位置 | 职责 | 内容 |
|---|---|---|
| `META.md` | 文档规范 | 分层、ID、状态机、标签词表、引用规则、hash 锁、技术中立、门禁 |
| `glossary.md` | 术语表 | 每个术语只定义一次；语句用 `{{名字}}` 引用 |
| `locks.yml` | 锁表 | `accepted` 语句正文 hash，由 `just lock` 生成 |
| `req/` | 需求语句 | 按域拆分，一文件一方向；`accepted` 正文不可修改 |
| `dec/` | 决策 | 技术选型、交互、缓存策略，可被推翻，不改 REQ |
| `ver/` | 验证 | 用例与实测结果；未执行一律 `pending` |

### 语句文件（`req/`）

| 文件 | 方向 | 语句数 |
|---|---|---|
| [conn.stmts.md](req/conn.stmts.md) | 连接与会话 | 16 |
| [node.stmts.md](req/node.stmts.md) | 节点创建与删除 | 15 |
| [data.stmts.md](req/data.stmts.md) | 数据格式与读写 | 12 |
| [ui.stmts.md](req/ui.stmts.md) | 工作台与树浏览 | 12 |
| [cli.stmts.md](req/cli.stmts.md) | 命令行与脚本（zkCli 语法） | 31 |
| [auth.stmts.md](req/auth.stmts.md) | ACL 与身份 | 9 |
| [ops.stmts.md](req/ops.stmts.md) | 巡检与追溯 | 5 |

## 叙述层（无约束力，可自由演进）

| 目录 | 职责 | 内容 |
|---|---|---|
| `01-product/` | 产品意图 | 定位、边界、优先级与方向索引（逐条指向 `req/`） |
| `02-design/` | 架构与界面 | 技术分层、DESIGN 指针、UI 信息架构与交互意图 |
| `04-test/` | 验证策略 | 待回答的验证问题与故障场景；用例与结果在 `ver/` |
| `05-changes/` | 变更记录 | 讨论、证据、迁移映射与回退 |
| `06-ai/` | AI 协作规则 | 角色、护栏、上下文加载、代码生成规范与任务提示词 |
| `99-archive/` | 归档 | 已迁出的旧文档，只读 |

门禁：`just --justfile docs/justfile check`，期望 `RESULT: PASS`。

- 语义变更：新开 ID，旧条款置 `superseded`（正文不动），并刷新 `locks.yml`。
- 技术变更：只改 `dec/`，不得为了让实现通过而修改 `req/`。
- 行为只写在 `req/`；叙述层不得复述需求正文，也不得出现语句定义行。

## 文档导航

- [文档规范（META）](META.md)
- [术语表](glossary.md)
- 语句：[连接](req/conn.stmts.md)、[节点](req/node.stmts.md)、[数据](req/data.stmts.md)、[工作台](req/ui.stmts.md)、[CLI](req/cli.stmts.md)、[ACL 与身份](req/auth.stmts.md)、[巡检与追溯](req/ops.stmts.md)
- 决策：[技术与依赖](dec/tech-route.md)、[交互与布局](dec/interaction.md)、[缓存与查询](dec/data-strategy.md)
- 验证：[节点](ver/node.md)、[连接](ver/conn.md)、[数据](ver/data.md)、[工作台](ver/ui.md)、[CLI](ver/cli.md)、[ACL 与身份](ver/auth.md)、[巡检与追溯](ver/ops.md)
- [产品需求](01-product/prd.md)
- [产品路线图](01-product/roadmap.md)
- [技术设计](02-design/tech-design.md)
- [UI 设计说明](02-design/ui-design.md)
- [技术验证计划](04-test/test-plan.md)
- [产品验证路线图](04-test/validation-roadmap.md)
- [IPC 契约与文档同步](dec/ipc.md)
- [AI 协作规则](06-ai/collaboration.md)
- [AI 代码生成规则入口](06-ai/code-generation.md)
- [AI 通用代码与工程规则](06-ai/code-engineering.md)
- [ZK UI 应用代码约束](06-ai/code-application.md)
- [AI 任务上下文加载规则](06-ai/context-rules.md)
- [代码开发工作流与校验分层](06-ai/development-workflow.md)
- [树缓存与连接方向调查](05-changes/CHG-0001-tree-cache-and-connections.md)
- [活动产品讨论](05-changes/CHG-0002-product-discovery.md)
- [多中间件定位调查](05-changes/CHG-0003-middleware-product-scope.md)
- [首版交互集中审批包](05-changes/CHG-0004-first-release-interaction-decisions.md)
- [技术路线调查与依赖候选](05-changes/CHG-0005-technology-route-investigation.md)
- [首版删除与创建行为决议](05-changes/CHG-0006-first-release-delete-and-create.md)
- [语句规范试点与节点域迁移](05-changes/CHG-0007-statement-pilot.md)
- [全域规范化与旧文档收敛](05-changes/CHG-0008-domain-migration.md)
- 归档：[旧功能规格](99-archive/03-specs-tree-query-connections.md)

当前阶段是需求和技术调查。叙述层条款 ID 使用 `PRD-*`、`ROADMAP-*`、`DESIGN-*`、`TEST-*`、`CHG-*`；规范层使用 `REQ-*`、`DEC-*`、`VER-*`。
