---
doc_id: VER-CLI
title: 命令行与脚本验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 命令行与脚本验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-CLI-COMPLETE-001、REQ-CLI-COMPLETE-002、REQ-CLI-DANGER-002、REQ-CLI-DELETEALL-001、REQ-CLI-EXT-002、REQ-CLI-FLAGS-003、REQ-CLI-GRAMMAR-001、REQ-CLI-PREVIEW-001、REQ-CLI-RESULT-001、REQ-CLI-RESULT-002、REQ-CLI-RESULT-003、REQ-CLI-RESULT-004、REQ-CLI-SAVE-001、REQ-CLI-SCOPE-006、REQ-CLI-SCOPE-007、REQ-CLI-SCOPE-008、REQ-CLI-SCRIPT-001、REQ-CLI-SCRIPT-002、REQ-CLI-SCRIPT-003、REQ-CLI-SCRIPT-004、REQ-CLI-SCRIPT-005、REQ-CLI-SCRIPT-006

VER-CLI-COMPLETE-001 status: pending
对应：REQ-CLI-COMPLETE-001
步骤：执行并观察「命令输入必须提供自动补全，覆盖命令、子命令、参数与可安全枚举的路径子节点。」所述行为，记录实际结果。
结果：未执行

VER-CLI-COMPLETE-002 status: pending
对应：REQ-CLI-COMPLETE-002
步骤：执行并观察「新输入必须取消过时的补全请求；延迟响应不得覆盖当前候选，也不得造成请求无界堆积。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SAVE-001 status: pending
对应：REQ-CLI-SAVE-001
步骤：执行并观察「用户必须能保存单条常用命令与多行脚本文件，并从界面执行。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SCRIPT-001 status: pending
对应：REQ-CLI-SCRIPT-001
步骤：执行并观察「脚本必须按文件行顺序逐行执行，并提供执行全部与执行当前行两种操作；不得并行重排。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SCRIPT-002 status: pending
对应：REQ-CLI-SCRIPT-002
步骤：执行并观察「命令以换行或分号结束；引号内的分号不得作为结束符。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SCRIPT-003 status: pending
对应：REQ-CLI-SCRIPT-003
步骤：执行并观察「脚本遇到第一条失败命令必须立即停止，并返回失败命令与错误。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SCRIPT-004 status: pending
对应：REQ-CLI-SCRIPT-004
步骤：执行并观察「首版脚本只支持顺序命令、空行与注释，不支持变量、条件、循环或任意脚本语言执行。」所述行为，记录实际结果。
结果：未执行

VER-CLI-PREVIEW-001 status: pending
对应：REQ-CLI-PREVIEW-001
步骤：执行并观察「执行前必须展示目标连接与写操作摘要；摘要必须由待执行的同一解析结果产生。」所述行为，记录实际结果。
结果：未执行

VER-CLI-RESULT-001 status: pending
对应：REQ-CLI-RESULT-001
步骤：执行并观察「每条命令必须返回结果；失败必须返回明确错误，不得伪装成功或静默丢弃。」所述行为，记录实际结果。
结果：未执行

VER-CLI-RESULT-002 status: pending
对应：REQ-CLI-RESULT-002
步骤：执行并观察「脚本查询结果必须逐条保存并默认折叠，至少关联行号、命令文本、执行状态与结果或错误。」所述行为，记录实际结果。
结果：未执行

VER-CLI-RESULT-003 status: pending
对应：REQ-CLI-RESULT-003
步骤：执行并观察「查询结果仅在当前运行或当前会话内保留；切换连接或身份时必须清除或隔离结果。」所述行为，记录实际结果。
结果：未执行

VER-CLI-SCRIPT-005 status: pending
对应：REQ-CLI-SCRIPT-005
步骤：检查脚本默认目录、打开与保存流程，确认不存在通用文件管理，且脚本内容不含凭据字段。
结果：未执行

VER-CLI-RESULT-004 status: pending
对应：REQ-CLI-RESULT-004
步骤：执行脚本产生结果后重启应用，确认结果列表为空。
结果：未执行

VER-CLI-GRAMMAR-001 status: pending
对应：REQ-CLI-GRAMMAR-001
步骤：以 zkCli 形式执行 get /node、ls /、create -e /a b，确认位置参数与 flag 解析结果与官方行为一致。
结果：未执行

VER-CLI-DELETEALL-001 status: pending
对应：REQ-CLI-DELETEALL-001
步骤：对非空节点执行 deleteall，确认执行前展示目标路径与递归删除确认，且未统计后代数量。
结果：未执行

VER-CLI-SCRIPT-006 status: pending
对应：REQ-CLI-SCRIPT-006
步骤：脚本中插入 # 注释行与行尾注释，确认注释被忽略且命令解析结果不变。
结果：未执行

VER-CLI-EXT-002 status: pending
对应：REQ-CLI-EXT-002
步骤：检查命令清单与补全，确认不存在 :cmd 类命令；输入 :help 应明确报错。
结果：未执行

VER-CLI-SCOPE-006 status: pending
对应：REQ-CLI-SCOPE-006
步骤：逐条执行 17 条受支持命令；再执行 config、reconfig、setquota、printwatches、history，确认全部明确报错。
结果：未执行

VER-CLI-FLAGS-003 status: pending
对应：REQ-CLI-FLAGS-003
步骤：逐条执行受支持 flag；执行 get -x 等未知 flag 确认报错；执行 get -w 后修改节点，确认只在本次命令结果中提示且不建立长期订阅。
结果：未执行

VER-CLI-DANGER-002 status: pending
对应：REQ-CLI-DANGER-002
步骤：执行 setAcl 与 addauth，确认展示目标、参数、连接与身份并要求二次确认。
结果：未执行

VER-CLI-SCOPE-007 status: pending
对应：REQ-CLI-SCOPE-007
步骤：检查界面是否存在命令历史与重跑入口；执行 history、redo、printwatches、removewatches、config，确认全部报错。
结果：未执行

VER-CLI-SCOPE-008 status: pending
对应：REQ-CLI-SCOPE-008
步骤：执行 reconfig、setquota、delquota，确认明确报错并提示后续以界面形式评估。
结果：未执行
