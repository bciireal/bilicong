![BiliCong Banner Logo](src/assets/banner-logo.png?raw=true "BiliCong Banner Logo")

B 站手机版缓存视频导出工具

## 使用方法

- 打开手机开发者模式 -> USB 调试 -> 使用数据线连接电脑
- 到 [Github Release](https://github.com/bciireal/bilicong/releases/latest) 处下载压缩包
- 全部解压
- 运行 `bilicong_backend.exe`

## 构建方法

- 安装依赖
  - Node.js
  - Rust
  - adb
  - ffmpeg
- `npm i`
- `npx tauri dev` (开发) 或 `npx tauri build` (构建)

## TODO

- [ ] 任务并行
- [ ] 忽略或者标记带 `is_completed: false` 的视频
- [ ] 自由选择 `adb` 和 `ffmpeg`
- [ ] 优化 README
