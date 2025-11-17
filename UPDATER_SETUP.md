# 配置自动更新签名

本项目使用 Tauri Updater 实现自动更新功能，需要配置密钥对来确保更新包的安全性。

## 🔑 密钥说明

密钥对已生成并保存在：
- **私钥**: `C:\Users\Wonvy\.tauri\monofocus.key` （请妥善保管，不要泄露！）
- **公钥**: `C:\Users\Wonvy\.tauri\monofocus.key.pub` （已配置在 `tauri.conf.json` 中）

## ⚙️ 配置 GitHub Secrets

为了让 GitHub Actions 能够签名发布包，需要在 GitHub 仓库中配置以下 Secrets：

### 1. 访问仓库设置
进入你的 GitHub 仓库：
```
https://github.com/Wonvy/MonoFocus/settings/secrets/actions
```

### 2. 添加 TAURI_PRIVATE_KEY

点击 **New repository secret**，配置：
- **Name**: `TAURI_PRIVATE_KEY`
- **Value**: 
```
dW50cnVzdGVkIGNvbW1lbnQ6IHJzaWduIGVuY3J5cHRlZCBzZWNyZXQga2V5ClJXUlRZMEl5YXVRQWF0VTZqOXNFZVRQMEx2MjFFd3dlYkpZOTdUZG5vNkQ3aEhiWUNJZ0FBQkFBQUFBQUFBQUFBQUlBQUFBQWZBZTNOSkF6YVE2UnBOcDY0K0ExM0JWR3VpUkE2V1dSRTFYd0NhTTBORnZnM3BPZktTWWhPdE9VTnY5YnBHNERLRVhFdllOb3pmbnhoNktkOGJCWjdOMis1cVJrS3lPSmpPQ3ExeEVHYUxaVWtxTFp0RmlQZ2JOU3JZV01CdURIZnRVVEdNaWxIQUE9Cg==
```

### 3. 添加 TAURI_KEY_PASSWORD

点击 **New repository secret**，配置：
- **Name**: `TAURI_KEY_PASSWORD`
- **Value**: （留空，因为生成密钥时没有设置密码）

> **重要提示**: 如果你在生成密钥时设置了密码，请在这里填写该密码。

### 4. 保存私钥备份

**强烈建议**将私钥文件备份到安全的地方（如密码管理器或加密存储）：
```
C:\Users\Wonvy\.tauri\monofocus.key
```

⚠️ **警告**: 如果丢失私钥或密码，将无法再发布更新！

## 🚀 使用流程

配置完成后，每次发布新版本：

1. 更新 `src-tauri/tauri.conf.json` 中的版本号
2. 更新 `CHANGELOG.md` 记录更新内容
3. 创建并推送 tag：
   ```bash
   git tag v1.2.0
   git push origin v1.2.0
   ```
4. GitHub Actions 会自动：
   - 编译应用
   - 使用私钥签名
   - 生成 `latest.json`
   - 发布到 GitHub Releases

用户就可以通过托盘菜单的"检查更新"功能获取新版本了！

## 🔒 安全性

- ✅ 公钥已配置在 `tauri.conf.json` 中（公开可见）
- ✅ 私钥保存在 GitHub Secrets 中（加密存储）
- ✅ 发布包会被自动签名
- ✅ 客户端会验证签名，确保更新包未被篡改

## 📝 注意事项

1. **不要将私钥提交到 Git 仓库**
2. **不要在公开场合分享私钥**
3. **定期备份私钥文件**
4. **如需更换密钥，需要重新配置并发布新版本**

