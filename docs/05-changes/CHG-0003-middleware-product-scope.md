---
doc_id: CHG-0003
title: 多中间件产品定位调查与首版待确认项
status: proposed
---

# CHG-0003：多中间件产品定位调查与首版待确认项

## 目的

调查是否将 etcd 等中间件纳入产品，以及这是否要求从 ZooKeeper 桌面工具调整为通用中间件数据工作台。同时集中整理当前产品阶段仍需确认的需求和技术点。

## 当前产品基线

负责人已确认阶段目标：

1. 首版：单节点的可靠节点操作工作台。
2. 其次：节点数据维护与迁移工具。
3. 再其次：集群运行巡检台。

“单节点”按单个目标数据节点的操作粒度记录，不自动推断为单用户或单集群。当前 PRD 的产品对象仍是 ZooKeeper；本调查不自动改变该基线。

## 调查事实与来源

### E-001 etcd v3 数据/API 模型

[etcd 官方 API 文档](https://etcd.io/docs/v3.7/learning/api/) 说明 etcd API 以 gRPC 服务提供，包含 KV、Watch、Lease、Auth、Cluster、Maintenance 等服务；KV 操作以 key/range 为中心；事务是带比较条件的原子 If/Then/Else；Watch 可从 revision 开始订阅变化。

**对产品的含义（推论）**：etcd 功能不能简单映射成 ZooKeeper 的 znode CRUD。产品需要表达 key 前缀/range、revision/事务、lease 和 RBAC 等专属概念。

### E-002 etcd API guarantees

[etcd 官方 API guarantees](https://etcd.io/docs/v3.5/learning/api_guarantees/) 描述 KV、Watch、Lease API、历史 revision 与 compaction 等保证。

**对产品的含义（推论）**：节点版本/CAS 的交互可复用部分“并发保护”概念，但 UI 需显示 etcd revision/比较条件语义，不能统一伪装成 ZooKeeper version。

### E-003 etcd 官方集成目录中的客户端产品

[etcd 官方 integrations 目录](https://etcd.io/docs/v3.6/integrations/) 列出 etcd-manager、etcd-workbench 等客户端和工具。

**支持**：已有多种 etcd GUI/工作台，值得做竞品功能调研。

**不支持**：本项目加入 etcd 一定有市场差异或需求；这需要用户/市场验证。

### E-004 etcd-manager 项目说明

[etcd-manager GitHub](https://github.com/gtamas/etcdmanager) 自述支持 etcd v3，列出 key 浏览与编辑、TTL、历史 revision、多个配置档案、认证、集群健康、lease、用户与 role 管理等能力。

**边界**：这是项目自述，功能状态与当前维护/发布情况仍需实机和近期 release 核查。

### E-005 Etcd Workbench 项目说明

[Etcd Workbench GitHub](https://github.com/ruanjiayou/etcd-ui) 自述支持多连接、SSL/SSH、key 编辑、批量导入导出、revision 对比、key 搜索、lease、user/role 管理和桌面/网页包；其开发栈包含 Vue、Rust、Tauri、Tokio 和 etcd-client。

**对定位的含义（推论）**：若本项目多中间件化，仅“Rust + Tauri + 多连接 + 数据编辑”不足以构成差异；需先确定目标用户、跨中间件共性工作流和特有能力。

**边界**：功能来自仓库 README 自述；应复查版本、维护状态、实际体验和许可证后再用于产品决策。

### E-006 ZooKeeper GUI 对照

- [PrettyZoo 仓库](https://github.com/vran-dev/prettyZoo)列出多服务管理、节点 CRUD、ACL、SSH tunnel、配置导入导出、JSON/XML 格式化和终端操作，并说明仓库已归档。
- [ZooNavigator 仓库](https://github.com/elkozmon/zoonavigator)定位为 Web ZooKeeper UI，并列出支持的 ZooKeeper 版本范围。

**对定位的含义（推论）**：当前计划中的多连接、ACL、SSH tunnel、格式化和命令入口属于已有工具常见能力。潜在差异化仍应集中验证可靠重连状态、路径/数据权限身份体验、脚本化测试流程和本地工作流。

### E-007 ZooKeeper CLI 能力样本

[Apache ZooKeeper 3.9.6 CLI 文档](https://zookeeper.apache.org/doc/r3.9.6/zookeeperCLI.html)展示了 `ls`、`get`、`set`、`getAcl`、`setAcl`、`addauth`、`getEphemerals`、`history`、`redo` 等命令及示例。

**对本产品的含义（推论）**：官方 CLI 命令面可作为应用 CLI 语法与自动补全的基线；无需一开始覆盖所有管理命令，可按“首版节点操作闭环”和后续迁移/巡检阶段分层实现。

## 同类产品可借鉴点

以下是基于公开项目说明的产品模式提炼，不代表这些产品的实际质量已验证。

| 来源 | 公开能力/模式 | 对 ZK UI 的可借鉴点 | 阶段建议 |
|---|---|---|---|
| [etcd-manager](https://github.com/gtamas/etcdmanager) | key 浏览多视图、TTL、revision 历史/回滚、多配置档案、集群健康、lease 和 RBAC 管理 | 连接配置档案；把版本/历史作为数据详情的一等入口；在同一工作台结合数据管理与集群状态 | 配置档案可前置；历史/回滚需先评估 ZooKeeper 的可实现来源；lease/RBAC 属于 etcd 专属能力，不照搬到 ZK |
| [Etcd Workbench](https://github.com/ruanjiayou/etcd-ui) | 多连接、SSH/SSL、语法高亮、批量导入导出、路径补全、书签、变更监控、版本比较/冲突解决；Vue+Rust+Tauri | 路径自动补全和书签；多连接工作区；数据版本比较；导入导出预览；把冲突恢复做成明确工作流 | 路径补全和关键节点标记可进入 ZK 首版/近期；批量迁移、审计、复杂比较放第二阶段 |
| [PrettyZoo](https://github.com/vran-dev/prettyZoo) | 多 ZooKeeper 连接、实时同步、ACL、SSH tunnel、导入导出、节点 CRUD/搜索、终端操作、JSON/XML 格式化 | 常见操作 UI 化，同时保留高级命令入口；格式视图、ACL 和连接配置应减少页面跳转 | 首版只选可靠节点闭环必需项；功能自述中项目已归档，作为历史能力样本而非维护性标杆 |
| [ZooNavigator](https://github.com/elkozmon/zoonavigator) | ZooKeeper Web 管理界面，支持连接信息与可选 auth 配置 | 连接建立流程简洁；把认证信息与连接配置清楚组织 | 可借鉴连接 onboarding；产品为 Web 部署形态，不直接照搬部署架构 |

### 推荐吸收的通用体验

1. **工作区档案**：每个集群保留连接参数、最近路径、书签和编辑偏好；凭据单独安全保存。
2. **路径直达与补全**：完整路径可直接打开；CLI 与路径栏共享补全数据，不靠递归扫描全树。
3. **数据查看的多视图**：原始字节/文本与 JSON、YAML 等格式化视图并列；格式化是显示层行为，保存前看 diff、使用 version 检查。
4. **冲突处理工作流**：冲突时展示服务器当前值与本地草稿，允许复制/重新应用，不自动覆盖。
5. **CLI 与 UI 协同**：UI 负责可发现的高频操作，CLI/脚本负责组合和低频命令；两者共用连接上下文、错误模型和安全保护。
6. **操作可追溯**：将本工具发起的操作、查询结果和脚本逐行输出按连接与路径组织，用户可折叠查看。不要声称能还原外部客户端对 ZK 的全部历史。

### 明确不应直接照搬

- etcd 的 lease、revision、range、transaction、RBAC 不能伪装成 ZooKeeper znode、session、ACL 或 version；如果未来支持 etcd，应保留后端专属概念。
- 竞品宣传的“实时同步”“回滚”“安全存储”等需要检查其定义和实际行为，不能只按功能名称写入本项目需求。
- 多中间件支持本身不是差异化。应先以 ZooKeeper 首版验证核心工作流，再按用户需求决定是否增加 etcd。

## 集中建议

推荐将竞品优点拆为“可借鉴交互”和“中间件专属能力”：立即吸收多连接工作区、路径直达/补全、格式视图、保存前 diff、冲突恢复、CLI 补全与脚本结果组织；将 ACL/认证身份等 ZK 特性按 ZooKeeper 语义实现；不因竞品支持 etcd 就扩大首版后端范围。

### 负责人决议

**已批准：吸收同类产品的交互模式，保持 ZooKeeper 功能聚焦。** 不因竞品支持 etcd 而将 etcd 纳入当前产品或首版。

### 决议执行

- 第一阶段优先采用路径直达/补全、工作区档案、保存前 diff 和冲突恢复中直接支撑单节点可靠操作的模式。
- 第二阶段吸收多格式编辑、导入导出、命令脚本及本地迁移工作流。
- 第三阶段再建设四字命令、阈值判断等 ZooKeeper 集群巡检能力。
- etcd 及其他中间件继续作为未来独立定位决策，不添加到当前 PRD 的承诺范围。
- 已将阶段映射同步到 `01-product/roadmap.md`；产品边界更新见 `01-product/prd.md`。

## 产品范围选项

### 选项 A：首版只做 ZooKeeper，保留后续适配空间（推荐）

首版保持已确认方向；把连接、查询、编辑、执行历史等 UI 工作流设计为可复用边界，但不为了假想多后端提前抽象。用真实 ZooKeeper 用户流程验证产品价值后，再决定是否支持 etcd。

**收益**：维持首版聚焦，协议和权限语义更清楚，验证成本可控。

**代价**：若后续加入 etcd，需要新增后端适配和相应专属 UI/规格；部分早期组件可能要调整。

### 选项 B：现在定位为多中间件数据工作台

从产品定义开始纳入 ZooKeeper 与 etcd，规划通用连接工作区、数据浏览器、命令/脚本和插件式后端，同时为每个中间件保留专属数据模型、权限、事务、lease/session、watch 和巡检能力。

**收益**：统一多种中间件的管理入口，潜在受众更广。

**代价**：首版范围与架构复杂度明显上升；需要分别实现并验证两套语义，跨后端抽象可能退化为最低共同能力。

### 选项 C：插件式逐步扩展，但不承诺 etcd 首版

当前仍以 ZooKeeper 为唯一已承诺后端；设计稳定的命令/数据源边界，并将 etcd 作为下一阶段探索项。适配层只在第二后端正式获批时提炼，不提前造通用框架。

**收益**：保留产品扩张可能，同时延迟抽象决策。

**代价**：若以后要支持 etcd，需进行一次有计划的架构演进。

## 助手推荐

推荐 **选项 C**：记录多中间件作为产品定位的候选方向，但首版仍聚焦 ZooKeeper。当前先把第二后端作为调研/验证主题，不加入首版 PRD 承诺；等完成 ZooKeeper 核心闭环及用户价值验证后，再做是否纳入 etcd 的立项决策。

若负责人确认“多中间件是当前产品方向”，则应先改产品定位，再重写 PRD、数据模型、命令体系、认证/权限模型、测试矩阵和路线图；不能只在连接列表加一个 etcd 选项。

## 需求与技术待确认清单

### 首版产品范围

1. 单节点可靠操作的首版最小闭环包含哪些动作：连接、路径直达、读取、修改、ACL 查看/编辑、删除确认、刷新、冲突恢复？
2. 首版连接范围是否只需一个活动集群连接，还是多连接工作区本身也属于核心场景？
3. 首版成功标准是什么：任务完成率、节点定位耗时、冲突无损、重连后状态可信，还是其他指标？
4. 数据格式首版支持 JSON/YAML/文本/二进制中的哪些；自动识别还是用户选格式；原始字节如何查看和保存？
5. 认证身份按路径绑定是否进入首版；身份配置与 ZooKeeper session 的隔离规则是什么？

### CLI 与脚本

6. 命令语法采用 zkCli 风格兼容，还是设计更适合 UI 的命令集？当前仅确认应用内 ZooKeeper CLI，不等于承诺完整兼容官方 zkCli。
7. 自动补全覆盖命令、参数、路径和 ACL 身份的哪些部分；补全请求如何避免对集群造成大量读取？
8. 已确认脚本顺序执行、单行执行、失败立即停止、换行与 `;` 结束命令；仍需确定引号/转义、空行、注释、结果保留范围及脚本错误定位。
9. 保存命令/脚本是否支持变量、参数化和环境配置；首版推荐不支持通用控制流。

### 技术验证

10. 候选 Rust ZooKeeper 库是否满足连接、认证、ACL、version CAS、Watcher、session 重连和多 session 隔离？
11. 路径绑定多个认证配置时，是否采用独立 client/session；多身份同 session 的权限累积风险如何避免？
12. 节点缓存的 TTL、容量、事件失效和 generation 规则如何通过高延迟/断线场景验证？
13. 三平台最低系统版本、WebView 基线、凭据钥匙串、SSH tunnel、签名/更新要求是什么？
14. 数据编辑器是否需要大值分块/限制；序列化、字符编码、最大值和不可解码数据行为是什么？

## 集中审批

请一次确认以下决策：

- **产品范围**：A / B / C，或自定义方向。
- **多后端决策时间**：现在纳入产品定义，还是在 ZooKeeper 核心闭环验证后决策？
- 若保持 ZooKeeper 首版：首版是否必须包含多连接、ACL 写入、认证身份配置绑定和内置脚本编辑器？可按“必须 / 可后置”批量标注第 1、2、5、9 项。

负责人未确认前，当前已批准基线不变。
