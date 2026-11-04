---
doc_id: CHG-0060
title: 菜单后端 i18n、构建信息与可配置日志目录
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0060：菜单 i18n 与配置化

## 变更

| 点            | 处理                                                                                              |
|---------------|---------------------------------------------------------------------------------------------------|
| 菜单语言判断  | 新增后端 `i18n` 模块，按 `Locale` 枚举取文案表，删除 `locale == "zh-CN"`                          |
| 名称/版本写死 | 新增 `app_info` 命令（来自 Tauri 构建信息），前端「关于」使用返回值；菜单「关于」用 `{name}` 模板 |
| 事件字面量    | Rust 侧 `menu::ACTION_EVENT` 与 `menu::actions::*`，前端 `api/events.ts` 的 `MENU_ACTION*` 常量   |
| 日志目录      | 设置新增 `logs.directory`（空 = 文档目录下 `zkui/logs`），启动时读取；设置页提供输入并提示需重启  |
| 格式检测      | 所有改动执行 `cargo fmt`、`biome`、`just check`                                                   |
| 空值          | 新增接口使用确定值（`AppInfo` 字符串、日志目录字符串），不再扩展 `null`                           |

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
