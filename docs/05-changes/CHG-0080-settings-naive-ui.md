---
doc_id: CHG-0080
title: 设置页引入 Naive UI
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-04
---

# CHG-0080

- 引入 `naive-ui@2.45.3`；
- [App.vue](project/src/App.vue) 增加全局 `NConfigProvider`（`darkTheme`、中文/英文 locale、深色 themeOverrides），后续页面可逐步复用；
- [SettingsPage.vue](project/src/pages/settings/SettingsPage.vue) 表单迁移到 `NForm` / `NFormItem` / `NInput` / `NSelect` / `NInputNumber` / `NCheckbox` / `NColorPicker` / `NSlider` / `NButton` / `NAlert`；
- 设置页布局要求保持不变：左栏导航居中、右栏标签在左/控件在右、复选框靠右、返回与缩放百分比在右上；
- 其他页面暂不迁移，后续按同一 Provider 逐步替换。

## 验证

```text
由用户手动验证
```

重点：设置页各分组、代理行、快捷键、日志路径、保存/恢复默认、缩放百分比。
