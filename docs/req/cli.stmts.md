---
doc_id: REQ-CLI
title: 命令行与脚本需求语句
status: draft
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 命令行与脚本需求语句

只追加。`accepted` 条款正文不可修改；语义变更必须新开 ID 并让旧条款 `superseded`。
迁移来源：`01-product/prd.md`、`99-archive/03-specs-tree-query-connections.md`、`05-changes/CHG-0004`。

REQ-CLI-SCOPE-001 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-GRAMMAR-001
语句：命令只表达 ZooKeeper 连接、节点、ACL、数据与巡检操作，不得执行任意系统 shell 命令。
标签：#constraint #security
验证：VER-CLI-SCOPE-001
来源：PRD-012

REQ-CLI-SCOPE-002 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-GRAMMAR-001
语句：命令必须能复用当前连接与节点上下文，也必须能显式指定目标连接与路径。
标签：#capability
验证：VER-CLI-SCOPE-002
来源：SPEC

REQ-CLI-COMPLETE-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：命令输入必须提供自动补全，覆盖命令、子命令、参数与可安全枚举的路径子节点。
标签：#capability #ux
验证：VER-CLI-COMPLETE-001
来源：PRD-012

REQ-CLI-COMPLETE-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：新输入必须取消过时的补全请求；延迟响应不得覆盖当前候选，也不得造成请求无界堆积。
标签：#constraint #performance
验证：VER-CLI-COMPLETE-002
来源：SPEC

REQ-CLI-SAVE-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：用户必须能保存单条常用命令与多行脚本文件，并从界面执行。
标签：#capability
验证：VER-CLI-SAVE-001
来源：PRD-012

REQ-CLI-SCRIPT-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：脚本必须按文件行顺序逐行执行，并提供执行全部与执行当前行两种操作；不得并行重排。
标签：#invariant
验证：VER-CLI-SCRIPT-001
来源：PRD-013

REQ-CLI-SCRIPT-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：命令以换行或分号结束；引号内的分号不得作为结束符。
标签：#constraint
验证：VER-CLI-SCRIPT-002
来源：SPEC

REQ-CLI-SCRIPT-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：脚本遇到第一条失败命令必须立即停止，并返回失败命令与错误。
标签：#invariant #error
验证：VER-CLI-SCRIPT-003
来源：CHG-0004（E）；PRD-013

REQ-CLI-SCRIPT-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：首版脚本只支持顺序命令、空行与注释，不支持变量、条件、循环或任意脚本语言执行。
标签：#constraint #deferred
验证：VER-CLI-SCRIPT-004
来源：CHG-0004（E）

REQ-CLI-PREVIEW-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：执行前必须展示目标连接与写操作摘要；摘要必须由待执行的同一解析结果产生。
标签：#invariant #confirm
验证：VER-CLI-PREVIEW-001
来源：SPEC

REQ-CLI-RESULT-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：每条命令必须返回结果；失败必须返回明确错误，不得伪装成功或静默丢弃。
标签：#invariant #error
验证：VER-CLI-RESULT-001
来源：SPEC

REQ-CLI-RESULT-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：脚本查询结果必须逐条保存并默认折叠，至少关联行号、命令文本、执行状态与结果或错误。
标签：#capability #ux
验证：VER-CLI-RESULT-002
来源：PRD-013

REQ-CLI-RESULT-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：查询结果仅在当前运行或当前会话内保留；切换连接或身份时必须清除或隔离结果。
标签：#invariant #security
验证：VER-CLI-RESULT-003
来源：SPEC

REQ-CLI-SCRIPT-005 status: accepted since: 2026-11-02 owner: 产品负责人
语句：脚本必须保存在应用私有目录；用户可显式打开与保存普通文本文件；应用不提供通用文件管理能力；脚本不得内嵌凭据。
标签：#capability #security
验证：VER-CLI-SCRIPT-005
来源：CHG-0011

REQ-CLI-RESULT-004 status: accepted since: 2026-11-02 owner: 产品负责人
语句：查询结果不得跨应用重启保留，仅在当前运行或当前会话内有效。
标签：#invariant
验证：VER-CLI-RESULT-004
来源：CHG-0011

REQ-CLI-GRAMMAR-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：命令语法必须与 ZooKeeper 官方 zkCli 一致（位置参数与 flag 形式）；引号与转义只在脚本层补充，不改变 zkCli 语义。
标签：#constraint
验证：VER-CLI-GRAMMAR-001
来源：CHG-0012

REQ-CLI-SCOPE-003 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-GRAMMAR-002
语句：首版必须支持 ls、get、set、create、delete、deleteall、stat、getAcl、sync、help 十个命令；其余 zkCli 命令必须明确报错，不得静默忽略。
标签：#capability #constraint
验证：VER-CLI-SCOPE-003
来源：CHG-0012

REQ-CLI-SCOPE-004 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-FLAGS-002
语句：首版不得支持 -w 与 ls -R；未支持的 flag 必须明确报错，不得静默忽略。
标签：#constraint #error
验证：VER-CLI-SCOPE-004
来源：CHG-0012

