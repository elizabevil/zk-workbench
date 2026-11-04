---
doc_id: CHG-0029
title: S2b 设置页、配置存储与代理档案接线
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0029：S2b 设置页、配置存储与代理档案接线

## 交付

| 层        | 内容                                                                                                                 |
|-----------|----------------------------------------------------------------------------------------------------------------------|
| core 配置 | `config/settings.rs`：Settings 及子结构、默认值、校验、重试间隔计算                                                  |
| core 存储 | `config/store.rs`：`<config_dir>/zkui/app.toml`，临时文件 + 重命名写入，重置删除覆盖文件                             |
| core 连接 | 连接管理器读取设置：有序代理档案、`no_proxy` 直连、重试参数；代理全部失败即报错，不静默回退直连                      |
| Tauri     | `settings_get`、`settings_save`、`settings_reset`；`AppState` 共享 `Arc<SettingsStore>`                              |
| 前端      | `api/settings.ts`、`stores/settings.ts`、`pages/settings/SettingsPage.vue`、`/settings` 路由、工作台入口、中英文文案 |

## 设置页分区

通用（语言、主题、恢复上次连接）、通知（级别、命令完成、声音）、重试（开关、次数、初始/最大间隔）、任务（失败后继续）、代理（开关、
`no_proxy`、档案增删与字段）、审计（保留天数）、可观测与日志（占位，后续切片补开关）。

## 验证

- 单元测试 34 条通过（新增配置默认值、重试间隔、保存/重载/重置、非法值拒绝）；
- 集成测试 5 条通过（含有序回退与 `no_proxy` 直连），目标 `192.168.11.5:2181`，代理 `192.168.10.1:10808`；
- `just check` 退出码 0（fmt、clippy、biome、vue-tsc、cargo test）。

## 行为细节

- 代理启用且档案为空：直接报错，不回退直连；
- 档案按顺序尝试，某个失败继续下一个；全部失败后按重试参数等待再来一轮；
- `no_proxy` 命中时完全绕过代理；
- 恢复默认只重置配置项，不删除凭据与审计数据。

## 未完成

1. 代理密码尚未接入 keyring（当前 `ProxyProfile.password` 仅内存，DTO 与配置文件都不包含）；
2. 设置页缺少「测试连接」按钮与档案上/下移动；
3. 可观测（OTel 开关、端点、采样率）与日志路径仅占位；
4. 审计功能未实现（S3）；
5. 语言切换只写配置，尚未驱动 `vue-i18n` 运行时切换。

## 非目标（Non-Goals）

1. 不实现审计存储与时间线。
2. 不实现 OTel 导出。
3. 不改动连接与代理的既有错误分类。
4. 不提供配置导入导出。
5. 不做多窗口配置同步。

## 验证命令

```bash
just check
ZK_SPIKE_ENDPOINT=192.168.11.5:2181 ZK_PROXY_ENDPOINT=192.168.10.1:10808 \
  cargo test -p zkui-core --test connection -- --ignored --test-threads=1
```
