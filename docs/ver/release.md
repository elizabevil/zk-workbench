---
doc_id: VER-RELEASE
title: 首版发布判据用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 首版发布判据用例

规则：这是首版出口判据，全部为 `pending`；只有全部 `passed` 且无未处置的 `failed`，才可宣称首版闭环达成。

CYCLE: 2026-11-02
范围：REQ-CONN-GEN-002、REQ-CONN-WORKSPACE-001、REQ-DATA-EDIT-002、REQ-UI-TREE-004

VER-RELEASE-GATE-001 status: pending
对应：REQ-CONN-WORKSPACE-001
步骤：新建连接、展开若干节点、关闭并重启应用，确认连接配置与浏览现场恢复。
结果：未执行

VER-RELEASE-GATE-002 status: pending
对应：REQ-UI-TREE-004
步骤：输入绝对路径直达节点，确认无需逐级展开即可读取数据与 Stat。
结果：未执行

VER-RELEASE-GATE-003 status: pending
对应：REQ-DATA-EDIT-002
步骤：修改数据后保存，确认展示 diff 并使用读取时版本写入成功。
结果：未执行

VER-RELEASE-GATE-004 status: pending
对应：REQ-DATA-EDIT-002
步骤：用另一个客户端先修改同一节点，再保存，确认冲突被拒绝且提示可重新读取。
结果：未执行

VER-RELEASE-GATE-005 status: pending
对应：REQ-CONN-GEN-002
步骤：断开并重连后查看已展开子树，确认旧数据被标记为可能过期而非显示为新鲜。
结果：未执行
