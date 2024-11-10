<!--
SPDX-FileCopyrightText: 2024 Integral <integral@member.fsf.org>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# LibreTranslate Plugin for Pot-App 🌐

[中文](README.md) | English

## Available Instances 🔗

### Online Instances
- [translate.atomjump.com](https://translate.atomjump.com/)

### Local Offline Instance 💻
You can set up a local offline translation instance by following these steps:

1. Download Installation Package 📥
   - Go to [ott project](https://github.com/jianchang512/ott) to download the Windows pre-compiled package
   - Extract to an English directory path without spaces

2. Launch Service and Download Models 🚀
   - Double-click `start.exe` to launch the service
   - First launch will automatically download models (approximately 6.7GB)
   - If unable to access `https://raw.githubusercontent.com`:
     - Set proxy address in `set.ini`
     - Or download pre-packaged models from the project's Baidu Netdisk
   - Default service address is `http://127.0.0.1:9911`
   - For subsequent uses, simply double-click `start.exe`

3. Configure Plugin ⚙️
   - Enter `http://127.0.0.1:9911` in the instance URL
   - You can now start using local offline translation

**Note: When the instance URL is empty, [translate.argosopentech.com](https://translate.atomjump.com/) is used by default**

## Technical Details 🔧
The local offline translation is based on the [LibreTranslate](https://github.com/LibreTranslate/LibreTranslate) open source project, repackaged through the [ott](https://github.com/jianchang512/ott) project to provide a more convenient local deployment solution.

## License 📜
- Licensed under the [GNU General Public License, version 3 or any later version](LICENSES/GPL-3.0-or-later.txt).