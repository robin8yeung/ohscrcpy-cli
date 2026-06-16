## Context

ohscrcpy 是一个 Rust CLI 二进制应用，使用 SDL2 进行窗口渲染。当前启动后 macOS Dock 显示系统默认图标。需要在程序启动时通过 macOS Objective-C 运行时设置自定义 Dock 图标和应用图标。

项目已有 `rust-embed` 依赖用于嵌入资源文件（如 `scrcpy_server.hap`），可直接复用此机制嵌入 ICNS 图标。

## Goals / Non-Goals

**Goals:**
- 在 macOS 上为 ohscrcpy 进程设置自定义蓝色屏幕监控图标
- 通过 ICNS 资源嵌入 + `objc` FFI 实现，无需创建 `.app` bundle
- 最小化依赖增量，不影响 Linux/Windows 编译

**Non-Goals:**
- 不创建 `.app` bundle 包装
- 不做 Windows/Linux 平台图标
- 不修改服务端代码

## Decisions

### 决策 1：ICNS 资源生成方式

**选择：手动转换 + 预生成 ICNS 文件**

使用 macOS 自带 `sips` 或 `iconutil` 工具将 PNG 转为 ICNS，作为静态资源存放在 `cli/assets/app_icon.icns`。

替代方案对比：
| 方案 | 优点 | 缺点 |
|------|------|------|
| 手动预生成 ICNS | 零构建依赖，构建速度快 | 需手动更新 |
| build.rs 中用 `icns` crate 转换 | 自动从 PNG 生成 | 新增编译依赖，构建变慢 |
| 在线服务生成 | 简单 | 依赖外部服务，不适合 CI |

选择预生成：图标是静态资源，几乎不会变化，无需自动化转换。

### 决策 2：运行时设置图标的方式

**选择：`objc` crate 直接调用 NSApplication**

```
┌──────────────────────────────────────────────────────────┐
│  main()                                                   │
│    │                                                      │
│    ├── set_process_icon()  ← 新增，通过 objc 调用        │
│    │     │                                                │
│    │     ── NSApplication.sharedApplication               │
│    │           .setApplicationIconImage_(NSImage)          │
│    │                                                      │
│    ── run()  ← 原有逻辑                                   │
│          ├── SDL2 初始化                                    │
│          └── 主循环                                         │
└──────────────────────────────────────────────────────────┘
```

使用 `objc` crate（轻量级 Objective-C 运行时绑定）直接调用：
- `NSImage` 从 ICNS 数据加载
- `NSApplication.sharedApplication.setApplicationIconImage_()` 设置图标

替代方案：
| 方案 | 优点 | 缺点 |
|------|------|------|
| `objc` crate | 轻量，精确控制 | 需手写 Objective-C 调用 |
| `cocoa` crate | 封装更好 | 依赖更重，引入整个 AppKit 绑定 |
| SDL2 `SDL_SetWindowIcon` | 跨平台 API | 只设置窗口图标，不设置 Dock 图标 |

选择 `objc`：最轻量，只引入必要的 FFI 调用。

### 决策 3：平台隔离

使用 `#[cfg(target_os = "macos")]` 条件编译，确保 Linux/Windows 不引入 `objc` 依赖。

## Risks / Trade-offs

- **[ICNS 多分辨率]** ICNS 需包含多种尺寸（16/32/64/128/256/512/1024）以保证 Retina 显示效果 → 使用 `iconutil` 生成含多尺寸的 ICNS
- **[objc crate 维护]** `objc` crate 更新不频繁 → 锁定版本号，定期审查
- **[Dock 图标时机]** 需在 SDL2 初始化前设置，否则可能被 SDL2 覆盖 → 在 `run()` 最开始调用
- **[Linux/Windows 无图标]** 不影响功能，Dock/任务栏显示默认图标 → 可接受，非本次范围
