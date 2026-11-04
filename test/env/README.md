# 测试环境编排

本目录只负责 **启动与销毁** ZooKeeper 测试环境，不执行任何用例。用例与结论记录在 `docs/ver/`。

## 结构

```text
test/env/
  justfile                只含 list / up / down
  scenarios/
    <场景名>/compose.yaml  每个场景一个目录，独立端口
```

## 用法

```bash
just --justfile test/env/justfile list
just --justfile test/env/justfile up zk-39-single
just --justfile test/env/justfile down zk-39-single
```

## 容器运行时探测顺序

1. `docker compose`（docker 命令存在且支持 compose 插件）
2. `podman compose`
3. `podman-compose`

三者都不存在时报错退出，不做回退安装。

## 场景

| 场景            | 镜像               | 宿主机端口     | 用途                                                                   |
|-----------------|--------------------|----------------|------------------------------------------------------------------------|
| `zk-39-single`  | `zookeeper:3.9`    | 2181           | 首版主环境：节点 CRUD/CAS/ACL/认证/会话                                |
| `zk-35-single`  | `zookeeper:3.5`    | 2182           | 3.5 兼容性（`getAllChildrenNumber` 可用、`getEphemerals` 不可用）      |
| `zk-34-single`  | `zookeeper:3.4.14` | 2183           | 3.4 最低版本兼容（`getEphemerals`、`getAllChildrenNumber` 应明确报错） |
| `zk-39-cluster` | `zookeeper:3.9` ×3 | 2211/2212/2213 | 集群、会话重建、巡检（第三阶段）                                       |

## 约定

- `down` 使用 `-v`，销毁时同时清理数据卷，保证下次从干净状态开始。
- 端口固定且场景间不重叠，便于用例直接写死连接地址。
- 镜像 tag 未在目标机器上验证：`zookeeper:3.4.14` 是否仍可从镜像仓库拉取需实测，失败时记录到对应 CHG，不静默换 tag。
- 测试产生的数据不写入 `docs/`；结论写入 `docs/ver/` 的对应用例。
