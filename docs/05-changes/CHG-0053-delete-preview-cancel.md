---
doc_id: CHG-0053
title: 删除预览可取消
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0053：删除预览可取消

- `node_delete_preview` 增加 `request_id`，后端在遍历子树时检查取消标记；
- 新增 `node_delete_preview_cancel`，按请求设置取消标记；
- 取消返回 `app.cancelled`，前端显示「已取消预览」而非错误；
- 预览仍在后台依次读取子节点，取消在两次请求之间生效。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
