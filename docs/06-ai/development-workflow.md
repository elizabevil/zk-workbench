---
doc_id: AI-DEV-WORKFLOW
title: 代码开发工作流与校验分层
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 代码开发工作流与校验分层

本文件定义进入实现阶段后的分支、提交、命令入口与校验分层。与 `_common.md` 冲突时以 `_common.md` 第 0 节为准。

## 1. 分支模型

- 唯一长期分支：`main`。
- 功能分支命名：`<类型>/<版本>-<功能>`，例如 `feat/v1-zk-client`、`fix/v1-cas-conflict`、`docs/v1-ui-spec`。
- **只允许 rebase，禁止 merge 提交**；禁止 `merge --no-ff`、禁止 `push --force`（除非负责人明确要求）。
- 功能完成后 cherry-pick 到 `main`，再删除功能分支；定期清理已合入的中间分支，保持历史线性可读。
- 每个功能操作都必须有 commit，不允许"改完不提交、攒着一起提"。

## 2. 提交信息

- 标题 `type(scope): 做了什么`；正文列改动与验证事实。
- 决策、理由、取舍不写进提交信息，写入 `05-changes/` 或 `dec/`；提交信息规则见 `_common.md` 第 13 条。

## 3. 命令入口分层

根 `justfile` 是统一入口，只负责编排，不实现具体检查：

```text
justfile                统一入口：check / test / env-up / env-down / perf
├─ deploy/dev.justfile  本机开发依赖
├─ docs/justfile        文档门禁（15 项）
├─ project/src-tauri/justfile   代码侧检查（fmt/clippy/test）
└─ test/perf/justfile   性能基准（启动、内存）
```

- 子 justfile 各自独立可运行，但只被根入口调用，不互相调用。
- 根入口的 `check` 汇总文档与代码两侧结果；任一侧失败即失败。

## 4. 校验分层与顺序

1. **本地先跑通流程**，再写 CI 规则；CI 只固化已验证的本地命令，不引入新检查。
2. 文档门禁：`just --justfile docs/justfile check`（当前 15 项）。
3. 代码门禁：格式、lint、类型检查、单元/集成测试分开执行，不互相替代。
4. 性能门禁：启动时间与常驻内存按 `REQ-UI-PERF-001` 测量，基准可重跑并落盘。
5. 未运行的检查必须说明，不得写成通过。
6. 任何验证都必须留下仓库内文件（测试、脚本、编排或 justfile 配方）；临时目录中的内容要先提交再引用，细则见 `06-ai/guardrails.md`。

## 5. 性能预算

- 冷启动到可交互 ≤ 2 秒。
- 常驻内存 ≤ 100 MB，尽量不超过同等空壳桌面应用基线的两倍。
- 基准实现放在代码侧 justfile，数据与结论写入 `05-changes/`。

## 6. 非目标（Non-Goals）

1. 不定义具体 CI 平台与流水线 YAML（本地流程验证后再定）。
2. 不规定代码评审规模上限。
3. 不引入 merge 提交或 squash 合并。
4. 不在此定义发布与签名流程。
5. 不重复 `_common.md` 的执行授权规则。
