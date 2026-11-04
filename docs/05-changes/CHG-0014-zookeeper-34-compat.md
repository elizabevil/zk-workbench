---
doc_id: CHG-0014
title: 服务端兼容目标定为 3.4+ 与 zkCli 打包评估
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0014：服务端兼容目标定为 3.4+ 与 zkCli 打包评估

## 决策

1. 目标服务端最低版本：**ZooKeeper 3.4**。命令语法仍按 3.9 文档实现，但可用性以服务端实际能力为准，不支持时必须明确报错。
2. 不把 zkCli 打包进应用（sidecar）；命令由应用内 Rust 解析器按 zkCli 文法解析后经 Rust 核心执行。

## 依据

- 负责人要求兼容 3.4+。
- zkCli 作为 sidecar 会引入独立 session，破坏应用现有的会话、缓存与审计一致性（详见 `DEC-044`）。

## 影响

- 新增 `REQ-CONN-COMPAT-001` 与 `VER-CONN-COMPAT-001`。
- `DEC-043` 由 `proposed` 改为 `accepted`；新增 `DEC-044` 记录不打包 zkCli 的取舍。
- 实现时需按服务端版本探测命令能力，错误信息不得伪装成成功。

## 非目标（Non-Goals）

1. 不承诺 3.4 之前版本可用。
2. 不为 3.4 缺失的命令提供降级实现（只报错）。
3. 不打包 JRE、zkCli.jar 或 GraalVM native-image。
4. 不改变命令语法以适配旧版本（语法以 3.9 文档为准）。

## 候选方案与权衡（Alternatives & Trade-offs）

**打包 zkCli sidecar**：语法与行为天然一致，但会分裂 session（generation、Watcher、CAS、addauth、审计全部不共享），且需携带 JRE（约 150 MB+）或 native-image；命令摘要仍需自研解析器，收益有限。被拒绝。

**按服务端版本维护多套命令表**：维护成本高且容易与文档漂移；改为运行时能力探测 + 明确报错。

## 验证

`just --justfile docs/justfile check` 应输出 `RESULT: PASS`。
