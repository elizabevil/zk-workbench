---
doc_id: CHG-0073
title: 树样式、节点信息、返回与设置页可用性
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0073

- 树：选中行青色描边；行尾操作图标默认半透明，hover 显示，增/删/改分别用绿/红/蓝区分；
- Data 页增加编码、版本、修改时间；Stat 页补全 czxid/mzxid/ctime/aversion 等字段；
- Timeline 改名「变更记录」，明确只记录写操作，并提供「查看全部审计」入口；
- 审计/可观测返回改为 `router.back()`，无历史时回首页；
- 日志单文件上限改为 MB 配置（存储仍折算字节）；
- 设置页左侧导航固定，右侧内容独立滚动；
- 代理分区增加「保存代理配置」与当前生效代理提示；
- 快捷键录制增加「已保存」提示与占位说明。

## 验证

```text
just check  exit 0
vue-tsc / biome 通过
docs 门禁 15/15 PASS
```
