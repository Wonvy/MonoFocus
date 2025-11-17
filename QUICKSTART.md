# MonoFocus 快速入门指南

## 🚀 5 分钟上手

### 第一步：安装依赖

```bash
# 确保已安装 Rust
rustc --version  # 应显示 1.70+

# 确保已安装 Node.js
node --version   # 应显示 16+

# 安装项目依赖
npm install
```

### 第二步：运行开发模式

```bash
npm run tauri dev
```

首次运行会编译 Rust 代码，大约需要 5-10 分钟。后续启动只需几秒钟。

### 第三步：测试功能

1. **查看显示器布局**
   - 主窗口中会显示所有检测到的显示器

2. **移动鼠标**
   - 在不同显示器间移动鼠标
   - 观察非活跃显示器上的半透明遮罩

3. **调整透明度**
   - 拖动透明度滑条（0-80%）
   - 实时预览效果

4. **测试托盘**
   - 关闭主窗口（最小化到托盘）
   - 右键点击托盘图标
   - 尝试 Enable/Disable Shield

---

## 📁 关键文件说明

```
MonoFocus/
├── src/                      # 前端代码
│   ├── index.html           # 主界面（修改这里调整 UI）
│   ├── styles.css           # 样式（修改这里调整外观）
│   └── main.js              # 逻辑（修改这里调整前端行为）
│
├── src-tauri/src/           # Rust 后端
│   ├── main.rs              # 入口（Tauri 命令定义）
│   ├── monitor.rs           # 显示器检测（添加新显示器功能在这）
│   ├── mouse_watcher.rs     # 鼠标监听（调整轮询间隔在这）
│   ├── overlay.rs           # 遮罩窗口（修改遮罩行为在这）
│   ├── config.rs            # 配置管理（添加新配置项在这）
│   └── tray.rs              # 系统托盘（修改托盘菜单在这）
│
├── README.md                # 项目介绍
├── DEVELOPMENT.md           # 详细开发文档
├── BUILD.md                 # 构建指南
└── CONTRIBUTING.md          # 贡献指南
```

---

## 🎯 常见开发任务

### 修改主窗口 UI

编辑 `src/index.html` 和 `src/styles.css`：

```html
<!-- src/index.html -->
<div class="control-item">
  <div class="control-label">
    <span class="icon">🎨</span>
    <span>新功能</span>
  </div>
  <button id="myButton">点击我</button>
</div>
```

```css
/* src/styles.css */
#myButton {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  padding: 8px 16px;
  cursor: pointer;
}
```

保存后，前端会自动热重载。

### 添加 Tauri 命令

在 `src-tauri/src/main.rs` 中添加：

```rust
#[tauri::command]
fn my_new_command(param: String) -> String {
    format!("You sent: {}", param)
}

// 在 main() 中注册
.invoke_handler(tauri::generate_handler![
    // ... 其他命令
    my_new_command,
])
```

在前端调用：

```javascript
// src/main.js
import { invoke } from "@tauri-apps/api/tauri";

const result = await invoke("my_new_command", { param: "Hello" });
console.log(result); // "You sent: Hello"
```

### 修改遮罩透明度默认值

编辑 `src-tauri/src/config.rs`：

```rust
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            opacity: 0.7,  // 改为 70%
            // ...
        }
    }
}
```

### 调整鼠标轮询间隔

编辑 `src-tauri/src/mouse_watcher.rs`：

```rust
pub fn start_mouse_watcher<F>(callback: F) -> thread::JoinHandle<()> {
    // ...
    loop {
        // ...
        thread::sleep(Duration::from_millis(50)); // 改为 50ms
    }
}
```

---

## 🔍 调试技巧

### 查看 Rust 日志

```bash
# 启用详细日志
RUST_LOG=debug npm run tauri dev
```

在代码中输出日志：

```rust
println!("Debug: {:?}", variable);
dbg!(&variable);
```

### 查看前端日志

按 `F12` 打开 DevTools，在 Console 中查看：

```javascript
console.log("Debug:", value);
console.table(monitors); // 表格形式
```

### 检查遮罩窗口

遮罩窗口默认没有 DevTools，如需调试：

在 `src-tauri/src/overlay.rs` 中临时添加：

```rust
.build()?;

// 临时启用 DevTools
#[cfg(debug_assertions)]
window.open_devtools();
```

---

## 🧪 测试

### 运行所有测试

```bash
cargo test --manifest-path=src-tauri/Cargo.toml
```

### 测试特定模块

```bash
# 只测试 monitor 模块
cargo test --manifest-path=src-tauri/Cargo.toml monitor::tests

# 显示输出
cargo test --manifest-path=src-tauri/Cargo.toml -- --nocapture
```

### 添加新测试

在模块末尾：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }
}
```

---

## 📦 构建安装包

```bash
# 构建生产版本
npm run tauri build

# 输出在 src-tauri/target/release/bundle/
```

第一次构建会比较慢（10-15 分钟），后续构建快很多。

---

## ❓ 遇到问题？

### 编译错误

```bash
# 清理并重新编译
rm -rf src-tauri/target
npm run tauri dev
```

### 依赖问题

```bash
# 重新安装依赖
rm -rf node_modules package-lock.json
npm install
```

### 平台特定问题

查看 [BUILD.md](BUILD.md) 的"常见问题"章节。

---

## 📚 下一步

- 阅读 [DEVELOPMENT.md](DEVELOPMENT.md) 了解详细架构
- 查看 [Issues](https://github.com/yourusername/MonoFocus/issues) 寻找任务
- 阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 学习如何贡献

---

## 🎓 学习资源

- [Tauri 快速入门](https://tauri.app/v1/guides/getting-started/setup)
- [Rust 官方书](https://doc.rust-lang.org/book/)
- [JavaScript MDN](https://developer.mozilla.org/zh-CN/)

---

<div align="center">

**祝开发愉快！🎉**

有问题？提交 [Issue](https://github.com/yourusername/MonoFocus/issues) 或加入 [讨论](https://github.com/yourusername/MonoFocus/discussions)

</div>

