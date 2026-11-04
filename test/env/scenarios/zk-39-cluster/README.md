# zk-39-cluster

三节点 ZooKeeper 3.9 集群，宿主机端口 2211/2212/2213。

- 用途：集群连接、会话重建、generation 变化、巡检（第三阶段）。
- 对应：`TEST-001`、`TEST-003`、`REQ-CONN-GEN-001/002`、`REQ-OPS-INSPECT-001/002`。
- 停止单个节点验证部分失败：`podman stop zkui-zk2`（或对应 docker 命令），不通过 justfile。
- 启动：`just --justfile test/env/justfile up zk-39-cluster`
