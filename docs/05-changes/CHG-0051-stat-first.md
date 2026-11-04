---
doc_id: CHG-0051
title: 浏览节点先读 Stat，数据按需加载
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0051：浏览节点先读 Stat

## 背景

ZooKeeper 的 `getData` 没有 offset/length 参数，服务端始终返回完整 znode；客户端也按完整长度解析，因此无法在网络层做真正的分段读取。

## 方案

- `node_stat` 改用 `getChildren` 的响应读取 Stat（含 `data_length`），不再为了显示大小而拉取数据；
- 工作区选中节点先读 Stat：数据量不超过 256 KiB 才自动加载数据；
- 超过阈值时只显示大小与「加载数据」按钮，由用户显式触发；
- 仍然保留应用侧截断与「加载完整内容」，避免大文本渲染卡顿。

## 效果

- 浏览节点（看 Stat/ACL/子树）不再产生完整数据传输；
- 点击「加载数据」时仍会传输完整 znode，这是协议限制，不是实现缺陷。

## 验证

```text
cargo test -p zkui-core --lib   68 passed
just check                      exit 0
vue-tsc / biome                 通过
```

## 待办优化（TODO）

1. **硬限制**：节点数据超过 `jute.maxbuffer`（默认 1 MiB）时拒绝加载并提示拆分，避免无效传输；
2. **网络级分段**：ZooKeeper 协议没有 offset/length，需要同时改造服务端与客户端（fork），暂不实施；
3. **外部投影**：把 znode 同步到支持 HTTP Range 的存储再直读，需要评估一致性、权限与运维成本，未排期；
4. **加载体验**：大节点加载缺少进度与取消，当前为一次性请求。
