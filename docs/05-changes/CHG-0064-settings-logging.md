---
doc_id: CHG-0064
title: 通知等级、日志轮转与设置页侧边导航
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0064

## 通知

`NotificationSettings.desktop` 改为 `NotificationLevel` 枚举：`off / errors / warnings / all`，默认 `errors`；保留命令完成提醒与声音开关。场景：错误必达、警告可选、全部信息可开；命令完成单独控制。

## 日志

- 按等级拆分：`error.log`、`warn.log`、`info.log`、`debug.log`；
- 单文件默认上限 1 MiB，超限轮转为 `.log.1`；
- 默认保留 7 天，启动时按修改时间清理；
- 目录、大小上限、保留天数可在设置页配置，修改后需重启。

## 设置页

左侧分区导航，右侧可滚动内容；滚动时通过 IntersectionObserver 同步高亮当前分区。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
