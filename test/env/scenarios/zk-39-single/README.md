# zk-39-single

单节点 ZooKeeper 3.9，宿主机端口 2181。

- 用途：首版主环境，覆盖连接、会话、节点 CRUD、CAS、ACL、认证、命令区。
- 对应：`TEST-001`、`TEST-002`、`TEST-005`；`docs/ver/` 中除 3.4/3.5 专项外的用例。
- 启动：`just --justfile test/env/justfile up zk-39-single`
