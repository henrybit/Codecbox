# CodecBox

基于 **Tauri 2** + **SvelteKit** 的桌面音视频转换工具。转换核心走 FFmpeg，参数构建、媒体探测、进度解析与取消全部封装在 Rust 层。

## 能力

三种工作模式：

- **音频转音频**：MP3 / AAC / M4A / WAV / FLAC / OGG / Opus / WMA / AIFF
- **视频转音频**：从视频抽取音轨并转码
- **视频转视频**：MP4 / MKV / MOV / WebM / AVI / WMV / HLS（m3u8 + TS 分片）/ GIF

支持拖放文件、格式网格、码率/分辨率/帧率等参数、实时进度与取消。

## 环境

- Node.js 18+
- Rust 1.77+
- [FFmpeg](https://ffmpeg.org/)（需包含 `ffmpeg` 与 `ffprobe`）

macOS：

```sh
brew install ffmpeg
```

## 开发

```sh
npm install
npm run desktop
```

前端单独预览（无 Tauri / 文件对话框）：

```sh
npm run dev
```

## 架构

```
src/                         SvelteKit 界面
  lib/api                    invoke 封装
  lib/components             可复用 UI / 业务组件
  lib/stores                 转换状态
  lib/config                 模式与选项
src-tauri/src
  commands.rs                Tauri 命令入口
  ffmpeg/binary.rs           定位 ffmpeg / ffprobe
  ffmpeg/presets.rs          格式与编码器目录
  ffmpeg/probe.rs            ffprobe 媒体信息
  ffmpeg/args.rs             按模式组装 FFmpeg 参数
  ffmpeg/runner.rs           进程、进度事件、取消
```

Rust 对外命令：`check_ffmpeg`、`list_formats`、`probe_media`、`suggest_output_path`、`start_convert`、`cancel_convert`。

打包时可把 FFmpeg 放到可执行文件旁作为 sidecar；开发期默认使用系统 PATH，也可用环境变量：

- `CODECBOX_FFMPEG`
- `CODECBOX_FFPROBE`
