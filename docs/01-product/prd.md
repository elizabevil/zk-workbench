---
doc_id: PRD
title: ZK UI 产品需求
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# ZK UI 产品需求

> 本文件是叙述层：描述产品意图与范围。可验收行为以 `req/` 的语句为准（见 `META.md` 第 12 节）；两者冲突时以 `req/` 为准。

## 产品定位

ZK UI 是面向开发和运维人员的 ZooKeeper 桌面管理工作台。产品重点解决状态一致性、写操作安全和工作流连续性，而非单纯增加 CRUD
数量。

## 产品边界

- 首版面向 Windows、macOS、Linux 桌面端。
- 规划节点 CRUD、ACL、多集群、SSH Tunnel、四字命令巡检、本地审计、数据格式化与 diff。
- 首版不做移动端或 Web 共享服务。
- 跨平台目标是共享 Rust 核心和一致的行为语义，不要求像素级一致。
- 首版及当前产品范围聚焦 ZooKeeper；借鉴其他中间件工具的工作区和交互模式，不把 etcd 等后端纳入当前承诺范围。新增中间件需另行完成产品定位决策。

## 产品优先级

1. **首版：单节点的可靠节点操作工作台。** 优先保证围绕一个目标 znode 的定位、读取、数据/ACL
   查看与操作、版本冲突保护、结果反馈及重连后状态可信。这里的“单节点”描述首版核心操作粒度，不预设产品只能连接一个集群。
2. **其次：节点数据维护与迁移工具。** 在可靠读写基础上增加多格式编辑、脚本化测试流程、数据导入导出和迁移能力。
3. **再次：集群运行巡检台。** 增加多集群运行指标、四字命令巡检、阈值判断等能力。

后续能力不得挤占首版核心闭环的验证和交付；若跨阶段能力是首版闭环的必要依赖，应明确说明依赖理由。

## 用户需求

各方向的可验收行为全部在 `req/`，本文件只保留产品意图。方向索引：

| 方向         | 语句文件            | 覆盖范围                                                   |
|--------------|---------------------|------------------------------------------------------------|
| 连接与会话   | `req/conn.stmts.md` | 会话复用与隔离、状态模型、generation、工作区、认证         |
| 节点操作     | `req/node.stmts.md` | 创建、删除、类型、影响预览、部分失败                       |
| 数据格式     | `req/data.stmts.md` | 格式化视图、编辑缓冲、diff/CAS、搜索替换、大节点、导入导出 |
| 工作台与树   | `req/ui.stmts.md`   | 按需展开、缓存失效、路径直达、搜索范围、状态、布局         |
| 命令行与脚本 | `req/cli.stmts.md`  | 命令边界、补全、顺序执行、首错停止、结果隔离               |
| ACL 与身份   | `req/auth.stmts.md` | ACL 后置、权限错误区分、凭据保护                           |
| 巡检与追溯   | `req/ops.stmts.md`  | 四字命令巡检、本地追溯（后续阶段）                         |

### PRD-001 连接和工作区

用户可以管理多个 ZooKeeper 集群连接，并在本地恢复工作现场。行为与验收见 `req/conn.stmts.md` 的 `REQ-CONN-WORKSPACE-001`。

### PRD-002 浏览与定位节点

用户可以按需浏览节点树，也可以通过绝对路径直达节点。行为与验收见 `req/ui.stmts.md` 的 `REQ-UI-TREE-001`、`REQ-UI-TREE-004`。

### PRD-003 查看和编辑节点

用户可以查看节点数据与 Stat，并对持久、临时、顺序等 ZooKeeper 节点类型执行创建、读取、修改和删除。首版支持递归操作。ACL
权限管理后置；首版是否只读显示 ACL 待确认。数据查看与格式化的具体行为见 `req/data.stmts.md` 的 `REQ-DATA-VIEW-001`。

删除的危险性、确认步骤、影响范围与部分失败报告属于可验收行为，以 `req/node.stmts.md` 的 `REQ-NODE-DEL-001` 至
`REQ-NODE-DEL-008` 为准；本条款不再重复描述。

