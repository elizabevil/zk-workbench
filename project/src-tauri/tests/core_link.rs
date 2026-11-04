//! Tauri 侧的集成链接测试：确认 app crate 能依赖并使用 zkui-core。
//!
//! 真正的 IPC 命令测试要等 command 落地后再补；本测试只验证 workspace 依赖方向。

#[test]
fn core_crate_is_linked() {
    let version = zkui_core::version();
    assert!(!version.is_empty(), "zkui-core 版本号不应为空");
    assert_eq!(version.split('.').next(), Some("0"));
}
