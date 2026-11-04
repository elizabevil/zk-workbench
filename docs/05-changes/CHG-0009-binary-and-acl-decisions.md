---
doc_id: CHG-0009
title: 二进制数据替换与 ACL 只读决议
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0009：二进制数据替换与 ACL 只读决议

## 目的

收敛 `CHG-0008` 遗留的三条 `proposed` 语句中的两条：不可解码字节的编辑方式、首版 ACL 展示范围。

## 决策来源

产品负责人确认：

- 不可解码字节按十六进制显示；这类数据 **不提供**逐字节内联编辑与直接保存覆盖（文本内容可以内联编辑，二进制不行），编辑必须走完整替换流程。
- 首版 ACL **只读展示**。

两份决定的语义分别落到 `REQ-DATA-BYTES-002` 与 `REQ-AUTH-ACL-002`，状态由 `proposed` 改为 `accepted`。

## 决策内容

### 二进制与不可解码数据

1. `REQ-DATA-BYTES-001`（既有）：无法安全解码为文本时显示字节长度并提供只读十六进制预览。
2. `REQ-DATA-BYTES-002`（本次确认）：不得提供逐字节内联编辑；编辑只允许通过显式的完整替换流程（文件或十六进制输入），保存前展示字节级差异与版本冲突状态。
3. 文本内容仍可内联编辑，但其保存同样受 `REQ-DATA-EDIT-001`、`REQ-DATA-EDIT-002` 约束（独立缓冲、diff、CAS）。

### ACL 展示

1. `REQ-AUTH-ACL-001`（既有）：ACL 编辑后置，首版不得提供 ACL 编辑。
2. `REQ-AUTH-ACL-002`（本次确认）：首版只读显示 ACL；权限不足导致不可见必须与 ACL 内容为空区分。
3. `REQ-AUTH-ACL-003`（既有）：权限不足与连接故障、节点不存在必须区分显示。

## 影响文件

- `req/data.stmts.md`：`REQ-DATA-BYTES-002` 改状态与措辞，补 `验证：` 与 `来源：`。
- `req/auth.stmts.md`：`REQ-AUTH-ACL-002` 改状态与措辞，补 `验证：` 与 `来源：`。
- `ver/data.md`、`ver/auth.md`：各补一条 `pending` 用例，更新范围行。
- `locks.yml`：登记两条 accepted 语句的正文 hash。

## 非目标（Non-Goals）

1. 不改变 `REQ-DATA-BYTES-001` 的只读十六进制预览行为。
2. 不把 ACL 编辑纳入首版或第二版承诺。
3. 不定义完整替换流程的具体 UI 形态（对话框、文件选择器）。
4. 不改变文本编辑的既有约束。
5. 不处理 `REQ-CONN-AUTH-003`（身份变化提示仍为 `proposed`）。

## 候选方案与权衡（Alternatives & Trade-offs）

**允许二进制逐字节内联编辑**：需要处理任意字节编码、光标位置与编辑缓冲的字节对齐，且误改风险高；被拒绝。

**首版完全不展示 ACL**：无法解释权限错误，用户难以区分「没权限」与「节点不存在」；被拒绝，改为只读展示并区分不可见与为空。

## 遗留

- `REQ-CONN-AUTH-003` 已由 `CHG-0010` 转为 accepted（`#deferred`，第二版生效）。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
