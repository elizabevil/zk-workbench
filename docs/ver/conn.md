---
doc_id: VER-CONN
title: 连接与会话验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 连接与会话验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-CONN-AUTH-002、REQ-CONN-AUTH-003、REQ-CONN-AUTH-006、REQ-CONN-CLOSE-001、REQ-CONN-COMPAT-001、REQ-CONN-ERROR-001、REQ-CONN-GEN-001、REQ-CONN-GEN-002、REQ-CONN-SESSION-001、REQ-CONN-SESSION-002、REQ-CONN-SSH-001、REQ-CONN-STATE-001、REQ-CONN-STATE-002、REQ-CONN-WORKSPACE-001、REQ-CONN-WORKSPACE-002

VER-CONN-SESSION-001 status: pending
对应：REQ-CONN-SESSION-001
步骤：执行并观察「每个逻辑连接必须有独立的 {{session}}、状态、{{generation}}、{{watcher}}、缓存命名空…」所述行为，记录实际结果。
结果：未执行

VER-CONN-SESSION-002 status: pending
对应：REQ-CONN-SESSION-002
步骤：执行并观察「同一逻辑连接下的面板与操作必须复用该连接的长期客户端，不得按单次操作新建 {{session}}。」所述行为，记录实际结果。
结果：未执行

VER-CONN-STATE-001 status: pending
对应：REQ-CONN-STATE-001
步骤：执行并观察「连接状态必须至少区分创建中、连接中、已连接、断开或重连中、会话过期、关闭六种。」所述行为，记录实际结果。
结果：未执行

VER-CONN-STATE-002 status: pending
对应：REQ-CONN-STATE-002
步骤：执行并观察「界面必须分别表达可达性、{{session}} 状态、认证状态与数据是否 {{stale}}，不得用单一「连接成功」替代…」所述行为，记录实际结果。
结果：未执行

VER-CONN-GEN-001 status: pending
对应：REQ-CONN-GEN-001
步骤：执行并观察「{{session}} 重建或从断开恢复时必须递增 {{generation}}；携带旧 {{generation}} …」所述行为，记录实际结果。
结果：未执行

VER-CONN-GEN-002 status: pending
对应：REQ-CONN-GEN-002
步骤：执行并观察「旧 {{generation}} 的事件不得更新当前界面；重连后不得把旧缓存呈现为已确认的新鲜数据。」所述行为，记录实际结果。
结果：未执行

VER-CONN-WORKSPACE-001 status: pending
对应：REQ-CONN-WORKSPACE-001
步骤：执行并观察「用户必须能保存多个连接配置，并在本地保存各连接的浏览路径、树展开状态与数据格式偏好；切换或重启应用后可恢复，工作区状态不…」所述行为，记录实际结果。
结果：未执行

VER-CONN-WORKSPACE-002 status: pending
对应：REQ-CONN-WORKSPACE-002
步骤：执行并观察「首版同时只激活一个工作区连接；多连接并列与后台会话保活属于后续阶段。」所述行为，记录实际结果。
结果：未执行

VER-CONN-AUTH-002 status: pending
对应：REQ-CONN-AUTH-002
步骤：执行并观察「路径前缀到认证身份的绑定属于第二版，首版不得提供路径级身份自动切换。」所述行为，记录实际结果。
结果：未执行

VER-CONN-ERROR-001 status: pending
对应：REQ-CONN-ERROR-001
步骤：执行并观察「认证失败或权限不足时必须显示当前身份与可分类错误，不得静默轮试其他凭据。」所述行为，记录实际结果。
结果：未执行

VER-CONN-CLOSE-001 status: pending
对应：REQ-CONN-CLOSE-001
步骤：执行并观察「关闭连接时必须清理其 {{watcher}}、任务与客户端资源；应用退出时必须有序关闭全部连接。」所述行为，记录实际结果。
结果：未执行

VER-CONN-AUTH-003 status: pending
对应：REQ-CONN-AUTH-003
步骤：第二版实现后，配置路径级身份绑定，在同一连接内切换路径跨越身份边界，确认界面显式提示当前身份变化。
结果：未执行

VER-CONN-SSH-001 status: pending
对应：REQ-CONN-SSH-001
步骤：检查首版连接界面与命令，确认不存在 SSH Tunnel 入口。
结果：未执行

VER-CONN-AUTH-006 status: pending
对应：REQ-CONN-AUTH-006
步骤：在会话内执行 addauth，确认身份变化被展示且需确认；连接级身份仍使用独立 session。
结果：未执行

VER-CONN-COMPAT-001 status: pending
对应：REQ-CONN-COMPAT-001
步骤：分别连接 3.4、3.5、3.9 服务端，执行 getEphemerals、getAllChildrenNumber、config 等版本相关命令，确认不支持时返回明确错误而非静默忽略。
结果：未执行

VER-CONN-PROXY-001 status: pending
对应：REQ-CONN-PROXY-001
步骤：分别经 SOCKS5 与 HTTP CONNECT 连接 3.9 实例，确认可完成 get/set。
结果：未执行

VER-CONN-PROXY-002 status: pending
对应：REQ-CONN-PROXY-002
步骤：构造代理拒绝、握手超时与认证失败，确认错误分类正确且其他连接不受影响。
结果：未执行

VER-CONN-PROXY-003 status: pending
对应：REQ-CONN-PROXY-003
步骤：以四档范围保存代理凭据，检查配置与日志无明文。
结果：未执行

VER-CONN-PROXY-004 status: pending
对应：REQ-CONN-PROXY-004
步骤：配置两条代理，第一条不可用、第二条可用，确认按顺序回退成功；两条都不可用时错误包含两条的失败原因。
结果：未执行

VER-CONN-PROXY-005 status: pending
对应：REQ-CONN-PROXY-005
步骤：配置 no_proxy 精确主机与后缀，确认命中主机直连、未命中走代理。
结果：未执行

VER-CONN-RETRY-001 status: pending
对应：REQ-CONN-RETRY-001
步骤：断开网络观察重连：默认开启、最多 5 次、间隔 3s 递增至 10s；修改参数后行为随之变化。
结果：未执行

VER-CONN-PROFILE-001 status: pending
对应：REQ-CONN-PROFILE-001
步骤：新建档案并连接，重启应用确认档案仍在首页；删除时确认提示出现，取消后档案保留。
结果：未执行
