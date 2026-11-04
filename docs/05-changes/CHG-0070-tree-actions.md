---
doc_id: CHG-0070
title: 树节点悬停增删改与整行选中
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0070

- 节点整行可点击选中，选中行高亮并支持键盘 Enter/Space；
- 悬停/选中时显示三个圆形操作按钮：上方 `+` 新建子节点、右侧 `✎` 编辑、下方 `×` 删除；
- 根节点不显示编辑/删除；
- 选中节点后按 `Delete` 直接打开删除确认；
- 树容器右侧预留按钮空间，避免按钮被裁切。

## 验证

```text
just check  exit 0
vue-tsc/biome 通过
```
