---
doc_id: CHG-0076
title: 移除原生菜单，操作移入 Tools，细化分栏
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0076

- 移除 Tauri 原生 App/Edit 菜单，不再占用左上角；
- 工作区右上角 Tools 增加：审计、可观测、设置、关于、关闭窗口；
- 修正 Tools 下拉层级，避免被内容区遮挡；
- 分栏拖动条改为 3px 命中区 + 1px 细线，hover 变青色；
- 命令区移除历史按钮与右侧历史入口，历史保留在右侧栏「操作历史」Tab。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
