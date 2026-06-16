## 1. 资源准备

- [x] 1.1 将用户提供的 PNG 图标转换为 ICNS 格式，包含多分辨率（16/32/64/128/256/512/1024），保存到 `cli/assets/app_icon.icns`
  - 验证：`file cli/assets/app_icon.icns` 显示为 ICNS 格式

## 2. 依赖添加

- [x] 2.1 在 `cli/Cargo.toml` 中添加 `objc` crate 依赖（macOS only），使用 `#[cfg(target_os = "macos")]` 条件依赖
  - 验证：`cargo check` 在 macOS 上通过，Linux 上不受影响

## 3. 图标模块实现

- [x] 3.1 创建 `cli/src/icon.rs` 模块，实现 `set_process_icon()` 函数
  - 通过 `rust-embed` 加载嵌入的 ICNS 数据
  - 使用 `objc` 运行时调用 `NSImage` 和 `NSApplication.sharedApplication.setApplicationIconImage_()`
  - 使用 `#[cfg(target_os = "macos")]` 条件编译
  - 验证：`cargo check` 通过，无编译警告

- [x] 3.2 在 `cli/src/lib.rs` 或 `cli/src/main.rs` 中添加 `mod icon;` 声明（条件编译）
  - 验证：模块正确声明

## 4. 主流程集成

- [x] 4.1 在 `cli/src/main.rs` 的 `run()` 函数最开头（SDL2 初始化之前）调用 `icon::set_process_icon()`
  - 验证：`cargo build` 成功

## 5. 构建验证

- [x] 5.1 在 macOS 上执行 `cargo build` 并运行 `./target/debug/ohscrcpy`，确认 Dock 图标已变为自定义图标
  - 验证：肉眼确认 Dock 显示蓝色屏幕监控图标

- [x] 5.2 在 Linux 上执行 `cargo check`，确认不影响跨平台编译
  - 验证：`cargo check` 无错误

## 6. 文档更新

- [x] 6.1 更新 `openspec/changes/set-process-icon/tasks.md` 中所有 checkbox 状态
  - 验证：所有任务标记为完成
