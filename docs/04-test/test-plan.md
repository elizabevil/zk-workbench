---
doc_id: TEST-PLAN
title: 技术验证与测试计划
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 技术验证与测试计划

> 用例与实测结果记录在 `ver/` 的各域文件（`ver/node.md`、`ver/conn.md`、`ver/data.md`、`ver/ui.md`、`ver/cli.md`、
> `ver/auth.md`、`ver/ops.md`）。本文件只保留待回答的验证问题与故障场景，不记录执行结果。

## 测试环境

环境编排在 `test/env/`，只负责启动与销毁，不执行用例；说明见 `test/env/README.md`。

```bash
just --justfile test/env/justfile list
just --justfile test/env/justfile up zk-39-single
just --justfile test/env/justfile down zk-39-single
```

| 场景            | 镜像             | 端口           | 用途           |
|-----------------|------------------|----------------|----------------|
| `zk-39-single`  | zookeeper:3.9    | 2181           | 首版主环境     |
| `zk-35-single`  | zookeeper:3.5    | 2182           | 3.5 兼容性     |
| `zk-34-single`  | zookeeper:3.4.14 | 2183           | 最低版本 3.4   |
| `zk-39-cluster` | zookeeper:3.9 ×3 | 2211/2212/2213 | 集群与会话重建 |

运行时按 `docker compose` → `podman compose` → `podman-compose` 顺序探测；`down` 使用 `-v` 清理数据卷。

本文件定义当前阶段需回答的可行性问题。结果必须记录实际依赖版本、环境、步骤、观察结果和结论；计划本身不是通过证据。

### TEST-001 Rust ZooKeeper 客户端能力

验证候选 Rust 客户端在目标 ZooKeeper 环境下的连接、子节点读取、数据与 Stat 读取、version CAS、Watcher、断连和会话重建、认证及并发调用行为，并验证
Tauri 2 后端集成。

### TEST-002 树缓存与快速查询负载

用深树、宽树和高延迟环境验证按需加载、缓存命中、TTL 失效、Watcher 局部失效、generation
变化后缓存作废、路径直达及已加载范围搜索。记录请求数、响应时间、内存占用和数据新鲜度。结果用于决定 TTL 与容量，不预设 30
秒为最终值。

### TEST-003 多连接资源与生命周期

验证多个独立 session 并存时的内存、任务/线程、Watcher 数量、重连和关闭。确定活动连接上限、后台连接策略及应用退出行为。

### TEST-004 三平台关键 WebView 能力

Windows、macOS、Linux 分别验证虚拟滚动树、编辑器、大节点预览、文件导入导出、托盘和更新。特别记录 Linux WebKitGTK 的版本与性能结果。

### TEST-005 本地凭据保护

验证目标平台的系统钥匙串和打包体验；若采用本地加密存储，验证密钥来源、迁移和恢复方式，确认凭据不明文落盘。

## 故障场景

- 展开节点时断网，之后恢复连接。
- 会话过期后重新建立会话，检查 generation、Watcher 和缓存刷新。
- 缓存 TTL 内父节点发生新增、删除或重命名。
- 两个客户端并发修改同一节点，验证 CAS 冲突反馈。
- 一个连接断开或关闭，确认其他连接状态与缓存不受影响。
- 搜索或刷新过程中用户切换连接或取消操作。
- 删除影响预览遍历被用户取消，或预览后树被其他客户端修改，确认时如何重新校验。
- 递归删除执行中途失败：按「已完成 / 失败 / 未执行」分项报告，并只重试失败子树。
- 创建节点时父路径不存在：报错、显式勾选创建缺失父路径、中间节点为持久节点。
- 创建顺序节点后服务端返回完整路径，界面定位到实际路径而非用户输入。
- 临时节点创建后会话过期或重连：父节点列表与节点存在性按 generation 刷新。
