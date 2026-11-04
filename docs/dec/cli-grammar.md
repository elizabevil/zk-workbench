---
doc_id: DEC-CLI
title: 命令语法与命令集决策
status: accepted
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 命令语法与命令集决策

DEC-040 status: superseded updated: 2026-11-02 superseded-by: DEC-042
选择：应用内命令区使用 ZooKeeper 官方 zkCli 语法；支持 ls、get、set、create、delete、deleteall、stat、getAcl、sync、help；支持 get
-s、create -e/-s/-c/-t、set -v、delete -v；不支持 -w 与 ls -R；不提供应用扩展命令；deleteall
执行前确认但不统计后代数量；脚本层以 # 作为注释、保留引号与转义。
依据：负责人澄清原始需求为「在 UI 中直接使用 zkCli 命令」；CHG-0012。
推翻条件：zkCli 语法无法表达必要的写操作确认（例如需要结构化影响范围）时，回退到带类型的命令 DSL。
影响：supersede REQ-CLI-SCOPE-001、REQ-CLI-SCOPE-002；新增 REQ-CLI-GRAMMAR-001 等七条语句。

DEC-041 status: proposed updated: 2026-11-02
选择：`-w` watcher 订阅与 `ls -R` 全树遍历留到第三阶段评估，届时单独评审其对缓存与负载的影响。
依据：首版缓存失效策略已由内部 watcher 承担；全树遍历与按需加载原则冲突。
推翻条件：用户场景明确依赖命令行 watcher 或递归列举。
影响：无 REQ 变更。

DEC-042 status: superseded updated: 2026-11-02 superseded-by: DEC-045
选择：命令集与语法以 ZooKeeper 3.9 官方 CLI 文档（zookeeperCLI.html）为唯一依据。支持文档命令表中的
addauth、config、create、delete、deleteall、delquota、get、getAcl、getAllChildrenNumber、getEphemerals、history、listquota、ls、printwatches、reconfig、redo、removewatches、set、setAcl、setquota、stat、sync、version、whoami；排除
connect、quit、close（连接由界面管理）与四字命令（不在该命令表内）。flag 与文档一致。setAcl 提供 ACL 编辑，界面面板仍只读；addauth
允许显式叠加认证；危险命令二次确认；首版不提供 :cmd。
依据：负责人指定以 3.9 文档为准；CHG-0013。
推翻条件：需要支持低版本服务端命令子集，或需要四字命令进入 CLI。
影响：supersede DEC-040 与 REQ-AUTH-ACL-001、REQ-CONN-AUTH-001、REQ-CLI-SCOPE-003/004/005、REQ-CLI-FLAGS-001。

DEC-043 status: accepted updated: 2026-11-02
选择：必须支持连接 ZooKeeper 3.4 及以上；命令语法按 3.9 文档实现，但命令与 flag 的实际可用性由服务端版本决定，不支持时必须明确报错，不得静默降级或静默忽略参数。
依据：负责人要求兼容 3.4+；3.9 命令表中部分命令（如 getEphemerals、getAllChildrenNumber、config/reconfig）在 3.4 不存在。
推翻条件：实测显示 3.4 的协议或客户端库无法满足首版闭环。
影响：`REQ-CONN-COMPAT-001`；命令能力探测与错误文案随实现验证。

DEC-044 status: accepted updated: 2026-11-02
选择：不把 zkCli（Java 客户端）打包为 sidecar；命令由应用自身的 Rust 解析器按 zkCli 文法解析，经 Rust 核心执行。
依据：sidecar 会创建与应用主连接不同的独立 session，导致 generation、Watcher、CAS、addauth 身份、审计上下文全部分裂；还需打包
JRE 或 native-image，包体与启动延迟显著增加。
推翻条件：出现无法用 Rust 客户端表达、且 zkCli 能表达的命令需求（当前未发现）。
影响：无 REQ 变更；实现成本由 zkCli 文法的兼容测试承担。

DEC-045 status: accepted updated: 2026-11-02
选择：CLI 只保留 17
条命令（ls、get、set、create、delete、deleteall、stat、getAcl、setAcl、addauth、sync、getAllChildrenNumber、getEphemerals、listquota、version、whoami、help），只支持这些命令对应的
flag；-w 只对本次命令生效。命令历史与重跑、watcher 开关与移除、集群配置查看改为界面承担；reconfig、setquota、delquota
首版不提供，后续以界面形式评估。
依据：负责人确认「不必实现所有命令，一部分可以变成 UI 操作」；CHG-0015。
推翻条件：出现必须脚本化、界面无法表达的操作需求。
影响：supersede DEC-042 与 REQ-CLI-GRAMMAR-002、REQ-CLI-FLAGS-002、REQ-CLI-DANGER-001。
