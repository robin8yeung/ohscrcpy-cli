## Context

当前 CLI 客户端（Rust + SDL2）仅实现了返回键注入（`KeyBack`，subType=0x13）。键盘转发通道（`KEY_EVENT` 0x14）和文本输入通道（`TEXT_INPUT` 0x15）在服务端 `Protocol.ets` 中已定义 subType，`InputInjector.handleKeyEvent` 和 `handleTextInput` 已实现，但 CLI 客户端尚未接入。

文本输入通道服务端已实现"剪贴板写入 + Ctrl+V 粘贴"逻辑，但存在剪贴板覆盖问题——未保存/恢复用户原剪贴板内容。

```
┌──────────────────────────────────────────────────────────────────┐
│  CLI 客户端 (Rust + SDL2)                                       │
│                                                                  │
│  ┌─────────────┐   ┌──────────────────┐   ┌──────────────────┐  │
│  │ sdl.rs       │   │ control.rs       │   │ main.rs          │  │
│  │ poll_events  │   │ sdl_to_oh_keycode│   │ 事件循环         │  │
│  │ KeyDown/Up   │──▶│ encode_key_event │◀──│ ctrl_tx.send()   │  │
│  │              │   │ encode_text_input│   │                  │  │
│  └──────────────┘   └──────────────────┘   └────────┬─────────┘  │
│                                                      │           │
│         KEY_EVENT(0x14) / TEXT_INPUT(0x15)           │           │
│                         ▼                            ▼           │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │           写入线程 (TCP Writer)                           │    │
│  │   write_frame(type=0x10, payload) → flush               │    │
│  └──────────────────────────┬───────────────────────────────┘    │
└─────────────────────────────┼────────────────────────────────────┘
                              │ hdc fport
┌─────────────────────────────┼────────────────────────────────────┐
│  服务端 (OpenHarmony ArkTS) │                                    │
│                             ▼                                    │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │              ScrcpyService (TCP Server)                   │    │
│  │   PacketParser → 分发到 InputInjector.handle()           │    │
│  └──────────────────────────┬───────────────────────────────┘    │
│                             │                                    │
│         ┌───────────────────┼───────────────────┐                │
│         ▼                   ▼                   ▼                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐       │
│  │ handleKey    │  │ handleText   │  │ handleTouch/     │       │
│  │ Event()      │  │ Input()      │  │ Mouse/Volume…    │       │
│  │              │  │              │  │                  │       │
│  │ injectKey    │  │ 剪贴板缓存   │  │ injectTouch/     │       │
│  │ Event(keyCode)│  │ + Ctrl+V    │  │ MouseEvent       │       │
│  │              │  │ + 剪贴板恢复 │  │                  │       │
│  └──────┬───────┘  └──────┬───────┘  └──────────────────┘       │
│         │                 │                                      │
│         ▼                 ▼                                      │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │           OpenHarmony 系统输入栈                          │    │
│  │  MMI → IMF → 当前输入法 → insertText()                   │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

## Goals / Non-Goals

**Goals:**

- PC 键盘（内置及外接）按键通过 SDL2 事件系统实时转发到服务端，英文字母/数字/符号/方向键/功能键/数字小键盘可用
- 服务端 `handleTextInput` 粘贴前后保存并恢复用户剪贴板内容
- CLI 提供 `encode_key_event` 和 `encode_text_input` 编码函数，供后续文本输入 UI 调用
- 所有变更与现有协议帧格式完全兼容，无需改协议结构

**Non-Goals:**

- 不修改协议帧格式或新增帧类型
- 不涉及中文输入法（中文及任意 Unicode 文本通过 `TEXT_INPUT` 剪贴板路径处理）
- 不修改服务端 `injectSingleKey` 的注入机制
- 不处理 Ctrl 组合快捷键在服务端的语义解释（仅转发按键）
- 不处理 Shift+符号的修饰键组合输出 shifted 字符（Phase 1 先转发独立符号键）
- 不处理非美式键盘布局的字符映射差异

## Decisions

### 1. 键盘映射：使用 SDL2 Scancode（物理键位）而非 Keycode（逻辑键位）

**选择**：`sdl_to_oh_keycode` 接受 `sdl2::keyboard::Scancode`（物理键位），返回 `Option<u32>`（OH KeyCode）。

**理由**：
- Scancode 代表物理键位，跨平台（macOS/Windows/Linux）一致
- Keycode 受系统键盘布局影响，在不同平台行为不一致
- 与 Android scrcpy 的做法一致（使用 HID scancode）

**替代方案**：使用 `sdl2::keyboard::Keycode`（逻辑键位）→ 但 Keycode 在不同平台上的映射不一致，且与系统输入法交互复杂。不采用。

**映射覆盖**：88 个键位
- 字母 A-Z → 2017-2042
- 数字 0-9 → 2000-2009
- 方向键 ↑↓←→ → 2012-2015
- F1-F12 → 2090-2101
- Enter/Tab/Space/Backspace/Delete/Escape
- 修饰键：Shift/Ctrl/Alt/Meta（左右）、CapsLock、NumLock、ScrollLock
- 符号键：`-` `=` `[` `]` `\` `;` `'` `,` `.` `/` `` ` ``
- 导航键：Insert/Home/End/PageUp/PageDown/PrintScreen
- 数字小键盘：0-9、+、-、*、/、.、Enter

