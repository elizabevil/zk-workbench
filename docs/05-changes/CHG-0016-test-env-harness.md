---
doc_id: CHG-0016
title: 测试环境编排与容器运行时探测
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0016：测试环境编排与容器运行时探测

## 目的

为验证用例提供可复现的 ZooKeeper 环境：docker 或 podman 启动，每个场景独立目录，统一由一个环境 justfile 管理启动与销毁。

## 结构

```text
test/env/
  justfile                list / up / down，只负责启停
  scenarios/<场景>/compose.yaml
```

## 决策

1. 每个场景一个目录，端口固定且互不重叠：`zk-39-single` 2181、`zk-35-single` 2182、`zk-34-single` 2183、`zk-39-cluster`
   2211–2213。
2. 运行时自动探测：`docker compose` → `podman compose` → `podman-compose`；都不可用时报错，不尝试安装。
3. justfile 只提供 `list`、`up <场景>`、`down <场景>`；`down` 带 `-v` 清理数据卷，保证下次从干净状态开始。
4. 场景覆盖 3.4 / 3.5 / 3.9 与三节点集群，对应 `REQ-CONN-COMPAT-001` 与 `VER-CONN-COMPAT-001`。

## 影响

- 新增 `test/env/` 目录与 4 个场景；不修改产品代码。
- `04-test/test-plan.md` 增加「测试环境」一节指向该目录。

## 非目标（Non-Goals）

1. 不在 justfile 中执行测试用例或收集结果。
2. 不管理镜像构建（使用官方镜像）。
3. 不提供 CI 集成（后续单独决策）。
4. 不做故障注入（停节点用运行时原生命令，见场景 README）。
5. 不引入 testcontainers 等服务端库。

## 候选方案与权衡（Alternatives & Trade-offs）

**单个 compose 文件 + profiles 区分场景**：文件耦合、端口冲突需要额外变量，场景扩展后难以维护；改为每场景独立目录。

**testcontainers 在代码里拉起容器**：与语言和测试框架绑定，且不便于人工排查；改为外部编排 + justfile。

**在 justfile 中同时运行用例**：违反「环境只负责启停」的职责边界，且会让测试结果依赖 just 的退出码语义；被拒绝。

## 遗留

- 镜像 tag 未验证：`zookeeper:3.4.14` 是否仍可拉取需实测。本环境 `/run/user/1000` 只读，podman 无法启动，本轮只做了 justfile
  解析与 compose YAML 语法校验。
- 3.4 场景是否提供 SASL/TLS 变体，待认证验证需要时再定。

## 验证

- `just --justfile test/env/justfile --list` 输出三个 recipe。
- `just --justfile test/env/justfile list` 输出四个场景。
- 四个 `compose.yaml` 通过 YAML 解析校验。
