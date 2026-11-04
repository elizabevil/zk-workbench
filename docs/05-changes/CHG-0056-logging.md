---
doc_id: CHG-0056
title: 文件日志与应用内日志目录
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0056：文件日志与脱敏

- 日志写入用户文档目录 `zkui/logs/zkui.log`，按天滚动；
- 写入线程非阻塞，初始化失败不影响应用启动；
- 对 `password=/token=/secret=/authorization:` 做掩码，`authorization` 掩码到行尾；
- 设置页「日志」展示该目录；配置路径与日志路径分离。

## 未完成

- 保留期与按大小滚动；
- 脱敏仅覆盖上述键名，不做通用敏感字段识别。

## 验证

```text
cargo test -p zkui --lib logging  2 passed
just check                        exit 0
```
