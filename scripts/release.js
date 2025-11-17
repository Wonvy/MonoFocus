#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// 获取命令行参数
const args = process.argv.slice(2);
if (args.length === 0) {
  console.error('❌ 请指定版本类型: patch, minor, major 或具体版本号');
  console.error('用法: npm run release patch');
  console.error('      npm run release 1.4.0');
  process.exit(1);
}

const versionArg = args[0];

try {
  console.log('🚀 开始发布流程...\n');

  // 1. 更新版本号
  console.log(`📦 更新版本号 (${versionArg})...`);
  execSync(`npm version ${versionArg} --no-git-tag-version`, { stdio: 'inherit' });

  // 读取新版本号
  const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '../package.json'), 'utf8'));
  const newVersion = packageJson.version;
  console.log(`✅ 版本号已更新为: ${newVersion}\n`);

  // 2. 提交更改
  console.log('📝 提交更改到 Git...');
  execSync('git add -A', { stdio: 'inherit' });
  execSync(`git commit -m "chore: bump version to ${newVersion}"`, { stdio: 'inherit' });
  console.log('✅ 更改已提交\n');

  // 3. 创建标签
  console.log(`🏷️  创建标签 v${newVersion}...`);
  execSync(`git tag -a v${newVersion} -m "Release v${newVersion}"`, { stdio: 'inherit' });
  console.log('✅ 标签已创建\n');

  // 4. 推送到 GitHub
  console.log('⬆️  推送到 GitHub...');
  execSync('git push origin main', { stdio: 'inherit' });
  execSync(`git push origin v${newVersion}`, { stdio: 'inherit' });
  console.log('✅ 已推送到 GitHub\n');

  console.log('🎉 发布完成！');
  console.log(`📦 版本: v${newVersion}`);
  console.log('🔗 查看构建状态: https://github.com/Wonvy/MonoFocus/actions');
  console.log(`🔗 发布页面: https://github.com/Wonvy/MonoFocus/releases/tag/v${newVersion}`);

} catch (error) {
  console.error('\n❌ 发布失败:', error.message);
  process.exit(1);
}

