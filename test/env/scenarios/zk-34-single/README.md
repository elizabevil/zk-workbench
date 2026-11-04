# zk-34-single

单节点 ZooKeeper 3.4.14，宿主机端口 2183。

- 用途：验证最低支持版本 3.4；版本相关命令缺失时必须明确报错，不得静默降级。
- 对应：`REQ-CONN-COMPAT-001`、`VER-CONN-COMPAT-001`。
- 注意：镜像 tag `zookeeper:3.4.14` 需实测可拉取；3.4 无 `getEphemerals`/`getAllChildrenNumber`。
- 启动：`just --justfile test/env/justfile up zk-34-single`
