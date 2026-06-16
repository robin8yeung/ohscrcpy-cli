## ADDED Requirements

### Requirement: PC 键盘按键事件转发

CLI 客户端 SHALL 通过 SDL2 事件系统捕获 PC 键盘按键事件（KeyDown/KeyUp），将 SDL2 Scancode 映射为 OpenHarmony KeyCode 后通过 `KEY_EVENT`（0x14）协议帧实时转发到服务端。

#### Scenario: 转发字母键

- **WHEN** 用户按下或释放 PC 键盘字母键（A-Z）
- **THEN** CLI SHALL 将 SDL2 Scancode 映射为 OH KeyCode（2017-2042），并以 `KEY_EVENT` 帧发送

#### Scenario: 转发数字键

- **WHEN** 用户按下或释放 PC 键盘数字键（0-9，主键盘区）
- **THEN** CLI SHALL 将 SDL2 Scancode 映射为 OH KeyCode（2000-2009），并以 `KEY_EVENT` 帧发送

#### Scenario: 转发方向键

- **WHEN** 用户按下或释放方向键（↑↓←→）
- **THEN** CLI SHALL 将 SDL2 Scancode 映射为 OH KeyCode（DPAD_UP=2012, DPAD_DOWN=2013, DPAD_LEFT=2014, DPAD_RIGHT=2015），并以 `KEY_EVENT` 帧发送

#### Scenario: 转发功能键 F1-F12

- **WHEN** 用户按下或释放 F1-F12 功能键
- **THEN** CLI SHALL 将 SDL2 Scancode 映射为 OH KeyCode（2090-2101），并以 `KEY_EVENT` 帧发送

#### Scenario: 转发 Enter/Tab/Space/Backspace/Delete/Escape

- **WHEN** 用户按下或释放这些特殊键
- **THEN** CLI SHALL 映射为对应 OH KeyCode（ENTER=2054, TAB=2049, SPACE=2050, ESCAPE=2070, BACKSPACE=2055, FORWARD_DEL=2071），并以 `KEY_EVENT` 帧发送

#### Scenario: 转发修饰键

- **WHEN** 用户按下或释放 Shift/Ctrl/Alt/Meta/CapsLock 等修饰键
- **THEN** CLI SHALL 映射为对应 OH KeyCode（SHIFT_LEFT=2047, SHIFT_RIGHT=2048, CTRL_LEFT=2072, CTRL_RIGHT=2073, ALT_LEFT=2045, ALT_RIGHT=2046, META_LEFT=2076, META_RIGHT=2077, CAPS_LOCK=2074, NUM_LOCK=2102, SCROLL_LOCK=2075）

#### Scenario: 转发符号键

- **WHEN** 用户按下或释放独立符号键（`-` `=` `[` `]` `\` `;` `'` `,` `.` `/` `` ` ``）
- **THEN** CLI SHALL 映射为对应 OH KeyCode（MINUS=2060, EQUALS=2061, LEFT_BRACKET=2056, RIGHT_BRACKET=2057, BACKSLASH=2058, SEMICOLON=2062, APOSTROPHE=2063, SLASH=2064, COMMA=2043, PERIOD=2044, GRAVE=2059）

#### Scenario: 转发数字小键盘

- **WHEN** 用户按下或释放数字小键盘键
- **THEN** CLI SHALL 映射为对应 OH KeyCode（NUMPAD_0-9=2103-2112, NUMPAD_DIVIDE=2113, NUMPAD_MULTIPLY=2114, NUMPAD_SUBTRACT=2115, NUMPAD_ADD=2116, NUMPAD_DOT=2117, NUMPAD_ENTER=2119）

#### Scenario: 转发导航键

- **WHEN** 用户按下或释放 Insert/Home/End/PageUp/PageDown/PrintScreen
- **THEN** CLI SHALL 映射为对应 OH KeyCode（INSERT=2083, MOVE_HOME=2081, MOVE_END=2082, PAGE_UP=2068, PAGE_DOWN=2069, SYSRQ=2079）

#### Scenario: 按键释放事件

- **WHEN** 用户释放键盘上的任意键
- **THEN** CLI SHALL 发送 `isPressed=0` 的 `KEY_EVENT` 帧

#### Scenario: 按键重复事件被过滤

- **WHEN** SDL2 产生 `repeat: true` 的 KeyDown 事件
- **THEN** CLI SHALL 忽略该事件，不发送任何帧

#### Scenario: 未映射的按键

- **WHEN** 用户按下 `sdl_to_oh_keycode` 返回 `None` 的按键
- **THEN** CLI SHALL 忽略该事件，不发送任何帧

### Requirement: KEY_EVENT 协议帧格式

CLI SHALL 按以下格式编码 `KEY_EVENT` 控制帧：

```rust
// 控制帧 body（6 字节）
subType: 1 byte = 0x14
isPressed: 1 byte (1=按下, 0=释放)
keyCode: 4 bytes (BE, OpenHarmony KeyCode 值)
```

#### Scenario: 按键按下帧

- **WHEN** CLI 发送 KeyDown 事件
- **THEN** 生成的帧 SHALL 满足：subType=0x14, isPressed=1, keyCode=对应 OH KeyCode

#### Scenario: 按键释放帧

- **WHEN** CLI 发送 KeyUp 事件
- **THEN** 生成的帧 SHALL 满足：subType=0x14, isPressed=0, keyCode=对应 OH KeyCode
