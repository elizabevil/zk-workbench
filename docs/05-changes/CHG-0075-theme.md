---
doc_id: CHG-0075
title: 可配置背景与全局半透明面板
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0075

`AppearanceSettings` 结构化为：

```rust
mode: solid | gradient | image
color, gradient_from, gradient_to, gradient_angle
image_path
panel_opacity: 0-100
```

- 纯色、线性渐变、本地图片三种背景；
- 本地图片通过文件选择读入，复制到配置目录 `zkui/background.<ext>`，不依赖 asset 协议；
- 所有主要面板统一使用 `rgb(0 0 0 / panel_opacity)`，默认 50%；
- 设置页提供类型、颜色、渐变角度、图片选择/清除、面板透明度与实时预览；
- 背景与面板变化自动保存。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
