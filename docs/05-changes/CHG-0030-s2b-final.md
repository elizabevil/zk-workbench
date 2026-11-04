---
doc_id: CHG-0030
title: S2b 收尾：keyring、测试连接、排序、OTel 配置与语言热切换
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0030：S2b 收尾

## 交付

| 项         | 内容                                                                                                                                |
|------------|-------------------------------------------------------------------------------------------------------------------------------------|
| 秘密存储   | `secrets/`：代理密码写入系统凭据存储（service `zkui`，account `proxy:<name>`），支持读写删与存在性检查；失败返回 `app_secret_store` |
| 连接       | 代理档案启用用户名且无密码时，从 keyring 读取；读不到按代理认证失败处理                                                             |
| Tauri 命令 | `proxy_test`（按档案与目标做握手）、`proxy_password_set/clear/has`、`app_paths`（配置与日志路径）                                   |
| 设置页     | 代理档案上/下移动、密码设置/清除、测试连接与目标、OTel 开关/端点/采样率、配置与日志路径展示                                         |
| 语言       | 设置加载/保存/重置后即时切换 `vue-i18n` locale；启动时加载设置                                                                      |
| 测试       | 单元 38 条（新增 secret store 错误映射、配置默认值扩展）；`just check` 退出码 0                                                     |

## 说明

- 代理密码不进入 DTO、配置文件与生成的 TS 类型；`ProxyProfile.password` 标注 `serde(skip)` 与 `ts(skip)`。
- OTel 目前只有配置与提示，导出实现留待后续切片；开启时提示会向端点发送数据。
- 语言热切换覆盖前端文案；系统原生菜单尚未实现，菜单文案待菜单切片接入。

## 未完成

1. keyring 在本沙箱（无桌面会话）未实测，需在有 Secret Service 的桌面环境验证；
2. 审计（S3）与 OTel 导出未实现；
3. 主题切换只写配置，尚未应用 CSS 变量；
4. 设置页缺少「恢复默认」的二次确认（当前直接重置）；
5. 代理档案的密码存在性未在 UI 上标识（有 `proxy_password_has` 命令但未接线）。

## 非目标（Non-Goals）

1. 不实现审计与 OTel 导出。
2. 不实现系统菜单。
3. 不做配置导入导出。
4. 不改动已确认的连接/代理行为。

## 验证

```bash
just check
# 38 unit tests + 1 link test；spike/集成用例默认 ignored
```
