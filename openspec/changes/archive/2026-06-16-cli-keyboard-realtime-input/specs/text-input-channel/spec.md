## ADDED Requirements

### Requirement: TEXT_INPUT 协议帧格式

CLI SHALL 按以下格式编码 `TEXT_INPUT` 控制帧：

```rust
// 控制帧 body
subType: 1 byte = 0x15
textPayload: UTF-8 bytes (文本内容的 UTF-8 编码字节序列)
```

#### Scenario: ASCII 文本编码

- **WHEN** 编码纯 ASCII 文本（如 "hello"）
- **THEN** 生成的帧 SHALL 包含 subType=0x15，payload 为文本的 UTF-8 字节序列

#### Scenario: 中文文本编码

- **WHEN** 编码包含中文的文本（如 "你好"）
- **THEN** 生成的帧 SHALL 包含 subType=0x15，payload 为文本的 UTF-8 字节序列（6 字节）

#### Scenario: 空文本编码

- **WHEN** 编码空字符串
- **THEN** 生成的帧 SHALL 仅包含 subType=0x15（1 字节），无 payload

### Requirement: 服务端文本注入不破坏用户剪贴板

服务端在通过剪贴板粘贴方式注入文本时，SHALL 在粘贴完成后恢复用户原有的剪贴板内容。

#### Scenario: 保存原有剪贴板内容

- **WHEN** 服务端收到 `TEXT_INPUT` 帧
- **THEN** 服务端 SHALL 在写入待注入文本前，通过 `pasteboard.getSystemPasteboard().getData()` 读取并缓存当前剪贴板内容

#### Scenario: 粘贴后恢复剪贴板

- **WHEN** 服务端完成剪贴板写入 + Ctrl+V 粘贴注入流程
- **THEN** 服务端 SHALL 将缓存的原剪贴板内容通过 `pasteboard.getSystemPasteboard().setData()` 写回

#### Scenario: 原剪贴板为空

- **WHEN** 原有剪贴板内容为空
- **THEN** 服务端 SHALL 在粘贴完成后调用 `pasteboard.getSystemPasteboard().clear()` 清空剪贴板

#### Scenario: 剪贴板读取失败

- **WHEN** 读取原有剪贴板内容失败
- **THEN** 服务端 SHALL 继续执行文本注入流程（跳过保存），并在 hilog 中记录警告

### Requirement: 服务端接收文本输入帧

服务端 `InputInjector.handleTextInput` SHALL 解析 `TEXT_INPUT` 帧的 UTF-8 payload 并注入到当前焦点文本框。

#### Scenario: 解码 UTF-8 文本

- **WHEN** 服务端收到 `TEXT_INPUT` 帧
- **THEN** 服务端 SHALL 使用 `util.TextDecoder('utf-8')` 将 payload 解码为字符串

#### Scenario: 空 payload 不处理

- **WHEN** 服务端收到 `TEXT_INPUT` 帧但 payload 为空
- **THEN** 服务端 SHALL 不执行任何注入操作

#### Scenario: 剪贴板写入失败处理

- **WHEN** 剪贴板 `setData()` 操作失败
- **THEN** 服务端 SHALL 记录 hilog 错误日志，不继续执行 Ctrl+V 粘贴步骤
