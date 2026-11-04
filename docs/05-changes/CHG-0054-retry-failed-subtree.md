---
doc_id: CHG-0054
title: 仅重试失败子树
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-03
---

# CHG-0054：仅重试失败子树

- 递归删除失败后，结果中保留失败路径；
- 界面出现「重试失败子树」，点击后以失败路径为目标重新打开删除对话框；
- 重试前重新执行影响预览并需要再次确认，不自动重试；
- 重试使用递归模式，避免再次漏删后代。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
```
