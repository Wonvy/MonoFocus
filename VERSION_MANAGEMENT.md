# 版本管理指南

MonoFocus 使用自动化脚本统一管理版本号，确保所有文件中的版本号保持一致。

## 📦 版本号位置

版本号会自动同步到以下位置：
1. `package.json` - 主版本号（单一真实来源）
2. `src-tauri/Cargo.toml` - Rust 包版本
3. `src-tauri/tauri.conf.json` - Tauri 配置和窗口标题
4. `src/index.html` - 页面底部显示的版本号

## 🚀 发布新版本

### 方法 1：使用 npm version 命令（推荐）

```bash
# 修复版本（1.2.0 → 1.2.1）
npm version patch

# 次要版本（1.2.0 → 1.3.0）
npm version minor

# 主要版本（1.2.0 → 2.0.0）
npm version major

# 指定版本
npm version 1.5.0
```

**这个命令会自动：**
1. ✅ 更新 `package.json` 版本号
2. ✅ 运行 `scripts/update-version.js` 同步所有文件
3. ✅ 提交更改到 Git
4. ✅ 创建版本标签（如 `v1.3.0`）
5. ✅ 推送到 GitHub
6. ✅ 触发 GitHub Actions 自动构建和发布

### 方法 2：手动更新（不推荐）

如果需要手动更新：

```bash
# 1. 编辑 package.json 中的版本号
# 2. 运行同步脚本
npm run version

# 3. 提交更改
git add -A
git commit -m "chore: bump version to x.x.x"

# 4. 创建标签并推送
git tag vx.x.x
git push && git push --tags
```

## 📝 更新 CHANGELOG

在发布新版本前，记得更新 `CHANGELOG.md`：

```markdown
## [1.3.0] - 2025-11-19

### 新增 / Added
- 新功能描述

### 优化 / Improved
- 改进描述

### 修复 / Fixed
- Bug 修复描述
```

## 🔄 完整发布流程

```bash
# 1. 确保在 main 分支且代码是最新的
git checkout main
git pull

# 2. 更新 CHANGELOG.md
# 编辑 CHANGELOG.md，添加新版本的更新内容

# 3. 提交 CHANGELOG
git add CHANGELOG.md
git commit -m "docs: update changelog for vx.x.x"

# 4. 使用 npm version 发布
npm version minor  # 或 patch/major

# 5. 等待 GitHub Actions 完成构建
# 访问 https://github.com/Wonvy/MonoFocus/actions

# 6. 检查发布
# 访问 https://github.com/Wonvy/MonoFocus/releases
```

## 🎯 示例

发布 v1.3.0 版本：

```bash
# 更新 CHANGELOG
vim CHANGELOG.md

# 提交 CHANGELOG
git add CHANGELOG.md
git commit -m "docs: update changelog for v1.3.0"

# 自动发布
npm version minor

# ✨ 完成！GitHub Actions 会自动构建和发布
```

## ⚠️ 注意事项

1. **只在 main 分支发布** - 确保你在 main 分支上
2. **先提交代码** - 使用 `npm version` 前确保没有未提交的更改
3. **网络连接** - 推送标签需要稳定的网络连接
4. **等待构建** - GitHub Actions 需要 10-15 分钟完成构建

## 🔧 脚本说明

### `scripts/update-version.js`

自动同步版本号到所有需要的文件。

**更新内容：**
- ✅ `src/index.html` - 页面底部版本号
- ✅ `src-tauri/tauri.conf.json` - 包版本和窗口标题
- ✅ `src-tauri/Cargo.toml` - Rust 包版本

**使用：**
```bash
npm run version
```

### `package.json` scripts

```json
{
  "version": "node scripts/update-version.js",
  "postversion": "git add -A && git commit -m \"...\" && git tag ... && git push ..."
}
```

- `version` - 在 `npm version` 后自动运行，同步版本号
- `postversion` - 提交、打标签、推送

## 🐛 故障排除

### 版本号不一致

如果发现版本号不一致：

```bash
npm run version
```

### 推送失败

如果 `npm version` 推送失败：

```bash
# 手动推送
git push origin main
git push origin vx.x.x
```

### 构建失败

如果 GitHub Actions 构建失败：
1. 查看构建日志：https://github.com/Wonvy/MonoFocus/actions
2. 修复问题后重新推送标签：
```bash
git tag -d vx.x.x
git push origin :refs/tags/vx.x.x
git tag vx.x.x
git push origin vx.x.x
```

