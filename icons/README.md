# 图标资源说明

## 需要的图标文件

请将以下图标文件放置在此目录：

### Windows
- `icon.ico` - Windows 应用图标（256x256，包含多个尺寸）

### macOS
- `icon.icns` - macOS 应用图标

### Linux
- `icon.png` - 通用 PNG 图标（512x512）

### 多尺寸 PNG（用于不同场景）
- `32x32.png`
- `128x128.png`
- `128x128@2x.png` (256x256)

## 图标设计建议

- **主题**: 眼睛、显示器、专注等元素
- **风格**: 简约、现代、扁平化
- **配色**: 使用应用主色调 #667eea - #764ba2
- **背景**: 透明背景（PNG）

## 生成工具

可以使用以下在线工具生成多平台图标：
- [IconKitchen](https://icon.kitchen/)
- [AppIcon](https://appicon.co/)
- [Tauri Icon Generator](https://tauri.app/v1/guides/features/icons)

## 快速生成命令

如果你有一个 1024x1024 的 PNG 图标，可以使用 Tauri CLI 生成所有尺寸：

```bash
npm run tauri icon path/to/your/icon.png
```

这将自动生成所有需要的图标文件。

