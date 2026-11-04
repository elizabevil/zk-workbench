---
doc_id: DESIGN
title: ZK UI 技术设计
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# ZK UI 技术设计

## 分层

- **Rust 共享核心**：客户端连接、会话状态、节点操作、Watcher、四字命令、本地审计、凭据处理。
- **Tauri 2 应用层**：桌面集成与受控 IPC；权限使用 capabilities 配置。
- **Vue 3 前端**：工作区、树和数据编辑交互、状态展示；不实现 ZooKeeper 协议逻辑。

### DESIGN-001 有状态客户端复用

行为约束见 `req/conn.stmts.md` 的 `REQ-CONN-SESSION-001`、`REQ-CONN-SESSION-002`；本节只保留设计意图：按逻辑连接复用长期客户端，不把 session 当作无状态连接借还。

### DESIGN-002 会话世代

行为约束见 `req/conn.stmts.md` 的 `REQ-CONN-GEN-001` 与 `req/ui.stmts.md` 的 `REQ-UI-TREE-003`；IPC 契约仍待设计，属于决策层。

### DESIGN-003 树缓存与索引

缓存 TTL、容量与淘汰策略见 `dec/data-strategy.md` 的 `DEC-020`；失效行为见 `req/ui.stmts.md` 的 `REQ-UI-TREE-002`、`REQ-UI-TREE-003`。

### DESIGN-004 查询策略

查询与搜索策略见 `dec/data-strategy.md` 的 `DEC-021`；搜索范围约束见 `req/ui.stmts.md` 的 `REQ-UI-TREE-005`。

### DESIGN-005 Watcher 限制

Watcher 语义见 `dec/data-strategy.md` 的 `DEC-022`；事件与旧 generation 的行为约束见 `req/conn.stmts.md` 的 `REQ-CONN-GEN-002`。

## 跨平台约定

三平台的 WebView 版本与系统集成存在差异。具体平台策略统一维护在本节，不在功能规格中重复罗列。行为一致优先于像素级一致；快捷键、文件对话框、菜单、托盘和更新入口可按平台适配。
