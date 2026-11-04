---
doc_id: CHG-0065
title: 菜单文案读取内置 i18n 数据
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0065：菜单文案读取内置数据

- 新增 `src-tauri/i18n/menu.zh-CN.json` 与 `menu.en-US.json`；
- `i18n.rs` 通过 `include_str!` 在编译期内置并按 `Locale` 解析，Rust 代码不再保存文案字面量；
- 菜单名称使用 Tauri `package_info().name`，不写死应用名。

## 验证

```text
just check  exit 0
```
