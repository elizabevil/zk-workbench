# zk-35-single

单节点 ZooKeeper 3.5，宿主机端口 2182。

- 用途：验证 3.5 兼容性；`getAllChildrenNumber` 应可用，`getEphemerals` 应明确报错。
- 对应：`REQ-CONN-COMPAT-001`、`VER-CONN-COMPAT-001`。
- 启动：`just --justfile test/env/justfile up zk-35-single`
