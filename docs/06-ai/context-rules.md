---
doc_id: AI-CONTEXT-RULES
title: AI 任务上下文加载规则
status: active
---

# AI 任务上下文加载规则

## 稳定性分层

| 层 | 内容 | 稳定性 | 载体 |
|---|---|---|---|
| 系统层 | AI 角色、项目定位、审批边界和硬约束 | 极稳定 | `collaboration.md`、`guardrails.md` |
| 任务层 | 本次工作目标、范围、输出和验收条件 | 每次任务变化 | `prompts/task-*.md` |
| 上下文层 | 本次相关 PRD、设计、规格、测试和变更 | 按任务裁剪 | 由本文件决定 |

## 通用上下文

每项项目任务至少读取：

1. `docs/README.md` 与 `docs/META.md`（文档规范）。
2. `06-ai/collaboration.md` 与 `06-ai/guardrails.md`。
3. 当前任务提示词及其点名的项目文档。

若外部规范文件不可访问，不得假装已遵循其中未见到的 front matter、状态机或变更格式；应沿用当前可读结构，并在结果中指出规范缺口。

## 按任务加载

| 任务类型 | 额外上下文 |
|---|---|
| 产品需求讨论 | `01-product/prd.md`、`01-product/roadmap.md`、相关 CHG 和待决策项 |
| 功能设计 | 相关 `req/` 语句、`02-design/tech-design.md`、对应 `dec/` 决策 |
| 技术调查 | 调查目标对应的 PRD/设计/规格、`04-test/test-plan.md`、相关来源记录 |
| 需求变更 | 当前条款、依赖/验收/测试引用、相关 ADR/CHG；评估影响后再改基线 |
| 测试规划 | 相关需求和规格、`04-test/test-plan.md`、技术验证结论 |
| 文档治理 | `docs/META.md` 文档规范、README 索引和受影响文件 |
| 代码修改（必读） | `06-ai/code-generation.md`、`06-ai/code-engineering.md`、本次相关需求/设计/规格、相关实现和测试、manifest/锁文件/工具链配置 |

### 编码任务按改动范围裁剪

编码任务**始终**读取 `code-generation.md` 与 `code-engineering.md`。`code-application.md` 按改动域选择章节，避免无关场景信息进入上下文：

| 改动范围 | 追加加载 |
|---|---|
| Tauri command/event、Rust service、ZooKeeper client、session、连接池、Watcher、树缓存、节点读写 | `code-application.md` 第 1–4、7 节，以及对应设计/功能规格 |
| Vue 页面、Router、Pinia、数据编辑器、CLI 交互 | `code-application.md` 第 1–2、4–5、7 节，以及对应 UI/功能规格 |
| DSL parser、命令执行、补全、脚本结果 | `code-application.md` 第 1–2、5、7 节，以及 CLI 规格 |
| 凭据、身份、权限、危险操作、审计 | `code-application.md` 第 1–4、6–7 节，以及对应安全/认证规格；发现规则不足时加载全文 |
| 依赖选型/升级、语言或工具链版本、模块拆包、构建、封装、跨平台发布 | `code-application.md` 第 1–2、6 节，`05-changes/CHG-0005`、验证路线和相关 manifest/CI |

如果一项改动跨多个范围，合并相应章节。局部任务无需加载未涉及的需求文档；若实现中发现调用链、数据格式或安全边界跨域，再补读相关章节。

只加载与任务有关的上下文；如果发现跨文件冲突，再读取被引用文件并记录影响，不默认把整套文档重复粘贴进任务提示词。
