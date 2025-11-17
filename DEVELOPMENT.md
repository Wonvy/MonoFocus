# MonoFocus 开发文档

## 📋 目录

- [技术架构](#技术架构)
- [模块详解](#模块详解)
- [开发环境设置](#开发环境设置)
- [构建与测试](#构建与测试)
- [跨平台实现细节](#跨平台实现细节)
- [贡献指南](#贡献指南)

---

## 🏗 技术架构

### 整体架构

MonoFocus 采用 **Rust + Tauri** 架构，将系统级能力与现代 Web UI 完美结合。

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (Web)                       │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Canvas: 显示器布局可视化                         │  │
│  │  Slider: 透明度调节 (0-80%)                      │  │
│  │  Toggle: 护眼模式开关                            │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                          ↕ IPC (Tauri invoke)
┌─────────────────────────────────────────────────────────┐
│                    Backend (Rust)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ monitor.rs   │  │ mouse_       │  │ overlay.rs   │ │
│  │ - 检测显示器 │  │ watcher.rs   │  │ - 遮罩窗口   │ │
│  │ - 计算布局   │  │ - 鼠标位置   │  │ - 透明度控制 │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│  ┌──────────────┐  ┌──────────────┐                   │
│  │ config.rs    │  │ tray.rs      │                   │
│  │ - 配置管理   │  │ - 系统托盘   │                   │
│  └──────────────┘  └──────────────┘                   │
└─────────────────────────────────────────────────────────┘
                          ↕
┌─────────────────────────────────────────────────────────┐
│              Platform APIs (Win/Mac/Linux)              │
│  - Display enumeration                                  │
│  - Mouse position tracking                              │
│  - Window management                                    │
└─────────────────────────────────────────────────────────┘
```

### 技术栈

| 层级 | 技术 | 版本 | 用途 |
|------|------|------|------|
| 后端 | Rust | 1.70+ | 系统级操作、性能关键路径 |
| 框架 | Tauri | 1.5+ | 跨平台桌面应用框架 |
| 前端 | HTML5/CSS3/JS | ES2020+ | UI 界面 |
| 配置 | JSON | - | 持久化存储 |

---

## 📦 模块详解

### 1. `monitor.rs` - 显示器检测模块

#### 数据结构

```rust
pub struct MonitorInfo {
    pub id: String,              // 唯一标识
    pub x: i32,                  // 虚拟桌面 X 坐标
    pub y: i32,                  // 虚拟桌面 Y 坐标
    pub width: i32,              // 逻辑宽度（像素）
    pub height: i32,             // 逻辑高度（像素）
    pub physical_width_mm: Option<f32>,  // 物理宽度（毫米）
    pub physical_height_mm: Option<f32>, // 物理高度（毫米）
    pub scale_factor: f32,       // HiDPI 缩放比例
}
```

#### 核心函数

**`get_monitors() -> Vec<MonitorInfo>`**

获取所有已连接的显示器信息。

- **Windows**: 使用 `EnumDisplayMonitors` + `GetMonitorInfoW`
- **macOS**: 使用 `CGDisplay::active_displays` + `CGDisplay::bounds`
- **Linux**: 使用 X11 `XRRGetScreenResourcesCurrent` + `XRRGetOutputInfo`

**`normalize_layout(monitors, container_w, container_h) -> Vec<UIRect>`**

将真实显示器坐标映射到 UI 容器坐标。

算法步骤：
1. 计算所有显示器的包络盒（min_x, max_x, min_y, max_y）
2. 计算缩放比例：`scale = min(container_w / total_w, container_h / total_h)`
3. 为每个显示器计算 UI 坐标：
   ```
   ui_x = (monitor.x - min_x) * scale + margin
   ui_y = (monitor.y - min_y) * scale + margin
   ui_w = monitor.width * scale
   ui_h = monitor.height * scale
   ```

#### 平台差异

| 功能 | Windows | macOS | Linux |
|------|---------|-------|-------|
| 显示器枚举 | ✅ | ✅ | ✅ |
| 物理尺寸 | ❌ | ✅ | ✅ |
| DPI 缩放 | 🔶 简化 | ✅ | ✅ |

---

### 2. `mouse_watcher.rs` - 鼠标监听模块

#### 核心函数

**`get_mouse_position() -> Option<MousePosition>`**

获取鼠标全局坐标。

- **Windows**: `GetCursorPos`
- **macOS**: `CGEvent::location`
- **Linux**: `XQueryPointer`

**`find_monitor_at_position(monitors, pos) -> Option<String>`**

判断鼠标在哪个显示器上（矩形碰撞检测）。

**`start_mouse_watcher<F>(callback: F)`**

启动后台线程，每 100ms 检测一次鼠标位置，当显示器变化时触发回调。

#### 性能优化

- 使用 100ms 轮询间隔（平衡响应速度与 CPU 占用）
- 仅在显示器 ID 变化时触发回调（避免重复通知）
- 轮询线程独立运行，不阻塞主线程

---

### 3. `overlay.rs` - 遮罩窗口模块

#### 核心结构

```rust
pub struct OverlayManager {
    app: AppHandle,
    overlays: Arc<Mutex<HashMap<String, Window>>>,  // monitor_id -> Window
    config: Arc<Mutex<OverlayConfig>>,
}

pub struct OverlayConfig {
    pub opacity: f32,  // 0.0 - 0.8
    pub enabled: bool,
}
```

#### 关键方法

**`create_overlay(monitor: &MonitorInfo) -> Result<Window>`**

为指定显示器创建遮罩窗口：
1. 创建无边框、透明窗口
2. 设置大小和位置与显示器一致
3. 设置 `always_on_top = true`
4. 配置点击穿透（platform-specific）
5. 设置初始透明度

**`update_overlays(monitors, active_monitor_id)`**

更新所有遮罩层：
- 隐藏活跃显示器的遮罩
- 显示/创建非活跃显示器的遮罩
- 如果护眼模式关闭，隐藏所有遮罩

#### 点击穿透实现

| 平台 | 实现方式 |
|------|----------|
| Windows | `SetWindowLongPtrW` + `WS_EX_TRANSPARENT` |
| macOS | `NSWindow::setIgnoresMouseEvents_(YES)` |
| Linux | 设置窗口类型为 `_NET_WM_WINDOW_TYPE_DOCK` |

#### 透明度控制

通过在遮罩窗口中执行 JavaScript 动态修改背景色：
```javascript
document.body.style.backgroundColor = 'rgba(0, 0, 0, {opacity})'
```

---

### 4. `config.rs` - 配置管理模块

#### 配置文件位置

- **Windows**: `%APPDATA%\MonoFocus\config.json`
- **macOS**: `~/Library/Application Support/MonoFocus/config.json`
- **Linux**: `~/.config/MonoFocus/config.json`

#### 配置结构

```json
{
  "opacity": 0.6,
  "enabled": true,
  "auto_start": false,
  "theme": "auto"
}
```

#### 开机自启动实现

**Windows**:
- 写入注册表：`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
- 使用 `reg add` 命令

**macOS**:
- 创建 LaunchAgent plist 文件
- 路径：`~/Library/LaunchAgents/com.monofocus.app.plist`

**Linux**:
- 创建 `.desktop` 文件
- 路径：`~/.config/autostart/monofocus.desktop`

---

### 5. `tray.rs` - 系统托盘模块

#### 托盘菜单

```
┌─────────────────────┐
│ Disable Shield      │  ← 切换护眼模式
├─────────────────────┤
│ Settings            │  ← 显示主窗口
├─────────────────────┤
│ Exit                │  ← 退出应用
└─────────────────────┘
```

#### 事件处理

- **左键点击**: 显示主窗口
- **菜单项点击**: 触发对应操作
- **动态更新**: 根据护眼模式状态更新菜单文本

---

## 🛠 开发环境设置

### 前置要求

1. **Rust**（1.70+）
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js**（16+）
   ```bash
   # 使用 nvm
   nvm install 16
   nvm use 16
   ```

3. **Tauri CLI**
   ```bash
   npm install -g @tauri-apps/cli
   ```

### 平台特定依赖

#### Windows
```bash
# Visual Studio 2019+ with C++ Build Tools
# WebView2 Runtime (Windows 10/11 自带)
```

#### macOS
```bash
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libx11-dev \
    libxrandr-dev
```

### 克隆与安装

```bash
git clone https://github.com/yourusername/MonoFocus.git
cd MonoFocus
npm install
```

---

## 🔨 构建与测试

### 开发模式

```bash
# 启动开发服务器（带热重载）
npm run tauri dev
```

特性：
- 前端热重载
- Rust 代码自动重编译
- 实时日志输出

### 构建生产版本

```bash
npm run tauri build
```

输出位置：
- **Windows**: `src-tauri/target/release/bundle/msi/MonoFocus_1.0.0_x64.msi`
- **macOS**: `src-tauri/target/release/bundle/dmg/MonoFocus_1.0.0_x64.dmg`
- **Linux**: `src-tauri/target/release/bundle/appimage/MonoFocus_1.0.0_amd64.AppImage`

### 运行测试

```bash
# Rust 单元测试
cargo test --manifest-path=src-tauri/Cargo.toml

# 带输出的测试
cargo test --manifest-path=src-tauri/Cargo.toml -- --nocapture

# 特定模块测试
cargo test --manifest-path=src-tauri/Cargo.toml monitor::tests
```

### 代码格式化

```bash
# Rust
cargo fmt --manifest-path=src-tauri/Cargo.toml

# JavaScript
npm run format
```

---

## 🌍 跨平台实现细节

### 条件编译

Rust 使用 `cfg` 属性进行平台特定代码：

```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows 实现
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS 实现
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux 实现
}
```

### 依赖管理

在 `Cargo.toml` 中按平台引入依赖：

```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.51", features = [...] }

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
core-graphics = "0.23"

[target.'cfg(target_os = "linux")'.dependencies]
x11 = { version = "2.21", features = ["xlib", "xrandr"] }
```

### 已知平台差异

| 功能 | Windows | macOS | Linux | 解决方案 |
|------|---------|-------|-------|----------|
| 物理尺寸获取 | ❌ | ✅ | ✅ | 回退到分辨率比例 |
| 点击穿透 | ✅ | ✅ | 🔶 部分支持 | X11 可用，Wayland 受限 |
| 系统托盘图标 | ✅ | ✅ | 🔶 依赖桌面环境 | 提供回退方案 |
| 开机自启动 | ✅ | ✅ | ✅ | 平台特定实现 |

---

## 🎨 UI 设计规范

### 显示器布局可视化

**设计原则**：
- ✅ **比例准确**：根据物理尺寸或分辨率计算相对大小
- ✅ **位置还原**：保持系统中的显示器排列关系
- ✅ **极简风格**：无文字标签，纯视觉化
- ✅ **当前高亮**：用渐变色和阴影标识鼠标所在显示器

**实现细节**：
- Canvas 尺寸：460x200px
- 边距：20px
- 活跃显示器颜色：`#667eea`（渐变到 `#764ba2`）
- 非活跃显示器颜色：`#e0e0e0`
- 高亮效果：3px 边框 + 10px 模糊阴影

### 配色方案

```css
Primary: #667eea → #764ba2 (渐变)
Background: #f8f9fa
Text: #333
Secondary: #888
Border: #e0e0e0
```

---

## 🤝 贡献指南

### 提交规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/)：

```
feat: 添加新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式（不影响逻辑）
refactor: 重构
test: 添加测试
chore: 构建/工具链更新
```

示例：
```
feat(overlay): 添加动画过渡效果
fix(monitor): 修复 Linux 下多显示器检测问题
docs: 更新 README 安装说明
```

### Pull Request 流程

1. Fork 仓库
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'feat: add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 提交 PR，描述清楚更改内容

### 代码审查标准

- ✅ 通过所有单元测试
- ✅ 代码格式符合规范（`cargo fmt`）
- ✅ 无 Clippy 警告（`cargo clippy`）
- ✅ 添加必要的注释和文档
- ✅ 跨平台兼容性测试

---

## 🐛 调试技巧

### 启用详细日志

```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"
npm run tauri dev

# macOS/Linux
RUST_LOG=debug npm run tauri dev
```

### 调试前端

1. 开发模式下按 `F12` 打开 DevTools
2. 使用 `console.log` 输出调试信息
3. 在 Network 标签查看 Tauri IPC 调用

### 调试 Rust

```rust
// 使用 dbg! 宏
dbg!(&monitors);

// 使用 println!
println!("Monitor count: {}", monitors.len());

// 条件断言
assert_eq!(monitors.len(), 2);
```

---

## 📚 参考资源

- [Tauri 官方文档](https://tauri.app/)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Windows API 文档](https://docs.microsoft.com/en-us/windows/win32/api/)
- [macOS Core Graphics](https://developer.apple.com/documentation/coregraphics)
- [X11 编程手册](https://www.x.org/releases/current/doc/)

---

## 📝 待办事项

- [ ] 添加遮罩淡入淡出动画
- [ ] 支持自定义遮罩颜色
- [ ] 添加键盘快捷键支持
- [ ] 实现 Wayland 完整支持
- [ ] 添加更多单元测试
- [ ] 支持多语言界面

---

<div align="center">

**Happy Coding! 🚀**

如有问题，请提交 [Issue](https://github.com/yourusername/MonoFocus/issues)

</div>

