---
doc_id: CHG-0010
title: 路径导致身份变化的提示决议
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0010：路径导致身份变化的提示决议

## 目的

收敛 `REQ-CONN-AUTH-003`（切换路径导致认证身份变化时是否必须显式提示）。

## 决策

产品负责人确认：接受为语句，标记 `#deferred`，第二版生效。

- 语句状态：`proposed` → `accepted`。
- 生效范围：路径前缀到认证身份的绑定属于第二版（`REQ-CONN-AUTH-002`），首版只有连接级认证，不会出现路径触发的身份变化。
- 行为：身份变化必须显式提示，不得静默切换。

## 理由

1. 身份决定权限上下文，静默切换会让用户误判当前能做什么、正在修改哪个权限域。
2. 与既有约束同族：`REQ-UI-LAYOUT-002`（写操作确认展示身份）、`REQ-CLI-RESULT-003`（切身份隔离结果）。
3. 标记 `#deferred` 不影响首版实施，但保证第二版设计不遗漏。

## 影响文件

- `req/conn.stmts.md`：`REQ-CONN-AUTH-003` 改状态、补 `验证：` 与 `来源：`。
- `ver/conn.md`：补一条 `pending` 用例并更新范围行。
- `locks.yml`：登记新 accepted 语句 hash。

## 非目标（Non-Goals）

1. 不在首版实现路径级身份绑定。
2. 不定义提示的具体视觉形态（横幅、角标、状态栏）。
3. 不改变连接级认证的既有约束。
4. 不涉及 ACL 展示（已由 `CHG-0009` 定为首版只读）。
5. 不定义身份切换后查询结果的清理细节（见 `REQ-CLI-RESULT-003`）。

## 候选方案与权衡（Alternatives & Trade-offs）

**保持 `proposed` 等第二版设计时再定**：可能到实现时才发现缺失，且与既有身份类约束不同步；被拒绝。

**拒绝该约束**：第二版路径绑定存在静默切换风险，用户可能用错身份写数据；被拒绝。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
