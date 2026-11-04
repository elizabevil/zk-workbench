---
doc_id: DEC-DATA-STRATEGY
title: 缓存、查询与数据策略决策
status: proposed
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 缓存、查询与数据策略决策

本文件只记录实现策略选择；行为约束由 `req/` 定义，此处不复述正文。

DEC-020   status: proposed   updated: 2026-11-02
  选择：子节点列表按「连接 + 路径 + generation」分区缓存，TTL 初始取 30 秒作为实验值，容量上限与淘汰策略由 TEST-002 实测决定。
  依据：`DESIGN-003` 调查结论；30 秒不是已批准需求。
  推翻条件：实测显示 30 秒造成明显陈旧数据或请求放大。
  影响：行为约束见 `req/ui.stmts.md` 中的缓存失效与刷新语句。

DEC-021   status: proposed   updated: 2026-11-02
  选择：路径直达按输入路径逐级查询；全树搜索不进入首版默认能力，若提供必须由用户显式启动，并提供范围、限速、进度与取消。
  依据：`DESIGN-004`；全树搜索会产生大量请求。
  推翻条件：实测显示已加载索引搜索不满足核心场景。
  影响：行为约束见 `req/ui.stmts.md` 的搜索范围语句。

DEC-022   status: proposed   updated: 2026-11-02
  选择：Watcher 只作为缓存失效提示，不作为变更日志；事件订阅的断线、重连与会话过期恢复行为按目标服务端版本实测后定案。
  依据：`DESIGN-005`；经典 Watcher 为一次性通知。
  推翻条件：客户端库提供可验证的持久订阅语义并通过 Spike。
  影响：无 REQ 变更。
