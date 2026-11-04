---
doc_id: CHG-0001
title: 树缓存、快速查询与多连接方向调查
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0001 树缓存、快速查询与多连接方向调查

## 状态

草案：调查结论用于指导规格和验证，尚无性能测试结果或 Rust 客户端选型结论。

## 背景

项目需要降低大树和高延迟环境下的浏览成本，同时避免重连后显示陈旧数据，并支持多连接工作区。调查重点是树缓存、快速定位、客户端复用和连接隔离。

## 结论

1. 子节点按需加载；缓存只作为浏览加速，不作为节点数据编辑依据。
2. 路径直达与全局搜索是不同能力。路径直达是有限的定向请求；本地索引搜索只覆盖已加载节点；全树搜索若提供，应显式启动并控制负载。
3. ZooKeeper Watcher 不能充当完整变更日志；缓存需结合 generation、TTL、局部失效和手动刷新。
4. ZooKeeper 客户端是有状态 session。按逻辑连接复用长期客户端，不按单次操作创建连接；连接之间隔离 session、Watcher、generation、缓存和审计上下文。
5. 缓存 TTL、连接上限和空闲策略目前没有证据支持固定数值，交由 TEST-002、TEST-003 实测并由产品确认。

## 来源与证据

### E-001 Apache ZooKeeper Programmer’s Guide

[官方指南（3.4.9）](https://zookeeper.apache.org/doc/r3.4.9/zookeeperProgrammers.pdf) 描述 znode 树、读取/Stat、版本检查、session 和 Watcher。其 Watcher 章节说明经典 Watcher 为一次性通知，重新注册间隙可能发生多次变化。

**支持**：version CAS 的依据、Watcher 不等同变更日志、客户端会话有状态。

**不支持**：本产品应采用何种缓存 TTL、Rust 客户端是否线程安全或如何恢复 Watcher。该文档版本较旧，目标版本需确认。

### E-002 Rust `zookeeper` crate API

[docs.rs API](https://docs.rs/zookeeper/latest/zookeeper/) 列出 `ZooKeeper`、`Watcher`、状态、事件及 `Stat` 类型。

**支持**：这是候选 Rust 客户端的调研入口。

**不支持**：同一实例并发访问保证、重连语义、目标服务端兼容性或项目选型结论，均需 TEST-001 验证。

### E-003 Rust `zookeeper-async`

[docs.rs 项目页](https://docs.rs/crate/zookeeper-async/latest) 将其描述为基于 Tokio 的异步客户端。

**支持**：可列入候选库调查。

**不支持**：本项目所需功能已经满足或其维护状态适合本项目；需锁定版本检查并实测。

### E-004 Tauri Capabilities

[Tauri 官方 Capabilities 文档](https://tauri.app/security/capabilities/) 说明权限可按窗口/WebView 授予或限制，并可按平台配置。

**支持**：以 capability 声明平台系统命令权限的技术方向。

**不支持**：本项目未来配置自动安全；具体权限仍需逐项审查。

### E-005 Tauri Store

[Tauri Store 官方文档](https://v2.tauri.app/plugin/store/) 描述跨重启持久键值存储及 Windows、Linux、macOS 支持。

**支持**：可评估用于非敏感工作区状态。

**不支持**：Store 是凭据保险库；凭据不得因此直接明文写入。

## 影响文件

- `01-product/prd.md`：明确多连接、路径定位、缓存与追溯能力。
- `02-design/tech-design.md`：记录客户端复用、generation 和缓存查询策略。
- `03-specs/tree-query-connections.md`：细化行为边界。
- `04-test/test-plan.md`：列出尚待证明的关键能力。

## 后续

完成 TEST-001 至 TEST-003 后更新本变更单的状态与证据。若测试推翻当前结论，新增变更单并修订被影响的规格，不静默改写调查记录。
