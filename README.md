<!--
SPDX-FileCopyrightText: 2024 Integral <integral@member.fsf.org>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# Pot-App 翻译 LibreTranslate 插件 🌐

中文 | [English](README_EN.md)

## 可用实例 🔗

### 线上实例
- [translate.atomjump.com](https://translate.atomjump.com/)

### 本地离线实例 💻
您可以设置本地离线翻译实例，步骤如下：

1. 下载安装包 📥
   - 前往 [ott 项目](https://github.com/jianchang512/ott) 下载 Windows 预编译包
   - 解压到无空格的英文目录下

2. 启动服务并下载模型 🚀
   - 双击 `start.exe` 启动服务
   - 首次启动会自动下载模型（约 6.7GB）
   - 如果无法访问 `https://raw.githubusercontent.com`：
     - 在 `set.ini` 中设置代理地址
     - 或从项目提供的百度网盘下载已打包好的模型
   - 默认服务地址为 `http://127.0.0.1:9911`
   - 后续启动只需双击 `start.exe` 即可

3. 设置插件 ⚙️
   - 在实例 URL 中填入 `http://127.0.0.1:9911`
   - 即可开始使用本地离线翻译功能

**注意：当实例 URL 为空时，默认使用 [translate.argosopentech.com](https://translate.atomjump.com/)**

## 技术说明 🔧
本地离线翻译基于 [LibreTranslate](https://github.com/LibreTranslate/LibreTranslate) 开源项目，通过 [ott](https://github.com/jianchang512/ott) 项目进行再封装，提供了更便捷的本地部署方案。

## 许可证 📜
- 本软件使用 [GNU 通用公共许可证 (GPL), 版本 3 或更新版本](LICENSES/GPL-3.0-or-later.txt) 授权。
