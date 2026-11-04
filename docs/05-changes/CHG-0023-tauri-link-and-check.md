---
doc_id: CHG-0023
title: Tauri 链接验证与代码侧校验流水线
status: active
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# CHG-0023：Tauri 链接验证与代码侧校验流水线

## 目的

在系统依赖补齐后，验证 app crate 能链接 `zkui-core`，并把代码侧检查接入统一入口。

## 环境

- `pkg-config`、`glib-2.0`、`gtk+-3.0`、`webkit2gtk-4.1`、`libsoup-3.0`、`javascriptcoregtk-4.1` 均已可用；
- Node 26.12.1、npm 12.2.1、Cargo 1.99.1。

## Tauri 链接验证

```bash
cd project
CARGO_HOME=$PWD/.cargo-home cargo test -p zkui --test core_link
```

```text
Compiling zkui v0.1.0 (/home/debi/IdeaProjects/ZK-Workbench/project/src-tauri)
Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 09s
running 1 test
test core_crate_is_linked ... ok
test result: ok. 1 passed
```

## 校验流水线

新增 `project/justfile`（代码侧）与根 `justfile`（统一入口）：

```text
just check
├─ docs-check  → just --justfile docs/justfile check（15 项文档门禁）
└─ code-check  → just --justfile project/justfile check
                ├─ fmt-check   cargo fmt --all -- --check
                ├─ clippy      cargo clippy --workspace --all-targets -D warnings
                ├─ lint        biome check src config
                ├─ typecheck   vue-tsc --noEmit
                └─ test        cargo test --workspace（spike 用例默认 ignored）
```

`just check` 实际执行结果：退出码 0，文档门禁 15/15 PASS，代码侧全部通过。

## 工具链修正：TypeScript 固定 6.0.x

`vue-tsc` 3.3.14 通过 `typescript/lib/tsc` 子路径加载编译器。TS 7 的 package.json 不再暴露该子路径，`vue-tsc --noEmit` 会报
`ERR_PACKAGE_PATH_NOT_EXPORTED`。实测：

- TS 7.0.2：失败；
- TS 6.0.3：通过（package.json 无 `exports` 限制，子路径可解析）；
- 最终固定 `typescript ~6.0.3`，记录为 `DEC-033`。

## 影响

- 根 `justfile` 成为统一入口，子 justfile 各自独立运行、不互相调用。
- 代码侧检查从「手动命令」变为可重复执行的 recipe，满足「测试与脚本必须落盘」规则。
- 测试文件应用 `cargo fmt` 格式化，移除未使用的辅助函数。

## 非目标（Non-Goals）

1. 不引入 CI 流水线（本地流程先跑通）。
2. 不覆盖 IPC 命令测试（命令尚未实现）。
3. 不做性能基准。
4. 不改变产品质量语句与需求。

## 验证

`just check` 退出码 0；`cargo test -p zkui --test core_link` 通过；`biome check src config` 无错误。
