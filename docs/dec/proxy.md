---
doc_id: DEC-PROXY
title: 代理与传输层决策
status: accepted
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 代理与传输层决策

DEC-070   status: accepted   updated: 2026-11-02
  选择：首版支持 SOCKS5 与 HTTP CONNECT 代理；实现方式是 core 内的传输抽象 + 本地中继（`127.0.0.1` 临时端口转发到代理上游），ZooKeeper 客户端仍连本地中继；SSH Tunnel 维持第二阶段（`REQ-CONN-SSH-001`）。
  依据：负责人确认；中继方式对 `zookeeper-client` 零侵入，后续换 SSH 通道只需替换传输实现。
  推翻条件：客户端库提供可注入的自定义连接器且实测稳定。
  影响：新增 `REQ-CONN-PROXY-001..003`；连接生命周期需管理本地监听端口与转发任务。

DEC-071   status: proposed   updated: 2026-11-02
  选择：代理档案支持多条并按配置顺序依次尝试，全部失败才报错；代理凭据与连接凭据共用四档保存策略（永不保存 / for session / until restart / forever），凭据写入系统凭据存储；no_proxy 绕过列表首版提供，支持精确主机与域名后缀。
  依据：`REQ-AUTH-CRED-003/004` 已确认；避免两套凭据模型。
  推翻条件：代理需要与集群凭据不同的生命周期。
  影响：代理配置界面复用凭据组件。

DEC-072   status: accepted   updated: 2026-11-02
  选择：自动重连默认开启，最多 5 次，间隔从 3 秒递增至上限 10 秒（倍率 2、带抖动），参数在设置页可修改；代理层失败与 ZooKeeper 会话失败分开计数与展示。
  依据：负责人确认；与 `REQ-CONN-RETRY-001` 一致。
  推翻条件：实测显示重试策略导致会话风暴或延长故障感知。
  影响：连接管理器暴露重试状态与剩余次数给前端。
