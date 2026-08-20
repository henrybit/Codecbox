# Task Plan: Tauri 2 + SvelteKit 音视频转换工具

## Goal
用 Tauri 2 + SvelteKit 做一款模块化桌面音视频转换工具：音频转音频、视频转音频、视频转视频；FFmpeg 核心逻辑在 Rust 层封装。

## Current Phase
Phase 5

## Phases

### Phase 1: Requirements & Discovery
- [x] 确认空仓库与本机工具链
- **Status:** complete

### Phase 2: Planning & Structure
- [x] SvelteKit + Tauri 2 脚手架
- **Status:** complete

### Phase 3: Rust FFmpeg 核心封装
- [x] binary / presets / probe / args / runner / commands
- **Status:** complete

### Phase 4: SvelteKit UI 组件化
- [x] 模式切换、文件选择、格式/质量、进度
- **Status:** complete

### Phase 5: Testing & Delivery
- [x] cargo test 4 passed；svelte-check 0 errors；vite build + adapter-static 成功
- **Status:** complete

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| SvelteKit + tauri init | 用户明确要求 SvelteKit |
| tokio::process 调用系统/sidecar FFmpeg | 跨平台稳、易封装 |
| oneshot 取消转换 | 避免 Mutex 上 wait 死锁 |
| 自定义 CSS 令牌 | 组件样式可复用 |

## Errors Encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| sv create adapter cfTarget none | 1 | 改用默认 adapter，再手动换 static |
| env::consts::PATH_SEPARATOR | 1 | 改用 split_paths / join_paths |
| which_in cwd Option | 1 | 传入 PathBuf |
| convert wait 持有 Mutex | 1 | oneshot kill 通道 |
