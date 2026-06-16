## ADDED Requirements

### Requirement: ICNS 图标资源嵌入
CLI 二进制 SHALL 包含一个 ICNS 格式的图标资源文件，该资源在编译时被嵌入到二进制中。

#### Scenario: 构建时嵌入图标
- **WHEN** 执行 `cargo build`
- **THEN** `cli/assets/app_icon.icns`  SHALL 通过 `rust-embed` 被嵌入到 ohscrcpy 二进制中

### Requirement: macOS Dock 图标设置
在 macOS 平台上，ohscrcpy 启动时 SHALL 将 Dock 图标和应用图标设置为自定义图标。

#### Scenario: 启动时设置图标
- **WHEN** ohscrcpy 在 macOS 上启动
- **THEN** Dock 和应用图标 SHALL 显示为蓝色屏幕监控图标（而非系统默认图标）

#### Scenario: 非 macOS 平台不受影响
- **WHEN** ohscrcpy 在 Linux 或 Windows 上启动
- **THEN** 程序 SHALL 正常运行，不尝试设置图标，不引入 macOS 专属依赖

### Requirement: 图标设置时机
图标设置 SHALL 在 SDL2 窗口创建之前完成，确保图标不被 SDL2 初始化覆盖。

#### Scenario: 图标优先于 SDL2 初始化
- **WHEN** ohscrcpy 启动
- **THEN** `set_process_icon()` SHALL 在 `run()` 函数最开头、SDL2 初始化之前被调用