REQ-CLI-FLAGS-001 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-FLAGS-002
语句：首版支持的 flag 为 get -s、create -e、create -s、create -c、create -t、set -v、delete -v。
标签：#capability #constraint
验证：VER-CLI-FLAGS-001
来源：CHG-0012

REQ-CLI-SCOPE-005 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-EXT-002
语句：首版不得提供应用扩展命令，命令集与官方 zkCli 保持一致。
标签：#constraint
验证：VER-CLI-SCOPE-005
来源：CHG-0012

REQ-CLI-DELETEALL-001 status: accepted since: 2026-11-02 owner: 产品负责人
语句：deleteall 执行前必须显示目标路径与递归删除确认；首版不在命令行统计后代数量。
标签：#danger #confirm
验证：VER-CLI-DELETEALL-001
来源：CHG-0012

REQ-CLI-SCRIPT-006 status: accepted since: 2026-11-02 owner: 产品负责人
语句：脚本中以 # 开头的行必须视为注释；注释不得改变命令解析结果。
标签：#capability
验证：VER-CLI-SCRIPT-006
来源：CHG-0012

REQ-CLI-GRAMMAR-002 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-SCOPE-006
语句：命令集与语法必须以 ZooKeeper 3.9 官方 CLI 文档为准；首版不提供 connect、quit、close（连接由界面管理），也不提供四字命令（不在该文档命令表内）。
标签：#capability #constraint
验证：VER-CLI-GRAMMAR-002
来源：CHG-0013；ZooKeeper 3.9 zookeeperCLI

REQ-CLI-FLAGS-002 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-FLAGS-003
语句：支持的 flag 必须与 3.9 文档一致：get 的 -s/-w、ls 的 -s/-w/-R、stat 的 -w、create 的 -s/-e/-c/-t、delete 的 -v、set 的
-s/-v、setAcl 的 -s/-v/-R、getAcl 的 -s、config 的 -c/-w/-s、配额命令的 -n/-b/-N/-B、removewatches 的 -c/-d/-a/-l、reconfig 的
-s/-v/-file/-members/-add/-remove。
标签：#capability #constraint
验证：VER-CLI-FLAGS-002
来源：CHG-0013；ZooKeeper 3.9 zookeeperCLI

REQ-CLI-EXT-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：首版不提供 :cmd 类应用扩展命令；该能力列为后续可支持项，启用前必须另行评审。
标签：#constraint #deferred
验证：VER-CLI-EXT-002
来源：CHG-0013

REQ-CLI-DANGER-001 status: superseded since: 2026-11-02 owner: 产品负责人 superseded-by: REQ-CLI-DANGER-002
语句：setAcl、addauth、setquota、delquota、removewatches、reconfig 执行前必须展示目标路径、参数摘要、当前连接与身份，并要求二次确认。
标签：#danger #confirm #security
验证：VER-CLI-DANGER-001
来源：CHG-0013

REQ-CLI-SCOPE-006 status: accepted since: 2026-11-02 owner: 产品负责人
语句：首版命令集为
ls、get、set、create、delete、deleteall、stat、getAcl、setAcl、addauth、sync、getAllChildrenNumber、getEphemerals、listquota、version、whoami、help；其余命令必须明确报错，不得静默忽略。
标签：#capability #constraint
验证：VER-CLI-SCOPE-006
来源：CHG-0015

REQ-CLI-FLAGS-003 status: accepted since: 2026-11-02 owner: 产品负责人
语句：仅上述命令提供 flag：ls 的 -s/-w/-R、get 的 -s/-w、stat 的 -w、create 的 -s/-e/-c/-t、delete 的 -v、set 的 -s/-v、setAcl 的
-s/-v/-R、getAcl 的 -s；-w 只对本次命令生效，不得建立长期订阅；未支持的 flag 必须明确报错。
标签：#capability #constraint #error
验证：VER-CLI-FLAGS-003
来源：CHG-0015

REQ-CLI-DANGER-002 status: accepted since: 2026-11-02 owner: 产品负责人
语句：setAcl 与 addauth 执行前必须展示目标路径、参数摘要、当前连接与身份，并要求二次确认。
标签：#danger #confirm #security
验证：VER-CLI-DANGER-002
来源：CHG-0015

REQ-CLI-SCOPE-007 status: accepted since: 2026-11-02 owner: 产品负责人
语句：命令历史与重跑、watcher 开关与移除、集群配置查看由界面承担，不得以命令形式提供。
标签：#constraint
验证：VER-CLI-SCOPE-007
来源：CHG-0015

REQ-CLI-SCOPE-008 status: accepted since: 2026-11-02 owner: 产品负责人
语句：reconfig、setquota、delquota 首版不得提供；后续如需支持，必须以界面能力单独评估。
标签：#constraint #danger #deferred
验证：VER-CLI-SCOPE-008
来源：CHG-0015
