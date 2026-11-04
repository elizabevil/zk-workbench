---
doc_id: VER-NODE
title: 节点域验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 节点域验证用例

规则：用例状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。
周期变更时，依赖变更 DEC 的用例标 `stale` 并重跑。

CYCLE: 2026-11-02
范围：REQ-NODE-CREATE-001、REQ-NODE-CREATE-002、REQ-NODE-CREATE-003、REQ-NODE-CREATE-004、REQ-NODE-CREATE-005、REQ-NODE-CREATE-006、REQ-NODE-CREATE-007、REQ-NODE-DEL-001、REQ-NODE-DEL-002、REQ-NODE-DEL-003、REQ-NODE-DEL-004、REQ-NODE-DEL-005、REQ-NODE-DEL-006、REQ-NODE-DEL-007、REQ-NODE-DEL-008

VER-NODE-CREATE-001   status: pending
  对应：REQ-NODE-CREATE-001
  步骤：打开创建对话框，逐项确认路径、节点类型、数据、创建缺失父路径四个控件存在。
  结果：未执行

VER-NODE-CREATE-002   status: pending
  对应：REQ-NODE-CREATE-002
  步骤：提交含缩进与空行的 JSON 文本，读取节点原始字节，确认与输入逐字节一致。
  结果：未执行

VER-NODE-CREATE-003   status: pending
  对应：REQ-NODE-CREATE-003
  步骤：在不存在父路径下创建子节点，确认返回错误且未创建任何节点。
  结果：未执行

VER-NODE-CREATE-004   status: pending
  对应：REQ-NODE-CREATE-004
  步骤：勾选创建缺失父路径后创建多级路径，读取各级节点 Stat，确认中间节点为持久类型。
  结果：未执行

VER-NODE-CREATE-005   status: pending
  对应：REQ-NODE-CREATE-005
  步骤：创建临时节点，确认提示文案存在；关闭会话后确认节点消失。
  结果：未执行

VER-NODE-CREATE-006   status: pending
  对应：REQ-NODE-CREATE-006
  步骤：创建顺序节点，确认输入前缀与服务端返回路径不同，且界面定位到实际路径。
  结果：未执行

VER-NODE-CREATE-007   status: pending
  对应：REQ-NODE-CREATE-007
  步骤：分别构造成功、无权限、已存在、会话断开四种情况，确认返回路径或可分类错误。
  结果：未执行

VER-NODE-DEL-001   status: pending
  对应：REQ-NODE-DEL-001
  步骤：对任意节点触发删除，确认出现确认步骤且展示操作类型、目标路径、影响范围。
  结果：未执行

VER-NODE-DEL-002   status: pending
  对应：REQ-NODE-DEL-002
  步骤：删除空节点，确认对话框中不存在递归选项且可一次确认完成。
  结果：未执行

VER-NODE-DEL-003   status: pending
  对应：REQ-NODE-DEL-003
  步骤：打开非空节点删除，确认普通删除不可用；勾选删除子树后确认按钮才可用。
  结果：未执行

VER-NODE-DEL-004   status: pending
  对应：REQ-NODE-DEL-004
  步骤：对 120 个后代与 6000 个后代的子树分别触发预览，确认路径条数上限与总数显示规则。
  结果：未执行

VER-NODE-DEL-005   status: pending
  对应：REQ-NODE-DEL-005
  步骤：对大树触发预览并在遍历中取消，确认界面可交互且取消后无残留加载状态。
  结果：未执行

VER-NODE-DEL-006   status: pending
  对应：REQ-NODE-DEL-006
  步骤：构造执行中途失败（删除部分后代后断开权限），确认结果分项展示已完成、失败、未执行。
  结果：未执行

VER-NODE-DEL-007   status: pending
  对应：REQ-NODE-DEL-007
  步骤：失败后点击仅重试失败子树，确认重新展示预览并再次要求确认，且不自动重试。
  结果：未执行

VER-NODE-DEL-008   status: pending
  对应：REQ-NODE-DEL-008
  步骤：检查删除界面，确认不存在「删除父节点下指定子节点」模式，删除范围固定为选中节点子树。
  结果：未执行
