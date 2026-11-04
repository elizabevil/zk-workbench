---
doc_id: REQ-NODE
title: 节点域需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 节点域需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`99-archive/03-specs-tree-query-connections.md` 的 `SPEC-NODE-001/002` 与 `05-changes/CHG-0006` 的 D1/C1。

## 创建

REQ-NODE-CREATE-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：创建对话框必须包含路径、节点类型（持久、临时、顺序、临时顺序）、数据、是否创建缺失父路径四项。
标签：#capability #ux
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-001

REQ-NODE-CREATE-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：提交创建时数据必须按原始文本处理，不得自动做格式转换。
标签：#constraint
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-002

REQ-NODE-CREATE-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：父路径不存在时创建必须失败并返回可分类错误，不得静默创建父链。
标签：#constraint #error
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-003

REQ-NODE-CREATE-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：只有用户显式选择创建缺失父路径时才创建，且中间节点必须为持久节点。
标签：#constraint #danger
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-004

REQ-NODE-CREATE-005 status: accepted since: 2026-11-02 owner: 产品负责人
语句：创建临时节点时必须提示其随会话结束删除，且断线重连后可能已不存在。
标签：#capability #confirm
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-005

REQ-NODE-CREATE-006 status: accepted since: 2026-11-02 owner: 产品负责人
语句：创建顺序节点成功后必须定位到服务端返回的完整路径并高亮，结果区显示实际路径。
标签：#capability #ux
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-006

REQ-NODE-CREATE-007 status: accepted since: 2026-11-02 owner: 产品负责人
语句：创建结果必须返回实际创建的绝对路径；失败必须返回可分类错误（无权限、父节点不存在、节点已存在、会话断开）。
标签：#capability #error
来源：SPEC-NODE-001；CHG-0006（C1）
验证：VER-NODE-CREATE-007

## 删除

REQ-NODE-DEL-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：任何删除都必须由用户明确确认，确认界面必须展示操作类型、目标路径和影响范围。
标签：#invariant #danger #confirm
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-001

REQ-NODE-DEL-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：目标节点为空时，只允许执行「删除该节点」，不得提供递归选项。
标签：#constraint #danger
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-002

REQ-NODE-DEL-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：目标节点非空时，普通删除不得可用；用户必须显式勾选「删除 {{subtree}}」后才能确认执行。
标签：#constraint #danger #confirm
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-003

REQ-NODE-DEL-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：勾选后必须展示影响预览，列出前 100 条路径与后代总数；总数超过 5000 时只显示「≥5000」。
标签：#capability #preview #threshold
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-004

REQ-NODE-DEL-005 status: accepted since: 2026-11-02 owner: 产品负责人
语句：影响预览的遍历必须可取消，且不得阻塞界面。
标签：#capability #ux
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-005

REQ-NODE-DEL-006 status: accepted since: 2026-11-02 owner: 产品负责人
语句：执行结果必须满足 {{failure-report}}：区分已完成、失败（含路径与错误类别）与未执行，不得把部分失败呈现为整体成功。
标签：#invariant #error
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-006

REQ-NODE-DEL-007 status: accepted since: 2026-11-02 owner: 产品负责人
语句：必须提供「仅重试失败子树」，不得自动重试；重试前必须重新展示影响预览并再次确认。
标签：#capability #danger
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-007

REQ-NODE-DEL-008 status: accepted since: 2026-11-02 owner: 产品负责人
语句：不得提供「删除父节点下指定子节点」的批量选择模式；删除范围始终是 {{subtree}}。
标签：#constraint #danger
来源：SPEC-NODE-002；CHG-0006（D1）
验证：VER-NODE-DEL-008
