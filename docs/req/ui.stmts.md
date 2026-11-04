---
doc_id: REQ-UI
title: 工作台与树浏览需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 工作台与树浏览需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`99-archive/03-specs-tree-query-connections.md`、`02-design/ui-design.md`、`05-changes/CHG-0004`。

REQ-UI-TREE-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：展开一个节点时只读取其直接子节点；普通树浏览不得递归加载未展开的分支。
  标签：#capability #performance
  验证：VER-UI-TREE-001
  来源：SPEC-TREE-001；PRD-002

REQ-UI-TREE-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：同一连接、同一路径在缓存有效期内重复展开可以复用结果；用户必须能刷新当前节点。
  标签：#capability
  验证：VER-UI-TREE-002
  来源：SPEC-TREE-002

REQ-UI-TREE-003   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：重连、{{generation}} 变化或收到相关子节点事件时，对应缓存条目必须失效。
  标签：#invariant #error
  验证：VER-UI-TREE-003
  来源：SPEC-TREE-002

REQ-UI-TREE-004   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：路径直达必须按输入路径查询并读取目标节点，不得要求用户先在树上逐级展开。
  标签：#capability
  验证：VER-UI-TREE-004
  来源：SPEC-TREE-003

REQ-UI-TREE-005   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：名称或路径搜索只能覆盖当前已加载的路径索引，界面必须说明搜索范围，不得把局部结果表述为全局结果。
  标签：#capability #ux
  验证：VER-UI-TREE-005
  来源：SPEC-TREE-004

REQ-UI-STATE-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：对象树必须区分未加载、加载中、空节点、错误、可能过期五种状态。
  标签：#ux #invariant
  验证：VER-UI-STATE-001
  来源：02-design/ui-design.md

REQ-UI-STATE-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：主工作区必须区分原始视图与格式视图，并显示未保存、冲突、保存成功、保存失败状态。
  标签：#ux
  验证：VER-UI-STATE-002
  来源：02-design/ui-design.md

REQ-UI-LAYOUT-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：工作台必须提供连接切换、对象树、数据工作区、检查器、命令区与时间线六个区域，并允许调整面板宽度与记住布局。
  标签：#capability #ux
  验证：VER-UI-LAYOUT-001
  来源：CHG-0004（F）

REQ-UI-LAYOUT-002   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：所有写操作确认界面必须再次展示当前连接与认证身份。
  标签：#invariant #confirm
  验证：VER-UI-LAYOUT-002
  来源：02-design/ui-design.md

REQ-UI-NAV-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：查询请求必须可取消；过期响应不得覆盖用户的新选择。
  标签：#constraint #performance
  验证：VER-UI-NAV-001
  来源：02-design/ui-design.md

REQ-UI-HISTORY-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：首版必须提供命令历史面板，保留本次会话内已执行的命令，并支持选择后重跑。
  标签：#capability #ux
  验证：VER-UI-HISTORY-001
  来源：CHG-0015

REQ-UI-PERF-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：冷启动到可交互不得超过 2 秒；常驻内存不得超过 100 MB，并尽量控制在同等空壳桌面应用基线的两倍以内。
  标签：#performance #threshold
  验证：VER-UI-PERF-001
  来源：CHG-0017

REQ-UI-TABS-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：切换活动连接不得关闭已打开的数据标签；每个连接的标签集合必须独立保留，默认仅保留最近 3 个连接的标签；被移除连接的标签中存在未保存内容时，必须先提示用户处理。
  标签：#capability #invariant #confirm
  验证：VER-UI-TABS-001
  来源：CHG-0018

REQ-UI-INTL-001   status: accepted   since: 2026-11-02   owner: 产品负责人
  语句：首版必须支持中文与英文切换；界面文案、菜单与错误提示必须来自语言包，不得硬编码；语言偏好必须本地保存并在重启后恢复。
  标签：#capability #ux
  验证：VER-UI-INTL-001
  来源：CHG-0019
