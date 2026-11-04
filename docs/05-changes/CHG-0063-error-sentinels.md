---
doc_id: CHG-0063
title: 错误字段改为固定默认值
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0063：错误字段固定默认值

- `ErrorFields` 不再使用 `Option`：
    - `path` / `connection_id` / `detail` 缺失时为空字符串；
    - `version` 缺失时为 `-1`（ZooKeeper version 从 0 开始）；
    - `generation` 未知时为 `0`；
- 前端不再对 `fields` 做 null 判断，展示时把空串/-1 当作“无”；
- 错误契约保持 `{kind, scope, code, message_key, fields}`。

## 验证

```text
cargo test -p zkui-core --lib   通过
vue-tsc / just check            通过
```