### PRD-004 安全写入

写操作以 ZooKeeper version 检查保护并发修改。行为与验收见 `req/data.stmts.md` 的 `REQ-DATA-EDIT-002`。

### PRD-005 连接状态与实时性

应用应清楚呈现连接、断开、重连与数据可能过期的状态。行为与验收见 `req/conn.stmts.md` 的 `REQ-CONN-STATE-001`、
`REQ-CONN-STATE-002`、`REQ-CONN-GEN-001`、`REQ-CONN-GEN-002`。

### PRD-006 集群巡检

用户可以按集群、实例、指标查看四字命令结果及阈值状态。行为与验收见 `req/ops.stmts.md` 的 `REQ-OPS-INSPECT-001`、
`REQ-OPS-INSPECT-002`。

### PRD-007 本地操作追溯

应用记录自身发起的节点与 ACL 变更，并提供节点历史与全局时间线。行为与验收见 `req/ops.stmts.md` 的 `REQ-OPS-AUDIT-001`、
`REQ-OPS-AUDIT-002`。

### PRD-008 大节点与数据交换

大节点默认显示长度和预览，用户按需加载完整内容。行为与验收见 `req/data.stmts.md` 的 `REQ-DATA-LARGE-001`、
`REQ-DATA-EXPORT-001`。

### PRD-011 多格式数据查看与编辑

用户可以查看 ZooKeeper 节点原始数据，并对 JSON、YAML 提供格式化与语法高亮，编辑器支持常规搜索与替换。行为与验收见
`req/data.stmts.md` 的 `REQ-DATA-VIEW-001` 至 `REQ-DATA-EDIT-005`；不可解码字节的处理见 `REQ-DATA-BYTES-001`。

### PRD-012 ZooKeeper 应用命令行

应用提供面向 ZooKeeper 操作的命令行入口，作为 UI 的一部分，支持自动补全并可保存命令与脚本；不提供任意操作系统 shell。行为与验收见
`req/cli.stmts.md` 的 `REQ-CLI-SCOPE-001` 至 `REQ-CLI-COMPLETE-001`。

### PRD-013 脚本流程执行

用户可以保存 ZooKeeper 命令脚本并按行顺序执行，也可逐行执行当前脚本。行为与验收见 `req/cli.stmts.md` 的
`REQ-CLI-SCRIPT-001` 至 `REQ-CLI-RESULT-003`；命令区高亮与结果展示的交互见 `02-design/ui-design.md`。

### PRD-014 多连接工作台

用户可以在同一工作台管理多个 ZooKeeper 连接，并在连接、节点树、数据编辑器和命令入口之间切换；交互组织参考
DataGrip，不复刻其视觉设计。首版的活动连接与工作区约束见 `req/conn.stmts.md` 的 `REQ-CONN-WORKSPACE-002`。

### PRD-015 按路径绑定认证身份（第二版）

用户可以为本地工作区中的路径或路径前缀绑定认证身份配置。此能力计划在第二版交付；首版只支持连接级认证，不提供路径级自动切换。行为与验收见
`req/conn.stmts.md` 的 `REQ-CONN-AUTH-001`、`REQ-CONN-AUTH-002`。

## 工程约束

### PRD-009 技术栈

当前方向为 Rust + Tauri 2 + Vue 3。技术选型属于决策层，见 `dec/tech-route.md` 的 `DEC-001`；技术变更只改决策，不改任何
`req/` 语句。

### PRD-010 安全边界

凭据保护与命令权限的安全边界见 `req/auth.stmts.md` 的 `REQ-AUTH-CRED-001`、`REQ-AUTH-CRED-002`；命令权限最小化属于实现决策，见
`dec/tech-route.md` 的 `DEC-005`。

## 暂不纳入

移动端、Web 共享服务、多用户远程工作区、集中式合规审计、未经确认的离线写入和自动同步。
