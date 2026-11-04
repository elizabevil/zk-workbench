---
doc_id: CHG-0078
title: 工作区树、命令区与缩放修复
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0078

- 首页连接列表与分组标题整体居中，标题/提示与搜索框分列两侧；
- 树改为按层缩进与半透明连接线，子节点前缀用半透明 `–`；节点行宽随内容，增删改按钮贴近节点显示；
- 树宽度改用 `--tree-width`，修复拖拽分栏不生效；
- 数据区字节长度、编码、版本、格式切换移到内容下方；右上刷新只重新读取当前节点；
- 命令区改为可拖动高度，默认占工作区 40%，并记忆到本地；
- 弹窗字号改为 WebView 整体缩放，修复 Teleport 弹窗不跟随字号设置；新增 Ctrl/Cmd+滚轮缩放与 Ctrl/Cmd+0 复位；
- 审计与可观测页面使用黑色半透明面板背景；
- 修复树子节点加载失败污染数据区错误状态的问题：已展开的 `/cc` 无 READ 权限时，数据区仍显示当前节点内容，不再误报「权限不足」。

## 验证

```text
vue-tsc --noEmit        通过
biome check src config  通过（仅既有 2 条 warning）
vite build              通过
cargo test --workspace  通过（ZooKeeper 用例仍为 ignored）
```

手工验证：连接测试集群，展开 `/cc` 后选中 `/test/new/yaml` 执行 `ls /test/new/yaml`，数据区保持 YAML 内容；树与命令区分栏可拖动；字号
25 的弹窗随整体缩放。
