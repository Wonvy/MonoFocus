# Changelog

所有重要的变更都会记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [1.0.0] - 2025-11-17

### 新增
- 🎉 初始版本发布
- 🖥️ 多显示器自动检测（Windows / macOS / Linux）
- 🎯 实时鼠标位置跟踪
- 🌓 动态遮罩层（非活跃显示器显示半透明黑色遮罩）
- 🎚️ 可调节透明度（0-80%）
- 🎨 显示器布局可视化（真实比例展示）
- 💾 配置持久化存储
- 🔔 系统托盘支持
- 🚀 开机自启动选项
- 🌍 跨平台支持（Windows 10+, macOS 10.15+, Linux）
- 🎯 遮罩点击穿透（不影响正常操作）
- 📝 完整的文档和开发指南

### 技术细节
- 使用 Rust 1.70+ 和 Tauri 1.5+ 构建
- 极简前端（HTML5 + CSS3 + Vanilla JS）
- 轻量级，资源占用低
- 100ms 鼠标轮询间隔

---

## 版本规划

### [1.1.0] - 计划中
- [ ] 遮罩淡入淡出动画
- [ ] 自定义遮罩颜色
- [ ] 键盘快捷键支持
- [ ] 更多主题选项

### [1.2.0] - 计划中
- [ ] Wayland 完整支持（Linux）
- [ ] 多语言界面（中文、英文、日文等）
- [ ] 每显示器独立透明度设置
- [ ] 统计功能（使用时长、切换次数等）

### [2.0.0] - 长期规划
- [ ] 可视化配置编辑器
- [ ] 预设方案（工作、休息、游戏等）
- [ ] 云同步配置
- [ ] 插件系统

---

[Unreleased]: https://github.com/yourusername/MonoFocus/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/yourusername/MonoFocus/releases/tag/v1.0.0

