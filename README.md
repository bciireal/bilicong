# BiliSync

B 站手机版缓存视频导出工具

## 使用方法

- 到 release 处下载
- 全部解压
- 运行 `bilisync_backend.exe`

## 构建方法

- 安装依赖: Node.js Rust
- `npm i`
- `npx tauri dev` 或 `npx tauri build`

## TODO

- [ ] 任务并行
- [ ] 忽略或者标记带 `is_completed: false` 的视频
- [ ] 自由选择 `adb` 和 `ffmpeg`
- [ ] 优化 README
