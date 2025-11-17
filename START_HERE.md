# 🚀 MonoFocus - 从这里开始

欢迎使用 MonoFocus！这是一个简单的入门指南，帮助你快速上手。

---

## 📚 我应该先读哪个文档？

根据你的角色选择：

### 👤 **普通用户 - 我想使用这个软件**
1. 阅读 [README.md](README.md) - 了解软件功能
2. 从 [Releases](https://github.com/yourusername/MonoFocus/releases) 下载安装包
3. 安装并运行

### 👨‍💻 **开发者 - 我想参与开发**
1. 阅读 [QUICKSTART.md](QUICKSTART.md) - 5分钟快速入门
2. 参考 [BUILD.md](BUILD.md) - 配置开发环境
3. 查看 [DEVELOPMENT.md](DEVELOPMENT.md) - 了解详细架构
4. 阅读 [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献指南

### 🔧 **贡献者 - 我想修复bug或添加功能**
1. 阅读 [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献流程
2. 查看 [GitHub Issues](https://github.com/yourusername/MonoFocus/issues) - 寻找任务
3. 参考 [DEVELOPMENT.md](DEVELOPMENT.md) - 技术细节

---

## ⚡ 超级快速开始（开发者）

如果你已经熟悉 Rust + Tauri 开发：

```bash
# 1. 克隆 & 安装
git clone https://github.com/yourusername/MonoFocus.git
cd MonoFocus
npm install

# 2. 运行
npm run tauri dev

# 3. 构建
npm run tauri build
```

就这么简单！

---

## 📁 项目文档速查

| 文档 | 内容 | 适用人群 |
|------|------|----------|
| [README.md](README.md) | 项目介绍、功能说明 | 所有人 |
| [QUICKSTART.md](QUICKSTART.md) | 5分钟快速入门 | 开发者 |
| [BUILD.md](BUILD.md) | 环境配置、构建指南 | 开发者 |
| [DEVELOPMENT.md](DEVELOPMENT.md) | 详细技术架构 | 开发者 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 贡献指南、代码规范 | 贡献者 |
| [CHANGELOG.md](CHANGELOG.md) | 版本更新历史 | 所有人 |
| [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | 项目总结 | 维护者 |

---

## 🎯 常见任务快速链接

### 开发任务
- [安装开发环境](BUILD.md#环境配置)
- [运行开发模式](QUICKSTART.md#第二步运行开发模式)
- [添加新功能](QUICKSTART.md#添加-tauri-命令)
- [运行测试](BUILD.md#测试)
- [构建发布版](BUILD.md#生产构建)

### 贡献任务
- [提交 Bug](https://github.com/yourusername/MonoFocus/issues/new?template=bug_report.md)
- [建议新功能](https://github.com/yourusername/MonoFocus/issues/new?template=feature_request.md)
- [查看待完成任务](https://github.com/yourusername/MonoFocus/issues)
- [了解提交规范](CONTRIBUTING.md#提交规范)
- [创建 Pull Request](CONTRIBUTING.md#pull-request-流程)

---

## 🏗️ 项目结构一览

```
MonoFocus/
│
├── 📄 文档
│   ├── README.md               # 项目介绍
│   ├── QUICKSTART.md           # 快速入门
│   ├── BUILD.md                # 构建指南
│   ├── DEVELOPMENT.md          # 开发文档
│   ├── CONTRIBUTING.md         # 贡献指南
│   └── CHANGELOG.md            # 更新日志
│
├── 🎨 前端 (src/)
│   ├── index.html              # 主界面
│   ├── styles.css              # 样式
│   ├── main.js                 # 前端逻辑
│   └── overlay.html            # 遮罩页面
│
├── 🦀 后端 (src-tauri/src/)
│   ├── main.rs                 # 入口
│   ├── monitor.rs              # 显示器检测
│   ├── mouse_watcher.rs        # 鼠标监听
│   ├── overlay.rs              # 遮罩窗口
│   ├── config.rs               # 配置管理
│   └── tray.rs                 # 系统托盘
│
└── ⚙️ 配置
    ├── package.json            # npm 配置
    ├── src-tauri/Cargo.toml    # Rust 依赖
    └── src-tauri/tauri.conf.json # Tauri 配置
```

---

## 💡 快速答疑

### Q: 我没有 Rust 经验，可以参与开发吗？
A: 可以！你可以：
- 改进前端 UI（只需 HTML/CSS/JS）
- 完善文档
- 报告 Bug
- 提出功能建议

### Q: 首次编译需要多久？
A: 约 10-15 分钟（取决于网络和硬件）。后续编译只需几秒。

### Q: 如何调试代码？
A: 查看 [QUICKSTART.md - 调试技巧](QUICKSTART.md#调试技巧)

### Q: 遇到编译错误怎么办？
A: 查看 [BUILD.md - 常见问题](BUILD.md#常见问题)

### Q: 如何提交我的代码？
A: 查看 [CONTRIBUTING.md - 开发流程](CONTRIBUTING.md#开发流程)

---

## 🎓 学习资源

### Rust 新手
- [Rust 官方书](https://doc.rust-lang.org/book/) - 从零开始学 Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 通过示例学习

### Tauri 新手
- [Tauri 快速入门](https://tauri.app/v1/guides/getting-started/setup)
- [Tauri API 文档](https://tauri.app/v1/api/js/)

### 前端基础
- [MDN Web 文档](https://developer.mozilla.org/zh-CN/)
- [JavaScript.info](https://zh.javascript.info/)

---

## 🤝 社区

- 💬 [GitHub Discussions](https://github.com/yourusername/MonoFocus/discussions) - 讨论和提问
- 🐛 [GitHub Issues](https://github.com/yourusername/MonoFocus/issues) - 报告问题
- 📧 Email: support@monofocus.example

---

## ⭐ 支持项目

如果你觉得这个项目有用：
- ⭐ Star 这个仓库
- 🐛 报告 Bug
- 💡 提出新功能
- 📝 改进文档
- 🔀 提交代码

---

## 📄 许可证

MIT License - 查看 [LICENSE](LICENSE) 文件

---

<div align="center">

**准备好了吗？选择你的角色，开始探索吧！🚀**

[👤 我是用户](README.md) | [👨‍💻 我是开发者](QUICKSTART.md) | [🔧 我想贡献](CONTRIBUTING.md)

</div>

