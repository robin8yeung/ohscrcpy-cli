## Why

当前 CLI 客户端（Rust + SDL2）仅实现了返回键注入（`KeyBack`，subType=0x13），PC 键盘的字母/数字/符号/功能键无法转发到 OpenHarmony 设备。同时服务端 `handleTextInput` 通过剪贴板+Ctrl+V 粘贴注入文本，但会直接覆盖用户剪贴板内容。现需为 CLI 客户端添加键盘实时输入功能，使 PC 键盘按键全面转发到服务端，并优化服务端剪贴板处理。

## What Changes

- **CLI 协议编码扩展**：在 `control.rs` 中新增 `encode_key_event`（KEY_EVENT 0x14）和 `encode_text_input`（TEXT_INPUT 0x15）编码函数
- **SDL2 键盘事件映射**：新增 `sdl_to_oh_keycode` 函数，将 SDL2 Scancode（物理键位）映射为 OpenHarmony KeyCode，覆盖字母 A-Z（2017-2042）、数字 0-9（2000-2009）、方向键（2012-2015）、F1-F12（2090-2101）、修饰键、符号键、导航键、数字小键盘共 88 个键位
- **SDL2 事件处理扩展**：在 `poll_events` 中处理 `Event::KeyDown`/`Event::KeyUp`，通过 `AppEvent::KeyDown`/`AppEvent::KeyUp` 传递到主事件循环
- **主事件循环键盘发送**：在 `main.rs` 事件循环中调用 `encode_key_event` 并通过 `ctrl_tx` 发送到写入线程
- **服务端剪贴板保存与恢复**：`handleTextInput` 在粘贴前后保存并恢复用户原有剪贴板内容

## Non-Goals

- 不修改协议帧格式或新增帧类型（`KEY_EVENT` 0x14 和 `TEXT_INPUT` 0x15 已在服务端 `Protocol.ets` 中定义）
- 不涉及中文输入法处理（中文及任意 Unicode 文本通过 `TEXT_INPUT` 剪贴板路径处理）
- 不修改服务端 `injectSingleKey` 的注入机制（已确认 `inputEventClient.injectKeyEvent` 对所有 keyCode 有效）
- 不处理 Ctrl 组合快捷键在服务端的语义解释（如 Ctrl+C 复制等，仅转发按键）
- 不处理 Shift+符号的修饰键组合输出 shifted 字符（Phase 1 先转发独立符号键，修饰键组合在后续迭代中完善）
- 不处理非美式键盘布局（AZERTY/QWERTZ 等）的字符映射差异

## Capabilities

### New Capabilities

- `keyboard-forwarding`: CLI 客户端键盘转发 — PC 键盘（内置及外接）按键事件通过 SDL2 事件系统捕获，经 SDL2 Scancode → OH KeyCode 映射后，通过 `KEY_EVENT`（0x14）协议实时转发到服务端
- `text-input-channel`: 文本输入通道 — 客户端通过 scrcpy 协议 `TEXT_INPUT`（0x15）通道发送 UTF-8 文本，服务端以剪贴板粘贴方式注入目标文本框

### Modified Capabilities

（无现有 spec 需要修改）

## Impact

- **CLI 客户端核心文件**：
  - `cli/src/control.rs` — 新增 `encode_key_event`、`encode_text_input`、`sdl_to_oh_keycode` 函数及单元测试
  - `cli/src/renderer/mod.rs` — `AppEvent` 枚举新增 `KeyDown`/`KeyUp` 变体
  - `cli/src/renderer/sdl.rs` — `poll_events` 处理 SDL2 `Event::KeyDown`/`Event::KeyUp`
  - `cli/src/main.rs` — 主事件循环处理 `AppEvent::KeyDown`/`AppEvent::KeyUp` 并发送
- **服务端核心文件**：
  - `scrcpy_server/entry/src/main/ets/scrcpyservice/InputInjector.ets` — `handleTextInput` 增加剪贴板保存/恢复
- **受影响平台**：macOS / Windows / Linux CLI 均适用（SDL2 Scancode 跨平台一致）
- **无新增依赖**：使用已有的 `byteorder` crate 进行大端序编码，SDL2 事件系统已集成
- **ArkTS 严格模式兼容**：服务端改动不涉及新类型，仅使用已有的 `pasteboard.getData()`/`setData()` API
