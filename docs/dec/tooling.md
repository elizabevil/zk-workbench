---
doc_id: DEC-TOOLING
title: 工具链、配置分层与错误模型决策
status: accepted
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 工具链、配置分层与错误模型决策

DEC-060   status: accepted   updated: 2026-11-02
  选择：前端 lint 与格式化使用 Biome 单一工具（`biome check`），类型检查用 `vue-tsc --noEmit`，测试用 Vitest；版本锁定，不使用 ESLint + Prettier 组合。Oxlint 作为备选：若 Biome 的 Vue 解析在项目中出问题，切换到 Oxlint 做 lint（其 `.vue` 仅覆盖 `<script>` 块）。
  依据：Biome v2.3 起支持 Vue/Svelte/Astro 的 script、style 与模板格式化（`html.experimentalFullSupportEnabled`）；Oxlint 对 `.vue` 只 lint `<script>` 块，表单与样式需另配格式化工具。二者均为 Rust 实现，性能满足要求。
  推翻条件：Biome 的 Vue 实验支持出现阻塞性解析问题，或 Oxlint 的表单支持成熟到可替代。
  影响：新增 `biome.json` 与 `package.json` 脚本；不引入 ESLint 体系。

DEC-061   status: accepted   updated: 2026-11-02
  选择：应用配置分两层——内嵌默认配置（随二进制打包）+ 用户自定义配置（用户数据目录覆盖）。样式令牌用 `theme.json`，行为配置用 `app.toml`；配置只允许调整呈现与阈值，不得改变行为语义（如关闭删除确认）。
  依据：负责人要求「代码固定、配置可调」；配置不得成为绕过需求的通道。
  推翻条件：出现必须由配置切换的产品行为，此时应升级为显式功能开关并经评审。
  影响：配置 schema 与校验在 Rust 侧实现；前端只消费渲染后的令牌。

DEC-062   status: proposed   updated: 2026-11-02
  选择：Cargo 采用 workspace，但当前只设一个业务 crate `crates/zkui-core`，`src-tauri` 作为薄壳成员；Spike 与集成测试放 `crates/zkui-core/tests/`。暂不拆 CLI parser 与 IPC 类型 crate。
  依据：核心逻辑需要脱离 Tauri 依赖独立编译与测试；单 core crate 避免过早拆包。
  推翻条件：出现第二个独立消费方（如独立 CLI 二进制）或类型生成需要隔离。
  影响：`project/Cargo.toml` 变为 workspace 根；迁移现有脚手架代码。

DEC-063   status: accepted   updated: 2026-11-02
  选择：错误模型以 ZooKeeper 原生错误码为基座（与 Java 客户端 `KeeperException.Code` 一一对应），应用与 IPC 侧使用带前缀的 `kind`：`zk_*`、`app_*`、`ipc_*`，并携带 `scope`、`code`、结构化字段与 `message_key`；原始异常文本只进日志与详情，不作为协议。
  依据：负责人要求尽量遵循 ZK 错误码并加应用区分；前端按 `message_key` 做国际化。
  推翻条件：ZK 客户端库无法稳定暴露原生错误码。
  影响：错误类型定义在 `zkui-core`，前端不解析文案。
