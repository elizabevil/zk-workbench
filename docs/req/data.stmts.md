---
doc_id: REQ-DATA
title: 数据格式与读写需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 数据格式与读写需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`01-product/prd.md`、`99-archive/03-specs-tree-query-connections.md`、`02-design/ui-design.md`。

REQ-DATA-VIEW-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：查看模式必须支持原始字节与格式化视图切换；格式化只改变展示，不得修改原始字节或触发写入。
标签：#invariant
验证：VER-DATA-VIEW-001
来源：PRD-011

REQ-DATA-VIEW-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：JSON 与 YAML 必须支持语法高亮与格式化，查看模式可折叠与展开。
标签：#capability #ux
验证：VER-DATA-VIEW-002
来源：PRD-011

REQ-DATA-EDIT-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：编辑必须使用独立缓冲并显示未保存状态；只有用户明确保存才写入 ZooKeeper。
标签：#invariant
验证：VER-DATA-EDIT-001
来源：PRD-011

REQ-DATA-EDIT-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：保存前必须展示 diff 并使用读取时 version 做 {{cas}}；冲突时不得自动覆盖或盲目重试。
标签：#invariant #danger
验证：VER-DATA-EDIT-002
来源：PRD-004

REQ-DATA-EDIT-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：编辑器必须支持当前文档内查找、上/下一个、匹配计数、替换当前项与替换全部。
标签：#capability #ux
验证：VER-DATA-EDIT-003
来源：PRD-011

REQ-DATA-EDIT-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：替换只作用于本地草稿；替换全部前必须显示匹配数并提供撤销。
标签：#constraint
验证：VER-DATA-EDIT-004
来源：SPEC

REQ-DATA-EDIT-005 status: accepted since: 2026-11-02 owner: 产品负责人
语句：格式校验失败时必须显示错误位置，不得丢弃或覆盖原始内容。
标签：#error
验证：VER-DATA-EDIT-005
来源：SPEC

REQ-DATA-BYTES-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：数据无法安全解码为文本时必须显示字节长度并提供只读十六进制预览。
标签：#capability #error
验证：VER-DATA-BYTES-001
来源：SPEC

REQ-DATA-BYTES-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：不可解码数据不得提供逐字节内联编辑；编辑只允许通过显式的完整替换流程（文件或十六进制输入），保存前必须展示字节级差异与版本冲突状态。
标签：#danger #preview
验证：VER-DATA-BYTES-002
来源：SPEC；CHG-0009

REQ-DATA-LARGE-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：大节点必须默认显示长度与预览，用户按需加载完整内容。
标签：#capability #performance
验证：VER-DATA-LARGE-001
来源：PRD-008

REQ-DATA-EXPORT-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：导入导出连接配置与约定支持的数据时，不得导出明文凭据。
标签：#security #constraint
验证：VER-DATA-EXPORT-001
来源：PRD-008

REQ-DATA-LARGE-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：大节点必须默认展示前 4 KiB 与总长度；完整加载必须由用户显式触发；数据超过 1 MiB 时必须提示加载代价。以上数值为实验值，实测后可调整。
标签：#capability #performance #threshold
验证：VER-DATA-LARGE-002
来源：CHG-0011
