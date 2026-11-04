---
doc_id: CHG-0042
title: 编辑连接档案
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0042：编辑连接档案

首页连接卡片增加「编辑」：复用新建对话框并回填名称、主机、端口；保存走 `profile_save` 的更新分支，保留原 id，不触发连接。

## 验证

```text
vue-tsc / biome / just check   通过
```
