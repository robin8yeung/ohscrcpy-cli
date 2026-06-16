## 1. 协议编码扩展（control.rs）

- [x] 1.1 在 `cli/src/control.rs` 中新增 `encode_key_event(keycode: u32, is_pressed: bool) -> Vec<u8>`：subType=0x14 + isPressed(1B) + keyCode(4B BE)
- [x] 1.2 在 `cli/src/control.rs` 中新增 `encode_text_input(text: &str) -> Vec<u8>`：subType=0x15 + UTF-8 bytes
- [x] 1.3 为上述两个函数添加单元测试

## 2. SDL2 键盘事件处理

- [x] 2.1 在 `cli/src/control.rs` 中新增 `sdl_to_oh_keycode(scancode: sdl2::keyboard::Scancode) -> Option<u32>` 映射函数，覆盖字母 A-Z、数字 0-9、方向键、F1-F12、Enter/Tab/Space/Backspace/Delete/Escape、修饰键、符号键、数字小键盘、导航键
- [x] 2.2 在 `cli/src/renderer/mod.rs` 的 `AppEvent` 枚举中新增 `KeyDown { keycode: u32 }` 和 `KeyUp { keycode: u32 }` 变体
- [x] 2.3 在 `cli/src/renderer/sdl.rs` 的 `poll_events` 中处理 `Event::KeyDown` 和 `Event::KeyUp`，调用 `sdl_to_oh_keycode` 转换后推送 `AppEvent::KeyDown`/`AppEvent::KeyUp`
- [x] 2.4 在 `cli/src/main.rs` 主事件循环中处理 `AppEvent::KeyDown` 和 `AppEvent::KeyUp`，调用 `encode_key_event` 并通过 `ctrl_tx` 发送
- [x] 2.5 运行 `cargo build` 确认编译通过

## 3. 服务端剪贴板保存与恢复

- [x] 3.1 在 `InputInjector.handleTextInput` 中，`board.setData(pasteData)` 前通过 `board.getData()` 缓存原剪贴板内容
- [x] 3.2 在 Ctrl+V 粘贴完成后恢复原剪贴板内容；原内容为空时调用 `board.clear()`
- [x] 3.3 对 `getData()` 和 `setData()`（恢复阶段）的异常进行 try-catch，失败时记录 hilog 警告

## 4. 联调验证

- [ ] 4.1 构建 CLI（`cargo build --release`），启动服务端，连接设备，测试字母键（a-z）输入
- [ ] 4.2 测试数字键（0-9）、方向键（↑↓←→）、Enter/Tab/Space/Backspace/Delete/Escape
- [ ] 4.3 测试修饰键（Shift/Ctrl/Alt）和符号键
- [ ] 4.4 测试数字小键盘（如有）
- [ ] 4.5 验证剪贴板恢复功能
