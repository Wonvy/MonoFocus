#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// 读取 package.json 获取版本号
const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '../package.json'), 'utf8'));
const version = packageJson.version;

console.log(`Updating version to ${version}...`);

// 更新 index.html 中的版本号
const indexHtmlPath = path.join(__dirname, '../src/index.html');
let indexHtml = fs.readFileSync(indexHtmlPath, 'utf8');
indexHtml = indexHtml.replace(
  /<p class="version-text">v[\d.]+<\/p>/,
  `<p class="version-text">v${version}</p>`
);
fs.writeFileSync(indexHtmlPath, indexHtml);
console.log('✓ Updated src/index.html');

// 更新 tauri.conf.json 中的版本号和窗口标题
const tauriConfPath = path.join(__dirname, '../src-tauri/tauri.conf.json');
let tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.package.version = version;
tauriConf.tauri.windows[0].title = `MonoFocus v${version}`;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
console.log('✓ Updated src-tauri/tauri.conf.json');

// 更新 Cargo.toml 中的版本号
const cargoTomlPath = path.join(__dirname, '../src-tauri/Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(
  /^version = "[\d.]+"$/m,
  `version = "${version}"`
);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log('✓ Updated src-tauri/Cargo.toml');

console.log('✨ Version update complete!');

