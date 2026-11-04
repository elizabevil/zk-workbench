---
doc_id: REQ-CONN
title: 连接与会话需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 连接与会话需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`01-product/prd.md`、`99-archive/03-specs-tree-query-connections.md`、`02-design/tech-design.md`、`05-changes/CHG-0004`。

REQ-CONN-SESSION-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：每个逻辑连接必须有独立的 {{session}}、状态、{{generation}}、{{watcher}}、缓存命名空间与审计上下文；一个连接的重连或失效不得影响另一个连接。
  标签：#invariant #constraint
  验证：VER-CONN-SESSION-001
  来源：SPEC-CONN-001；PRD-014

REQ-CONN-SESSION-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：同一逻辑连接下的面板与操作必须复用该连接的长期客户端，不得按单次操作新建 {{session}}。
  标签：#constraint
  验证：VER-CONN-SESSION-002
  来源：SPEC-CONN-002；DESIGN-001

REQ-CONN-STATE-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：连接状态必须至少区分创建中、连接中、已连接、断开或重连中、会话过期、关闭六种。
  标签：#capability #ux
  验证：VER-CONN-STATE-001
  来源：SPEC-CONN-003

REQ-CONN-STATE-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：界面必须分别表达可达性、{{session}} 状态、认证状态与数据是否 {{stale}}，不得用单一「连接成功」替代。
  标签：#invariant #ux
  验证：VER-CONN-STATE-002
  来源：PRD-005

REQ-CONN-GEN-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：{{session}} 重建或从断开恢复时必须递增 {{generation}}；携带旧 {{generation}} 的增量操作必须被拒绝，并要求刷新相关子树。
  标签：#invariant #error
  验证：VER-CONN-GEN-001
  来源：DESIGN-002

REQ-CONN-GEN-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：旧 {{generation}} 的事件不得更新当前界面；重连后不得把旧缓存呈现为已确认的新鲜数据。
  标签：#invariant #error
  验证：VER-CONN-GEN-002
  来源：PRD-005；DESIGN-005

REQ-CONN-WORKSPACE-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：用户必须能保存多个连接配置，并在本地保存各连接的浏览路径、树展开状态与数据格式偏好；切换或重启应用后可恢复，工作区状态不得写入 ZooKeeper。
  标签：#capability
  验证：VER-CONN-WORKSPACE-001
  来源：PRD-001

REQ-CONN-WORKSPACE-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：首版同时只激活一个工作区连接；多连接并列与后台会话保活属于后续阶段。
  标签：#constraint
  验证：VER-CONN-WORKSPACE-002
  来源：CHG-0004（B）

REQ-CONN-AUTH-001   status: superseded   since: 2026-11-02   owner: 产品负责人   superseded-by: REQ-CONN-AUTH-006
  语句：首版必须支持连接级认证配置；不同身份必须对应独立 {{session}}，不得在同一 {{session}} 上静默叠加不同身份。
  标签：#constraint #security
  验证：VER-CONN-AUTH-001
  来源：PRD-015；CHG-0004（D）

REQ-CONN-AUTH-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：路径前缀到认证身份的绑定属于第二版，首版不得提供路径级身份自动切换。
  标签：#constraint
  验证：VER-CONN-AUTH-002
  来源：PRD-015

REQ-CONN-ERROR-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：认证失败或权限不足时必须显示当前身份与可分类错误，不得静默轮试其他凭据。
  标签：#error #security
  验证：VER-CONN-ERROR-001
  来源：CHG-0004（D）

REQ-CONN-CLOSE-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：关闭连接时必须清理其 {{watcher}}、任务与客户端资源；应用退出时必须有序关闭全部连接。
  标签：#constraint
  验证：VER-CONN-CLOSE-001
  来源：SPEC-CONN-003

REQ-CONN-AUTH-003   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：切换路径导致实际使用的认证身份变化时，界面必须显式提示，不得静默切换身份。
  标签：#security #ux #deferred
  验证：VER-CONN-AUTH-003
  来源：02-design/ui-design.md；CHG-0010（第二版生效）

REQ-CONN-SSH-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：SSH Tunnel 属于第二阶段能力，首版不得提供。
  标签：#constraint #deferred
  验证：VER-CONN-SSH-001
  来源：CHG-0011

REQ-CONN-AUTH-006   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：连接级认证身份必须各自使用独立 {{session}}；命令 addauth 允许显式向当前 {{session}} 添加认证，但必须先确认并显示身份变化，不得静默叠加。
  标签：#constraint #security #confirm
  验证：VER-CONN-AUTH-006
  来源：CHG-0013

REQ-CONN-COMPAT-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：必须支持连接 ZooKeeper 3.4 及以上；命令与 flag 的可用性由服务端版本决定，不支持时必须明确报错，不得静默降级或静默忽略参数。
  标签：#capability #constraint #error
  验证：VER-CONN-COMPAT-001
  来源：CHG-0014

REQ-CONN-PROXY-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：首版必须支持 SOCKS5 与 HTTP CONNECT 代理；代理只作用于连接建立，不改变 ZooKeeper 操作语义。
  标签：#capability
  验证：VER-CONN-PROXY-001
  来源：CHG-0026

REQ-CONN-PROXY-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：代理不可用或握手失败必须返回可分类错误并注明代理阶段，不得误报为集群故障；单个连接的代理失败不得影响其他连接。
  标签：#error #invariant
  验证：VER-CONN-PROXY-002
  来源：CHG-0026

REQ-CONN-PROXY-003   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：代理凭据必须遵循四档保存范围，不得以明文写入配置文件或日志。
  标签：#security #constraint
  验证：VER-CONN-PROXY-003
  来源：CHG-0026

REQ-CONN-PROXY-004   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：代理档案必须支持多条，并按配置顺序依次尝试；某条不可用时必须尝试下一条，全部失败才报错。
  标签：#capability #error
  验证：VER-CONN-PROXY-004
  来源：CHG-0027

REQ-CONN-PROXY-005   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：必须支持 no_proxy 绕过列表，命中列表的主机不得经过代理；匹配至少支持精确主机与域名后缀。
  标签：#capability #constraint
  验证：VER-CONN-PROXY-005
  来源：CHG-0027

REQ-CONN-RETRY-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：自动重连必须默认开启，最多尝试 5 次，重试间隔从 3 秒递增至上限 10 秒；这些参数必须可在设置中修改。
  标签：#capability #ux
  验证：VER-CONN-RETRY-001
  来源：CHG-0027
