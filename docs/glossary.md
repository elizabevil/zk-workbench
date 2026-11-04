---
doc_id: GLOSSARY
title: 术语表
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 术语表

术语全库只定义一次。语句中以 `{{名字}}` 引用，例如 `删除 {{subtree}}`。

格式：`TERM <名字> = <一句定义>`。名字只允许小写字母、数字、点、下划线、连字符。

TERM subtree = 选中节点及其全部后代
TERM znode = ZooKeeper 树中的一个节点，包含路径、数据和 Stat
TERM stat = 节点元信息，含版本号 czxid、mzxid、version、cversion 等
TERM generation = 连接会话世代；会话重建或从断开恢复时递增
TERM session = 客户端与 ZooKeeper 集群之间的有状态会话
TERM cas = 基于读取时版本的写入校验；版本不一致时拒绝写入
TERM stale = 缓存或界面数据可能已过期、不得呈现为已确认的最新状态
TERM watcher = ZooKeeper 的一次性变化通知机制
TERM acl = 节点访问控制列表
TERM four-letter-command = ZooKeeper 服务端的运维查询命令
TERM preview = 执行前展示的影响范围概览，不构成原子快照
TERM node-type = 持久、临时、顺序、临时顺序四类节点及其合法组合
TERM workspace = 多连接、树、编辑器与命令区组成的桌面工作区
TERM danger-operation = 删除、递归、批量写入等可能造成数据损失的操作
TERM failure-report = 区分已完成、失败、未执行的分项结果报告
