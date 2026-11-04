---
doc_id: CHG-0066
title: 修复页面未占满窗口
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0066：修复页面未占满窗口

## 原因

`#app` 有 `height: 100%`，但 `App.vue` 里的 `.app` 没有设置高度，百分比高度链在 `.app` 处断掉；工作区 `.shell` 的
`height: 100%` 因此按内容高度计算，状态栏下方留下空白。

## 修复

- `.app` 设置 `height: 100%`、`display: flex`、`flex-direction: column`，子页面 `flex: 1; min-height: 0`；
- 首页与设置页改为自身滚动（`height: 100%; overflow-y: auto`），避免外层 `overflow: hidden` 造成内容裁切。

## 验证

```text
npm run build      通过
just check         exit 0
```
