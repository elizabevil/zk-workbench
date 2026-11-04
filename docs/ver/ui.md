---
doc_id: VER-UI
title: 工作台与树浏览验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 工作台与树浏览验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-UI-HISTORY-001、REQ-UI-LAYOUT-001、REQ-UI-LAYOUT-002、REQ-UI-NAV-001、REQ-UI-STATE-001、REQ-UI-STATE-002、REQ-UI-TREE-001、REQ-UI-TREE-002、REQ-UI-TREE-003、REQ-UI-TREE-004、REQ-UI-TREE-005

VER-UI-TREE-001   status: pending
  对应：REQ-UI-TREE-001
  步骤：执行并观察「展开一个节点时只读取其直接子节点；普通树浏览不得递归加载未展开的分支。」所述行为，记录实际结果。
  结果：未执行

VER-UI-TREE-002   status: pending
  对应：REQ-UI-TREE-002
  步骤：执行并观察「同一连接、同一路径在缓存有效期内重复展开可以复用结果；用户必须能刷新当前节点。」所述行为，记录实际结果。
  结果：未执行

VER-UI-TREE-003   status: pending
  对应：REQ-UI-TREE-003
  步骤：执行并观察「重连、{{generation}} 变化或收到相关子节点事件时，对应缓存条目必须失效。」所述行为，记录实际结果。
  结果：未执行

VER-UI-TREE-004   status: pending
  对应：REQ-UI-TREE-004
  步骤：执行并观察「路径直达必须按输入路径查询并读取目标节点，不得要求用户先在树上逐级展开。」所述行为，记录实际结果。
  结果：未执行

VER-UI-TREE-005   status: pending
  对应：REQ-UI-TREE-005
  步骤：执行并观察「名称或路径搜索只能覆盖当前已加载的路径索引，界面必须说明搜索范围，不得把局部结果表述为全局结果。」所述行为，记录实际结果。
  结果：未执行

VER-UI-STATE-001   status: pending
  对应：REQ-UI-STATE-001
  步骤：执行并观察「对象树必须区分未加载、加载中、空节点、错误、可能过期五种状态。」所述行为，记录实际结果。
  结果：未执行

VER-UI-STATE-002   status: pending
  对应：REQ-UI-STATE-002
  步骤：执行并观察「主工作区必须区分原始视图与格式视图，并显示未保存、冲突、保存成功、保存失败状态。」所述行为，记录实际结果。
  结果：未执行

VER-UI-LAYOUT-001   status: pending
  对应：REQ-UI-LAYOUT-001
  步骤：执行并观察「工作台必须提供连接切换、对象树、数据工作区、检查器、命令区与时间线六个区域，并允许调整面板宽度与记住布局。」所述行为，记录实际结果。
  结果：未执行

VER-UI-LAYOUT-002   status: pending
  对应：REQ-UI-LAYOUT-002
  步骤：执行并观察「所有写操作确认界面必须再次展示当前连接与认证身份。」所述行为，记录实际结果。
  结果：未执行

VER-UI-NAV-001   status: pending
  对应：REQ-UI-NAV-001
  步骤：执行并观察「查询请求必须可取消；过期响应不得覆盖用户的新选择。」所述行为，记录实际结果。
  结果：未执行

VER-UI-HISTORY-001   status: pending
  对应：REQ-UI-HISTORY-001
  步骤：执行若干命令后打开历史面板，确认列出本会话命令并可选择重跑。
  结果：未执行

VER-UI-PERF-001   status: pending
  对应：REQ-UI-PERF-001
  步骤：在目标机器上测量冷启动到可交互时间与稳定态常驻内存，并与同等空壳桌面应用基线对比。
  结果：未执行

VER-UI-TABS-001   status: pending
  对应：REQ-UI-TABS-001
  步骤：打开 4 个连接的标签集合后切回最早的连接，确认其标签已被移除；在有未保存标签的连接上验证移除前会出现提示。
  结果：未执行

VER-UI-INTL-001   status: pending
  对应：REQ-UI-INTL-001
  步骤：切换中英文，检查主界面、原生菜单、错误提示与命令结果文案；重启后确认语言恢复。
  结果：未执行

VER-UI-SETTINGS-001   status: pending
  对应：REQ-UI-SETTINGS-001
  步骤：打开设置页逐项检查九个分区；修改若干项后恢复默认并确认生效。
  结果：未执行

VER-UI-SHELL-001   status: pending
  对应：REQ-UI-SHELL-001
  步骤：启动应用确认进入主页；新建连接成功后确认进入工作区；检查左上角连接信息、切换与断开。
  结果：未执行
