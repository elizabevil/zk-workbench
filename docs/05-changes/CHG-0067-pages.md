---
doc_id: CHG-0067
title: 审计与可观测独立页面、设置页居中
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0067：页面拆分与设置页布局

- 新增 `/audit`：审计列表、校验、清理、导出、保留期设置；
- 新增 `/observability`：本地指标、OTel 配置与导出；
- 设置页移除审计/可观测分区，避免长页面与大量审计记录拖慢设置页；
- 首页与工作区顶栏增加“审计/可观测”入口；
- 设置页宽度居中（max-width + margin auto），左侧导航改为 18% flex 布局，右侧不再内部滚动。

## 验证

```text
just check  exit 0
vue-tsc/biome 通过
```