### 2. 事件过滤：排除按键重复事件

**选择**：`Event::KeyDown { repeat: false, .. }` 仅处理首次按下，忽略操作系统产生的重复事件。

**理由**：
- 服务端 `injectKeyEvent` 每次调用独立注入一个按键事件
- 按键重复由操作系统按固定间隔产生，频率和延迟与用户期望不一致
- 避免网络传输大量重复帧

### 3. 服务端剪贴板保存与恢复

**选择**：`handleTextInput` 在写剪贴板前通过 `pasteboard.getSystemPasteboard().getData()` 读取并缓存原有内容，粘贴完成后恢复。

**理由**：
- 当前实现直接覆盖用户剪贴板，用户体验差
- `pasteboard.getSystemPasteboard().getData()` 是 `@ohos.pasteboard` 标准 API，API 20 完全支持
- 恢复操作在粘贴完成后异步执行（~60ms），不影响文本注入时效

**时序**：
```
getData() → 缓存 originalPasteData (try-catch, 失败为 null)
  → setData(text) → injectKey(Ctrl down) → injectKey(V down) → injectKey(V up) → injectKey(Ctrl up)
    → restoreClipboard():
        originalPasteData != null → setData(originalPasteData)
        originalPasteData == null → clear()
```

### 4. 协议编码：使用 byteorder crate 的 WriteBytesExt

**选择**：`encode_key_event` 和 `encode_text_input` 使用项目已有的 `byteorder` crate 进行大端序编码。

**理由**：
- 项目已依赖 `byteorder = "1"`，无需新增依赖
- `WriteBytesExt::write_u32::<BigEndian>` 简洁且类型安全
- 与现有 `encode_touch`、`encode_video_params` 风格一致

## Risks / Trade-offs

- **[符号键 Shift 修饰]** 当前只转发物理键位的 keyCode，不判断 Shift/Ctrl 修饰键状态。`Shift+1`（期望 `!`）会转发为 `KEYCODE_1` 的 down/up，而非 `!`。
  → **缓解**：Phase 1 先确保字母/数字/独立符号键可用；修饰键组合在后续迭代中通过读取 SDL2 modifier 状态完善。

- **[剪贴板恢复竞态]** 粘贴是异步的（setTimeout 串联 ~60ms），在粘贴完成前如果用户手动复制了新内容，恢复操作可能覆盖用户的新剪贴板。
  → **缓解**：竞态窗口极短。如需更可靠方案，可改为监听剪贴板变化事件确认粘贴已消费后再恢复。

- **[非美式键盘布局]** `Scancode` 按物理键位映射，在 AZERTY/QWERTZ 等布局上键位映射可能与用户期望的字符不一致。
  → **缓解**：这是 scrcpy 类工具的通用限制。

## Open Questions

1. 设备侧输入法是否需要设置为特定输入法才能让注入的 keyCode 正常进入文本框？
2. 部分符号键在不同键盘布局下的 OH KeyCode 映射是否正确？需实机测试验证。
