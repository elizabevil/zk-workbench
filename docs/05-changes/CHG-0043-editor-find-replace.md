---
doc_id: CHG-0043
title: 编辑器查找与替换
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0043：编辑器查找与替换

- 当前文档内查找、匹配计数与「下一个」定位；
- 替换当前项；
- 全部替换前显示匹配数并确认，替换后可撤销（单级快照）；
- 所有替换只作用于本地草稿，保存仍需 diff 确认与 CAS。

验证：`vue-tsc` / `biome` / `just check` 通过。
