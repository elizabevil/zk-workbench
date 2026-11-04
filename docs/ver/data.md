---
doc_id: VER-DATA
title: 数据格式与读写验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 数据格式与读写验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-DATA-BYTES-001、REQ-DATA-BYTES-002、REQ-DATA-EDIT-001、REQ-DATA-EDIT-002、REQ-DATA-EDIT-003、REQ-DATA-EDIT-004、REQ-DATA-EDIT-005、REQ-DATA-EXPORT-001、REQ-DATA-LARGE-001、REQ-DATA-LARGE-002、REQ-DATA-VIEW-001、REQ-DATA-VIEW-002

VER-DATA-VIEW-001 status: pending
对应：REQ-DATA-VIEW-001
步骤：执行并观察「查看模式必须支持原始字节与格式化视图切换；格式化只改变展示，不得修改原始字节或触发写入。」所述行为，记录实际结果。
结果：未执行

VER-DATA-VIEW-002 status: pending
对应：REQ-DATA-VIEW-002
步骤：执行并观察「JSON 与 YAML 必须支持语法高亮与格式化，查看模式可折叠与展开。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EDIT-001 status: pending
对应：REQ-DATA-EDIT-001
步骤：执行并观察「编辑必须使用独立缓冲并显示未保存状态；只有用户明确保存才写入 ZooKeeper。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EDIT-002 status: pending
对应：REQ-DATA-EDIT-002
步骤：执行并观察「保存前必须展示 diff 并使用读取时 version 做 {{cas}}；冲突时不得自动覆盖或盲目重试。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EDIT-003 status: pending
对应：REQ-DATA-EDIT-003
步骤：执行并观察「编辑器必须支持当前文档内查找、上/下一个、匹配计数、替换当前项与替换全部。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EDIT-004 status: pending
对应：REQ-DATA-EDIT-004
步骤：执行并观察「替换只作用于本地草稿；替换全部前必须显示匹配数并提供撤销。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EDIT-005 status: pending
对应：REQ-DATA-EDIT-005
步骤：执行并观察「格式校验失败时必须显示错误位置，不得丢弃或覆盖原始内容。」所述行为，记录实际结果。
结果：未执行

VER-DATA-BYTES-001 status: pending
对应：REQ-DATA-BYTES-001
步骤：执行并观察「数据无法安全解码为文本时必须显示字节长度并提供只读十六进制预览。」所述行为，记录实际结果。
结果：未执行

VER-DATA-LARGE-001 status: pending
对应：REQ-DATA-LARGE-001
步骤：执行并观察「大节点必须默认显示长度与预览，用户按需加载完整内容。」所述行为，记录实际结果。
结果：未执行

VER-DATA-EXPORT-001 status: pending
对应：REQ-DATA-EXPORT-001
步骤：执行并观察「导入导出连接配置与约定支持的数据时，不得导出明文凭据。」所述行为，记录实际结果。
结果：未执行

VER-DATA-BYTES-002 status: pending
对应：REQ-DATA-BYTES-002
步骤：对不可解码节点确认不存在内联编辑入口；通过完整替换流程写入新字节，检查字节级差异展示与版本冲突提示。
结果：未执行

VER-DATA-LARGE-002 status: pending
对应：REQ-DATA-LARGE-002
步骤：读取 4 KiB、100 KiB、2 MiB 三种节点，确认预览长度、总长度与显式加载提示一致。
结果：未执行
