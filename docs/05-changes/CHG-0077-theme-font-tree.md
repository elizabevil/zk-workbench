---
doc_id: CHG-0077
title: 首页居中、界面字号与磨砂背景
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0077

- 首页 Connections 标题与操作区居中；
- `AppearanceSettings` 新增 `font_size`（默认 14px），通过缩放整体 UI 生效；
- 新增 `background_blur`（0-30px）：背景层独立渲染并模糊，面板使用 `backdrop-filter` 形成磨砂玻璃；
- 修复 `TreeNodeItem.vue` 样式块丢失：补回 tree 连接线、选中描边、仅 hover 显示操作按钮的样式。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
