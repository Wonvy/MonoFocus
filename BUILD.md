# MonoFocus 构建指南

## 📋 目录

- [快速开始](#快速开始)
- [环境配置](#环境配置)
- [开发模式](#开发模式)
- [生产构建](#生产构建)
- [常见问题](#常见问题)

---

## 🚀 快速开始

```bash
# 1. 克隆仓库
git clone https://github.com/yourusername/MonoFocus.git
cd MonoFocus

# 2. 安装依赖
npm install

# 3. 运行开发模式
npm run tauri dev

# 4. 构建生产版本
npm run tauri build
```

---

## 🛠 环境配置

### 通用要求

1. **Rust** (1.70+)
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 验证安装
   rustc --version
   cargo --version
   ```

2. **Node.js** (16+)
   ```bash
   # 推荐使用 nvm
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 16
   nvm use 16
   
   # 验证安装
   node --version
   npm --version
   ```

### Windows 配置

#### 必需软件

1. **Visual Studio 2019 或更高版本**
   - 下载 [Visual Studio Community](https://visualstudio.microsoft.com/downloads/)
   - 安装时选择 "使用 C++ 的桌面开发"

2. **WebView2**
   - Windows 10/11 已预装
   - 如需手动安装：[下载 WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

#### 验证环境

```powershell
# 检查 MSVC 编译器
cl

# 检查 Rust Windows 工具链
rustup show
```

### macOS 配置

#### 必需软件

1. **Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **验证安装**
   ```bash
   xcode-select -p
   # 应输出: /Library/Developer/CommandLineTools
   ```

#### 可选：完整 Xcode

如果需要调试或更多功能：
```bash
# 从 App Store 安装 Xcode
# 然后运行
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### Linux 配置

#### Ubuntu / Debian

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libx11-dev \
    libxrandr-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev
```

#### Fedora / RHEL

```bash
sudo dnf install -y \
    webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libX11-devel \
    libXrandr-devel
```

#### Arch Linux

```bash
sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libx11 \
    libxrandr
```

---

## 🔧 开发模式

### 启动开发服务器

```bash
npm run tauri dev
```

特性：
- ✅ 前端热重载（HTML/CSS/JS 修改即时生效）
- ✅ Rust 自动重编译（保存 `.rs` 文件后自动构建）
- ✅ 实时日志输出
- ✅ DevTools 支持（按 F12 打开）

### 开发时的目录结构

```
MonoFocus/
├── src/                    # 前端源码（开发时修改这里）
│   ├── index.html
│   ├── styles.css
│   ├── main.js
│   └── overlay.html
├── src-tauri/              # Rust 后端（开发时修改这里）
│   ├── src/
│   │   ├── main.rs
│   │   ├── monitor.rs
│   │   ├── mouse_watcher.rs
│   │   ├── overlay.rs
│   │   ├── config.rs
│   │   └── tray.rs
│   └── target/             # 编译输出（自动生成）
└── node_modules/           # npm 依赖（自动生成）
```

### 调试技巧

#### 启用详细日志

```bash
# Windows (PowerShell)
$env:RUST_LOG="debug"
npm run tauri dev

# macOS / Linux
RUST_LOG=debug npm run tauri dev
```

#### 查看 Rust 日志

在 Rust 代码中使用：
```rust
println!("Debug info: {:?}", variable);
```

#### 查看前端日志

在浏览器 DevTools 的 Console 标签中查看 `console.log` 输出。

---

## 📦 生产构建

### 构建命令

```bash
npm run tauri build
```

### 构建时间

| 平台 | 首次构建 | 增量构建 |
|------|----------|----------|
| Windows | ~15 分钟 | ~2 分钟 |
| macOS | ~12 分钟 | ~2 分钟 |
| Linux | ~10 分钟 | ~2 分钟 |

*实际时间取决于硬件配置*

### 构建输出位置

#### Windows

```
src-tauri/target/release/bundle/
├── msi/
│   └── MonoFocus_1.0.0_x64.msi          # 主安装包
└── nsis/
    └── MonoFocus_1.0.0_x64-setup.exe    # 备选安装包
```

#### macOS

```
src-tauri/target/release/bundle/
├── dmg/
│   └── MonoFocus_1.0.0_x64.dmg          # 磁盘镜像
└── macos/
    └── MonoFocus.app                     # 应用包
```

#### Linux

```
src-tauri/target/release/bundle/
├── appimage/
│   └── MonoFocus_1.0.0_amd64.AppImage   # 通用包
├── deb/
│   └── MonoFocus_1.0.0_amd64.deb        # Debian/Ubuntu
└── rpm/
    └── MonoFocus-1.0.0-1.x86_64.rpm     # Fedora/RHEL
```

### 构建优化

#### 减小二进制大小

在 `src-tauri/Cargo.toml` 中添加：

```toml
[profile.release]
opt-level = "z"       # 优化大小
lto = true            # 链接时优化
codegen-units = 1     # 更好的优化
strip = true          # 移除符号
panic = "abort"       # 减小 panic 处理代码
```

#### 构建特定平台

```bash
# 仅构建可执行文件（不打包）
cargo build --release --manifest-path=src-tauri/Cargo.toml

# 仅打包（假设已编译）
npm run tauri build -- --no-bundle
```

---

## 🧪 测试

### 运行测试

```bash
# Rust 单元测试
cargo test --manifest-path=src-tauri/Cargo.toml

# 显示测试输出
cargo test --manifest-path=src-tauri/Cargo.toml -- --nocapture

# 测试特定模块
cargo test --manifest-path=src-tauri/Cargo.toml monitor::tests
```

### 添加测试

在模块末尾添加测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

---

## 🎨 图标生成

### 准备图标

1. 创建一个 **1024x1024** 的 PNG 图标
2. 确保背景透明
3. 使用简洁的设计

### 自动生成所有尺寸

```bash
# 假设你的图标在 assets/icon.png
npm run tauri icon assets/icon.png
```

这将生成：
- `icons/icon.icns` (macOS)
- `icons/icon.ico` (Windows)
- `icons/32x32.png`
- `icons/128x128.png`
- `icons/128x128@2x.png`
- `icons/icon.png` (Linux)

---

## ❓ 常见问题

### Windows

**问题**: 编译失败，提示找不到 MSVC

**解决方案**:
1. 安装 Visual Studio with C++ Build Tools
2. 确保 `cl.exe` 在 PATH 中
3. 重启终端

---

**问题**: WebView2 未安装

**解决方案**:
```powershell
# 下载并安装
winget install Microsoft.EdgeWebView2Runtime
```

---

### macOS

**问题**: 签名失败

**解决方案**:
```bash
# 临时禁用签名（仅用于开发）
export TAURI_SKIP_DEVSERVER_CHECK=true
npm run tauri build -- --no-bundle
```

---

**问题**: 权限被拒绝

**解决方案**:
```bash
# 授予可执行权限
chmod +x src-tauri/target/release/monofocus
```

---

### Linux

**问题**: 缺少依赖库

**解决方案**:
```bash
# 检查缺少的库
ldd src-tauri/target/release/monofocus

# 安装缺失的库（示例）
sudo apt install libwebkit2gtk-4.0-37
```

---

**问题**: AppImage 无法运行

**解决方案**:
```bash
# 安装 FUSE（AppImage 依赖）
sudo apt install fuse libfuse2

# 授予执行权限
chmod +x MonoFocus_1.0.0_amd64.AppImage
```

---

### 通用问题

**问题**: 编译速度太慢

**解决方案**:
```bash
# 使用 sccache 缓存编译结果
cargo install sccache
export RUSTC_WRAPPER=sccache

# 增加并行编译任务数
export CARGO_BUILD_JOBS=8
```

---

**问题**: 依赖更新冲突

**解决方案**:
```bash
# 清理所有依赖
rm -rf node_modules
rm -rf src-tauri/target
rm package-lock.json

# 重新安装
npm install
```

---

## 📚 更多资源

- [Tauri 构建指南](https://tauri.app/v1/guides/building/)
- [Rust 编译优化](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [跨平台打包](https://tauri.app/v1/guides/building/cross-platform)

---

## 🆘 获取帮助

如果遇到问题：

1. 查看 [GitHub Issues](https://github.com/yourusername/MonoFocus/issues)
2. 搜索 [Tauri Discussions](https://github.com/tauri-apps/tauri/discussions)
3. 提交新的 Issue（附带详细错误信息）

---

<div align="center">

**祝构建顺利！🎉**

</div>

