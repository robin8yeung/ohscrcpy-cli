## Why

当前 ohscrcpy CLI 启动后，macOS Dock 和窗口左上角显示的是系统默认的齿轮/终端图标，无法让用户快速识别这是 ohscrcpy 应用。需要为进程设置自定义图标，提升品牌识别度和用户体验。

## What Changes

- 将用户提供的蓝色屏幕监控图标（PNG）转换为 macOS ICNS 格式，嵌入到 CLI 二进制中
- 在程序启动时（SDL2 窗口创建前），通过 macOS Objective-C 运行时调用 `NSApplication` 设置 Dock 图标和应用图标
- 仅影响 macOS 平台，Linux/Windows 不受影响

**非目标**：
- 不创建 `.app` bundle（当前是纯 CLI 二进制）
- 不修改服务端（scrcpy_server）代码
- 不做 Windows/Linux 平台的图标设置

## Capabilities

### New Capabilities

- `process-icon`: 为 ohscrcpy CLI 进程设置自定义应用图标，包括 ICNS 资源准备、macOS 运行时图标设置、构建流程集成

### Modified Capabilities

（无）

## Impact

- **cli/Cargo.toml**: 新增 `objc` crate 依赖（仅 macOS）
- **cli/src/icon.rs**: 新增模块，提供 `set_process_icon()` 函数
- **cli/src/main.rs**: 在 `run()` 入口处调用图标设置
- **cli/assets/**: 新增 `app_icon.icns` 资源文件
- **cli/build.rs**（可选）: 添加 PNG → ICNS 转换逻辑
- 仅 macOS 平台受影响，Linux/Windows 编译不受影响
