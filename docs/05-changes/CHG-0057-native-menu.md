---
doc_id: CHG-0057
title: 原生应用菜单
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0057：原生应用菜单

- 菜单：应用（设置、关于、关闭窗口）、编辑（撤销/重做/剪切/复制/粘贴/全选）；
- 没有退出快捷键；关闭窗口走窗口关闭而不是进程退出；
- 设置/关于通过 `menu.action` 事件交给前端处理；
- 保存设置时按当前语言重建菜单（zh-CN / en-US）。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
