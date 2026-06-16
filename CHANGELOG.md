# 变更日志

所有显著变更将记录在此文件中。格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/)。

## [0.1.1] - 2026-06-16

### Added

- 进程图标设置功能
- 键盘实时输入功能
- 安装脚本支持无 sudo 安装（`--user` 参数）

### Fixed

- 修复安装脚本的仓库名错误（ohscrcpy-cli）
- 修复安装脚本的版本解析问题（HTML 页面链接格式）

### Improved

- 优化安装脚本的错误处理（增加 API 速率限制处理和备用获取方式）
- 更新 .gitignore，忽略构建产物

## [1.0.0] - 2026-05-16

### Added

- 实时投屏：H.264 硬件编码 + 平台原生硬件解码
- 触控注入：鼠标操控设备触摸屏
- 应用管理：HAP 安装 / 卸载
- 音量 / 亮度远程控制
- 功能键模拟：返回、主页、最近任务
- 内嵌 hdc shell 模拟终端
- 运行时动态参数调整（分辨率、码率、帧率）
- macOS 客户端（VideoToolbox 硬解）
- Windows 客户端（MediaFoundation 硬解）
- macOS 打包脚本（签名 + 公证）
- Windows 打包脚本（Inno Setup）
