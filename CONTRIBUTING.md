# 贡献指南

感谢你考虑为 MonoFocus 做出贡献！🎉

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发流程](#开发流程)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [问题反馈](#问题反馈)

---

## 🤝 行为准则

本项目采用 [Contributor Covenant](https://www.contributor-covenant.org/) 行为准则。
参与本项目即表示你同意遵守其条款。

核心原则：
- 尊重所有贡献者
- 接受建设性批评
- 关注对社区最有利的事情
- 对他人表现出同理心

---

## 💡 如何贡献

### 报告 Bug

如果你发现了 bug，请：

1. 检查 [Issues](https://github.com/yourusername/MonoFocus/issues) 是否已有相关报告
2. 如果没有，创建新 Issue，包含：
   - **标题**: 简洁描述问题
   - **环境**: 操作系统、版本号
   - **复现步骤**: 详细说明如何触发 bug
   - **期望行为**: 你期望发生什么
   - **实际行为**: 实际发生了什么
   - **截图/日志**: 如有可能，附上相关信息

### 建议新功能

1. 先在 [Discussions](https://github.com/yourusername/MonoFocus/discussions) 讨论
2. 说明功能的用途和好处
3. 如果得到认可，创建 Feature Request Issue

### 贡献代码

1. **Fork 仓库**
   ```bash
   # 在 GitHub 上点击 Fork 按钮
   ```

2. **克隆你的 Fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/MonoFocus.git
   cd MonoFocus
   ```

3. **创建特性分支**
   ```bash
   git checkout -b feature/amazing-feature
   ```

4. **进行更改**
   - 编写代码
   - 添加测试
   - 更新文档

5. **运行测试**
   ```bash
   cargo test --manifest-path=src-tauri/Cargo.toml
   ```

6. **提交更改**
   ```bash
   git add .
   git commit -m "feat: add amazing feature"
   ```

7. **推送到你的 Fork**
   ```bash
   git push origin feature/amazing-feature
   ```

8. **创建 Pull Request**
   - 在 GitHub 上打开 Pull Request
   - 清楚描述你的更改
   - 链接相关 Issue

---

## 🔧 开发流程

### 环境设置

详细说明请参考 [BUILD.md](BUILD.md)

```bash
# 安装依赖
npm install

# 启动开发模式
npm run tauri dev
```

### 项目结构

```
src-tauri/src/
├── main.rs          # 入口，Tauri 命令定义
├── monitor.rs       # 显示器检测
├── mouse_watcher.rs # 鼠标监听
├── overlay.rs       # 遮罩窗口管理
├── config.rs        # 配置管理
└── tray.rs          # 系统托盘

src/
├── index.html       # 主界面
├── styles.css       # 样式
├── main.js          # 前端逻辑
└── overlay.html     # 遮罩页面
```

### 开发工作流

1. **选择任务**
   - 查看 [Issues](https://github.com/yourusername/MonoFocus/issues)
   - 选择标记为 `good first issue` 的任务（新手友好）

2. **编写代码**
   - 遵循现有代码风格
   - 添加必要的注释
   - 保持函数简洁

3. **测试**
   ```bash
   # Rust 测试
   cargo test --manifest-path=src-tauri/Cargo.toml
   
   # 手动测试
   npm run tauri dev
   ```

4. **格式化**
   ```bash
   # Rust
   cargo fmt --manifest-path=src-tauri/Cargo.toml
   
   # JavaScript
   npm run format
   ```

5. **检查 Lints**
   ```bash
   cargo clippy --manifest-path=src-tauri/Cargo.toml
   ```

---

## 📏 代码规范

### Rust

遵循官方 [Rust 风格指南](https://doc.rust-lang.org/1.0.0/style/)

**关键点**：
- 使用 4 空格缩进
- 变量名使用 `snake_case`
- 类型名使用 `PascalCase`
- 常量使用 `SCREAMING_SNAKE_CASE`
- 每行最多 100 字符
- 添加文档注释 `///`

**示例**：
```rust
/// 获取所有显示器信息
///
/// # Returns
///
/// 返回包含所有显示器信息的向量
pub fn get_monitors() -> Vec<MonitorInfo> {
    // 实现...
}
```

### JavaScript

遵循 Airbnb JavaScript 风格指南

**关键点**：
- 使用 2 空格缩进
- 使用分号
- 使用 `const` / `let`，避免 `var`
- 优先使用箭头函数
- 使用模板字符串

**示例**：
```javascript
const drawMonitors = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  // 实现...
};
```

### CSS

**关键点**：
- 使用 2 空格缩进
- 类名使用 `kebab-case`
- 按字母顺序排列属性
- 使用简写属性

**示例**：
```css
.control-panel {
  background: white;
  border-radius: 10px;
  display: flex;
  padding: 16px 20px;
}
```

---

## 📝 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/)

### 格式

```
<类型>(<范围>): <描述>

[可选的正文]

[可选的脚注]
```

### 类型

| 类型 | 说明 |
|------|------|
| `feat` | 新功能 |
| `fix` | 修复 bug |
| `docs` | 文档更新 |
| `style` | 代码格式（不影响功能） |
| `refactor` | 重构（既不是新功能也不是 bug 修复） |
| `perf` | 性能优化 |
| `test` | 添加测试 |
| `chore` | 构建/工具链更新 |
| `ci` | CI/CD 配置 |
| `revert` | 回滚提交 |

### 范围（可选）

- `monitor` - 显示器检测
- `mouse` - 鼠标监听
- `overlay` - 遮罩窗口
- `config` - 配置管理
- `tray` - 系统托盘
- `ui` - 前端界面
- `docs` - 文档

### 示例

```
feat(overlay): 添加淡入淡出动画

为遮罩窗口添加平滑的显示/隐藏动画，提升用户体验。

动画时长: 200ms
缓动函数: ease-in-out

Closes #123
```

```
fix(mouse): 修复 Linux 下鼠标位置获取错误

在 Wayland 环境下无法正确获取鼠标位置，
改为使用 wl_pointer 协议。

Fixes #456
```

---

## 🐛 问题反馈

### 创建 Issue

使用我们的 Issue 模板：

**Bug Report**:
```markdown
## 描述
简洁清晰地描述问题

## 复现步骤
1. 打开...
2. 点击...
3. 看到错误

## 期望行为
应该发生什么

## 实际行为
实际发生了什么

## 环境
- OS: [e.g. Windows 11]
- 版本: [e.g. 1.0.0]

## 截图
如有可能，附上截图
```

**Feature Request**:
```markdown
## 功能描述
清晰描述你想要的功能

## 用例
这个功能解决什么问题？

## 替代方案
你考虑过的其他解决方案

## 附加信息
其他相关信息
```

---

## ✅ Pull Request 检查清单

提交 PR 前，确保：

- [ ] 代码遵循项目风格规范
- [ ] 所有测试通过
- [ ] 添加了新功能的测试
- [ ] 更新了相关文档
- [ ] Commit 消息遵循规范
- [ ] 没有引入新的警告
- [ ] PR 描述清晰，链接了相关 Issue

---

## 🎓 资源

- [Tauri 文档](https://tauri.app/)
- [Rust 官方书](https://doc.rust-lang.org/book/)
- [MDN Web 文档](https://developer.mozilla.org/)
- [Git 教程](https://git-scm.com/book/zh/v2)

---

## 🙏 致谢

感谢所有贡献者！你们的努力让 MonoFocus 变得更好。

查看完整贡献者列表：[Contributors](https://github.com/yourusername/MonoFocus/graphs/contributors)

---

## 💬 获取帮助

如有问题：

- 📧 Email: support@monofocus.example
- 💬 Discussions: [GitHub Discussions](https://github.com/yourusername/MonoFocus/discussions)
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/MonoFocus/issues)

---

<div align="center">

**再次感谢你的贡献！❤️**

</div>

