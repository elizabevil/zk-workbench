---
doc_id: DEC-PLATFORM
title: 平台支持与最低版本决策
status: proposed
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# 平台支持与最低版本决策

本文件记录平台范围与最低版本候选；未在目标平台验证前，不得对外宣称「已支持」。

DEC-030   status: accepted   updated: 2026-11-02
  选择：目标平台为 Windows、macOS、Linux 桌面；最低版本候选为 Windows 10 1809+、macOS 12+、Ubuntu 22.04+（WebKitGTK 2.40+）。
  依据：Tauri 2 与 WebView 版本要求；三平台验证尚未执行。
  推翻条件：TEST-004 实测显示候选版本无法满足编辑器、虚拟树或文件对话框需求。
  影响：无 REQ 变更；版本号是验证目标，不是已完成的兼容性承诺。

DEC-031   status: proposed   updated: 2026-11-02
  选择：首版不做自动更新、代码签名与公证；发布流程在正式发布前单独决策。
  依据：当前处于需求与验证阶段，签名与更新涉及持续成本与账号责任。
  推翻条件：负责人要求首版即提供自动更新。
  影响：无 REQ 变更。

DEC-032   status: accepted   updated: 2026-11-02
  选择：工具链基线取当前项目环境：Node 26.12.1、npm 12.2.1、Cargo 1.99.1（rustc 1.99.1）。版本以仓库声明文件为准：package.json 的 engines、rust-toolchain.toml、Cargo.lock。
  依据：负责人指定以当前环境为基线；CHG-0017。
  推翻条件：目标平台构建或依赖要求提高最低版本。
  影响：写入 package.json、rust-toolchain.toml 与锁文件；升级须单独评审。
