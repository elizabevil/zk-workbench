---
doc_id: DEC-TECH-ROUTE
title: 技术与依赖决策
status: proposed
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 技术与依赖决策

本文件只记录**实现手段**，不改变任何 REQ。技术方向可随 Spike 结果推翻；推翻时旧 DEC 置 `superseded` 并新开 DEC，REQ 不受影响。

DEC-001   status: proposed   updated: 2026-11-02
  选择：技术栈为 Rust + Tauri 2 + Vue 3；业务核心在 Rust，前端经受校验的 IPC 调用。
  依据：CHG-0005 推荐路线；三平台 WebView 最小验证尚未完成。
  推翻条件：TEST-004 显示关键 WebView 能力不可用，或三平台打包不可维护。
  影响：无 REQ 变更；若推翻需新开 DEC 并评估交付计划。

DEC-002   status: proposed   updated: 2026-11-02
  选择：ZooKeeper 客户端以 zookeeper-client 0.11.2 为首选候选，zookeeper-async 为对照；验证后再定生产依赖。
  依据：CHG-0005 候选比较、Spike A 矩阵设计。
  推翻条件：并发访问、重连、Watcher 恢复、递归删除部分失败、目标版本兼容任一不达标。
  影响：无 REQ 变更。

DEC-003   status: proposed   updated: 2026-11-02
  选择：编辑器使用 CodeMirror 6；Monaco 不进入首版默认依赖。
  依据：CHG-0005 编辑器比较；首版只需 JSON/YAML 查看编辑、基础脚本编辑与搜索替换。
  推翻条件：大文档性能或 WebView 兼容不达标，或后续需要 IDE 级脚本能力。
  影响：无 REQ 变更。

DEC-004   status: proposed   updated: 2026-11-02
  选择：一般偏好与凭据分开存储；凭据优先系统 keyring，无可用后端时明确提示，禁止明文降级。保存范围分四档：永不保存、for session（连接关闭即清除）、until restart（应用退出即清除）、forever（写系统凭据存储，默认不选）。
  依据：CHG-0005；Tauri Store 不作为凭据库。
  推翻条件：TEST-005 显示目标平台 keyring 不可用且 Stronghold 主密钥流程可接受。
  影响：若产生用户可见后果，新增 REQ；当前不修改已有 REQ。

DEC-005   status: proposed   updated: 2026-11-02
  选择：Tauri capabilities 按窗口与平台最小化授权；前端条件分支不作为安全边界。
  依据：PRD-010 的安全边界描述；Tauri Capabilities 官方文档。
  推翻条件：目标平台能力限制导致最小权限集合无法支撑必需交互。
  影响：无 REQ 变更；权限清单随实现逐项审查。
