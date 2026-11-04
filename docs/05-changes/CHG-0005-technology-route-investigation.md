---
doc_id: CHG-0005
title: 首版技术路线与依赖候选调查
status: active
updated: 2026-10-30
---

# CHG-0005：首版技术路线与依赖候选调查

## 目的与边界

收集支持 ZK UI 首版技术选型的公开事实，提出候选路线和验证顺序。当前仓库没有应用代码与锁文件；以下是技术调查初稿，不是已批准依赖清单，也不是实测结果。

调查以首版可靠 ZooKeeper 节点操作为中心：连接与 session、节点 CRUD/CAS、JSON/YAML 查看和编辑、CLI/脚本、跨平台桌面与凭据保护。多中间件抽象和集群巡检不作为本轮选型目标。

## 当前已确定的技术方向

- 桌面框架：Tauri 2。
- 前端：Vue 3。
- 业务核心：Rust。
- 产品协议：首版 ZooKeeper。

## 事实与候选比较

### 1. Rust ZooKeeper 客户端

| 候选 | 已查事实 | 初步评价（推论） |
|---|---|---|
| `zookeeper-client` | [docs.rs API](https://docs.rs/zookeeper-client/latest/zookeeper_client/) 显示版本 0.11.2；异步 API；可选 Tokio、smol、async-global-executor；包含 StateWatcher、OneshotWatcher、PersistentWatcher、MultiReader、MultiWriter、ACL、SASL/TLS feature。文档描述 `Client` 代表 ZooKeeper session，支持 clone 共享 session 和 chroot client。 | **首选验证候选**：能力与 session/generation、持久监听及原子/批量读写需求较贴近；但 API 仍为 0.x，且公开 API 文档不证明并发安全、故障恢复或目标版本互操作，必须先做 spike。来源能力摘要，不作正式通过证据。 |
| `zookeeper-async` | [docs.rs crate](https://docs.rs/crate/zookeeper-async/latest) 当前列出 5.0.0，基于 Tokio；提供连接、Watcher、ACL、Stat、错误类型等。页面列出的最近正式版本时间为 2024-03-07。 | **备选**：API 看起来覆盖基础操作且基于 Tokio；从公开 release 元数据看更新节奏较慢，需检查 GitHub 活动、issue、兼容 ZooKeeper 版本和 SASL/TLS/持久 Watcher 覆盖。 |
| `zookeeper` | [docs.rs API](https://docs.rs/zookeeper/latest/zookeeper/) 列出客户端、Watcher、ACL、Stat、状态/错误类型。 | **备选/对照**：资料足以说明它是 Rust 社区客户端，但本轮未确认异步模型、维护节奏和高级 Watcher 能力；不建议仅凭类型目录选型。 |

**当前建议**：先以 `zookeeper-client` 0.11.2 做 spike，同时用同一小型验收程序比较 `zookeeper-async`。不在验证前锁定生产依赖。必须验证：持久/递归 watcher 与服务端最低版本、重连和 session 过期事件、认证信息隔离、同客户端并发调用、CAS、递归删除部分失败、三平台编译。

### 2. Tauri 2 权限与本地存储

- [Tauri Capabilities](https://tauri.app/security/capabilities/) 支持按窗口/WebView 授予权限，并支持按平台限定 capability。
- [Tauri Store 插件](https://v2.tauri.app/plugin/store/) 提供可跨重启保存的键值存储，并列出 Windows、Linux、macOS 支持；它不是凭据保险库。
- [Tauri Stronghold 插件](https://tauri.app/plugin/stronghold/) 将自身定位为加密秘密/密钥存储；其 API 需提供用于保护 vault 的密码/密钥处理逻辑。
- Rust [`keyring`](https://docs.rs/crate/keyring/latest) 提供统一 API，使用 macOS Keychain、Windows Credential Manager 及 Linux 等平台原生凭据存储；Linux 实际可用性取决于桌面 Secret Service/keyring 环境和 backend 配置。

**候选路线（推论）**：非敏感工作区偏好存储与秘密凭据分开。一般设置/最近路径候选为 Tauri Store 或 Rust 本地配置存储；密码/认证秘密优先评估系统 keyring。Stronghold 作为需要跨平台加密 vault 时的备选，但必须定义主密钥解锁、恢复和自动锁定体验。暂不混用两个秘密库，也不把秘密放入普通 Store。

### 3. Vue 状态与代码编辑器

- Vue 官方 [状态管理指南](https://vuejs.org/guide/scaling-up/state-management.html) 将 Pinia 作为推荐的状态管理选项。
- [CodeMirror 6 官方站点](https://codemirror.net/)列出语法高亮、行号、补全、折叠、搜索/替换，并包含 JSON/YAML 语言支持；[参考手册](https://codemirror.net/docs/ref/)明确提供大小写、整词、正则和 replace all 等搜索选项。其功能以扩展模块组合。
- [Monaco Editor](https://github.com/microsoft/monaco-editor) 是 VS Code 使用的完整代码编辑器，能力广；官方 README 指出稳定承诺主要针对 `monaco.d.ts` API，其余实现细节可变。

**当前建议（推论）**：Vue 状态候选 Pinia；编辑器首选 CodeMirror 6，因当前只需 JSON/YAML 查看/编辑、基础脚本编辑、补全和搜索替换，不需要 Monaco 的完整 IDE 能力。须做 Vue/Tauri 嵌入验证、撤销/dirty state、超大文档响应和 bundle 测量。Monaco 保留为若后续需求升级为复杂脚本 IDE 时的备选。

### 4. CLI 解析与脚本

本轮未选定 parser crate。由于产品命令是受限的 ZooKeeper DSL，建议先定义小型语法（命令、位置/命名参数、引号、转义、注释、换行/分号分隔），再选择 parser 实现。不要用 shell 或 JavaScript eval，也不要仅用按空格切分的字符串解析。

建议 Rust parser 产生有类型 AST；前端补全消费命令 schema，避免前后端维护两份语法。是否采用手写 tokenizer/parser 或 parser crate，列为 spike 的低成本比较项。

### 5. 仍未调查的组件

- Vue 虚拟树组件/树控件：需结合懒加载、数万节点可视量、键盘导航、异步加载和局部刷新调研。
- JSON/YAML 格式化/解析库：候选需支持保留原始字节、错误定位、避免不必要的 deserialize/serialize 改变内容；需分别验证格式化只用于视图和明确编辑序列化。
- Tauri 文件/对话框/SSH tunnel/更新插件：根据第二阶段需求再选，SSH tunnel 涉及凭据和生命周期，需独立安全评审。
- 本地审计持久化：首版如纳入，需决定 sqlite/文件格式、权限与清理边界；暂不为此预选数据库依赖。

## 推荐初始技术路线（待负责人批准）

1. Tauri 2 + Vue 3 + Rust；前后端经受限 Tauri commands/events 通信。
2. ZooKeeper 客户端以 `zookeeper-client` 0.11.2 为首选候选进行对照验证；暂不直接承诺选型。
3. Vue 前端以 Vite + TypeScript + Vue Router + Pinia 为推荐工具链候选；轻量工作区偏好不与业务缓存混在一个持久化 store。
4. 文本编辑器以 CodeMirror 6 为推荐候选；Monaco 不进入首版默认依赖。
5. 一般偏好与凭据分开；凭据先验证 `keyring` 原生后端，再与 Tauri Stronghold 比较解锁/恢复和平台体验。
6. CLI 由有类型 DSL parser 驱动，不执行任意 shell/JS。
7. 虚拟树、JSON/YAML parsing、审计存储尚不选库，先定义验收场景，再做最小对比。

## 验证路线

负责人已批准按本文件所列初步验证路线开展调查与 spike。此批准仅授权验证工作，不代表已批准任何候选 crate、npm 包、版本或最终技术路线。

### Spike A：Rust ZooKeeper 客户端对比

制作同一测试矩阵，对 `zookeeper-client` 与 `zookeeper-async` 实测连接、所有节点创建模式、读写、CAS、ACL 读取、连接级认证、Watcher、会话断开/过期/恢复、递归删除部分失败、编译平台和客户端并发访问。记录 crate/服务端版本、结果和维护证据。通过判据由 `TEST-001` 需求确认后补充。

### Spike B：Vue 编辑器

分别在实际 Tauri WebView 中嵌入 CodeMirror 6；只有在项目明确需要 IDE 级命令脚本能力时才额外评估 Monaco。覆盖 JSON/YAML 高亮、只读格式化视图、搜索/替换、长数据加载、dirty state、撤销、键盘操作和打包体积。

### Spike C：凭据存储

在 Windows Credential Manager、macOS Keychain、Linux Secret Service 环境测试 `keyring` 保存/读取/删除、无 keyring/headless 行为、打包与用户提示；再与 Stronghold 主密钥解锁/恢复流程作产品体验比较。不得把失败静默回退到明文或普通配置文件。

### Spike D：状态与树

确定树组件验收规模后，对候选组件测试懒加载、虚拟化、刷新局部性、键盘导航、selection 保持和过期请求取消。

## 初稿结论

目前足以收敛“候选优先顺序”，不足以宣布最终依赖：`zookeeper-client` + Vite/TypeScript/Vue Router/Pinia + CodeMirror 6 + 系统 keyring 是最值得先验证的路线；其关键风险集中在 ZooKeeper 客户端成熟度/目标版本兼容、Linux keyring 可用性和跨平台 WebView 编辑器性能。完成 Spike A-C 后再批准生产依赖。

## 前端工具链候选

| 能力 | 候选/推荐 | 官方资料支持的事实 | 当前约束建议 |
|---|---|---|---|
| Vue 工程脚手架与打包 | Vue 3 + `create-vue` + Vite + 官方 `@vitejs/plugin-vue` | [Vue 官方工具链](https://vuejs.org/guide/scaling-up/tooling)推荐新项目使用 Vite；Vite 有 Vue SFC 官方集成。 | 推荐；避免从旧 Vue CLI 起步。版本随首个可复现仓库初始化并锁定。 |
| 编程语言 | TypeScript | [Vue 官方 TypeScript 指南](https://vuejs.org/guide/typescript/overview)说明 Vue 一等支持 TS；Vite 只转译、不做类型检查。 | 推荐所有应用代码使用 TS；启用 `strict`。不允许把任意 `any` 作为逃避类型设计的常规方式。 |
| SFC 类型检查 | `vue-tsc` | Vue 官方建议对 Vue SFC 使用 `vue-tsc`；Vite 本身不 type-check。 | CI/检查脚本需有独立 type-check 步骤，不以成功构建代替类型检查。 |
| 路由 | Vue Router | [Vue Router 官方文档](https://router.vuejs.org/)说明其为 Vue 官方客户端路由，支持嵌套路由和多种 history。 | 推荐使用；history 模式需按 Tauri 深链/刷新行为 spike 决定，不默认写死。 |
| 全局 UI 状态 | Pinia | Vue 官方状态管理指南推荐 Pinia。 | 推荐将连接选择、工作区面板和用户偏好放入有边界的 stores；大型树缓存/查询结果不要无界持久化进 store。 |
| 编辑器 | CodeMirror 6 | 官方支持 JSON/YAML 语言、高亮、补全和搜索/替换。 | 推荐；复杂功能按扩展加载，另测大文档和 WebView 内存。 |
| 单元/组件测试 | Vitest + Vue Test Utils（后者需补证） | [Vitest 官方文档](https://vitest.dev/guide/)说明其与 Vite 集成；Vue 工具链指南推荐 Vitest 做 Vite 项目测试。 | 推荐 Vitest；版本需与选定 Vite/Node 版本兼容。 |
| SFC lint | ESLint + `eslint-plugin-vue` + TypeScript ESLint | [Vue 官方工具链](https://vuejs.org/guide/scaling-up/tooling)推荐 `eslint-plugin-vue`。 | 推荐 ESLint flat config；具体规则集需在仓库初始化时定版。 |
| 格式化 | Prettier（可选） | Vue 官方工具链说明 Prettier 支持 Vue SFC 格式化。 | 建议明确 ESLint/Prettier 分工，避免双重格式规则；不是产品运行依赖。 |
| 包管理器 | pnpm 或 npm | Vue/Vite 官方脚手架支持多个包管理器。 | 尚未有仓库，不锁定；建立仓库时只选择一个并提交唯一 lockfile，CI 使用 frozen lockfile。 |

### 对 AI 代码质量的初步硬约束建议

- Vue 源文件使用 `.ts`/`<script setup lang="ts">`；启用 TypeScript strict 和 SFC type-check。
- Vite build 不等于 type-check；CI 至少分开运行 lint、type-check、unit/component tests 和 production build。
- 依赖用 lockfile 复现；普通功能任务不得无理由升级主版本或更换同类库。
- Vue Router 负责视图导航，Pinia 负责边界清晰的跨组件 UI/工作区状态；远端数据缓存、业务操作及权限决策仍由 Rust 核心权威处理。
- Vite 官方当前指南列出 Node.js 20.19+ 或 22.12+；该版本号须随最终 Vite 主版本一起锁定，不能脱离版本永久固化。

## 来源清单（访问日期：2026-10-30）

1. [zookeeper-client docs.rs](https://docs.rs/zookeeper-client/latest/zookeeper_client/) — API、runtime feature、Watcher、ACL 与 reader/writer 辅助。
2. [zookeeper-async docs.rs](https://docs.rs/crate/zookeeper-async/latest) — 版本、依赖及项目说明。
3. [zookeeper docs.rs](https://docs.rs/zookeeper/latest/zookeeper/) — API 类型目录。
4. [Tauri Capabilities](https://tauri.app/security/capabilities/) — IPC/插件权限与平台限定。
5. [Tauri Store](https://v2.tauri.app/plugin/store/) — 普通持久化存储及平台支持。
6. [Tauri Stronghold](https://tauri.app/plugin/stronghold/) — 加密秘密存储及密钥初始化要求。
7. [Rust keyring](https://docs.rs/crate/keyring/latest) — 跨平台原生凭据 backend。
8. [Vue State Management](https://vuejs.org/guide/scaling-up/state-management.html) — Pinia 推荐。
9. [CodeMirror](https://codemirror.net/)、[CodeMirror Reference](https://codemirror.net/docs/ref/) — JSON/YAML 语言、搜索替换与扩展模型。
10. [Monaco Editor README](https://github.com/microsoft/monaco-editor) — 编辑器定位与 API 稳定边界。

## 审批事项

负责人已批准初步验证方案。按 Spike A-C 收集实测证据后再批准生产依赖；前端路线现增加路由、构建、TypeScript、类型检查、测试、lint 与格式化候选。具体版本及 Node/package manager 在仓库初始化时作为受控技术决策另行确定。
