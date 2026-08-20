# Findings

## Environment

- Empty workspace
- Node 24.14.0, npm 11.9.0, rustc 1.94.1
- ffmpeg at /opt/homebrew/bin/ffmpeg
- create-tauri-app 4.6.2 templates: svelte-ts, no sveltekit

## Tauri 2 + SvelteKit

- `@sveltejs/adapter-static` with `fallback: 'index.html'`
- `src/routes/+layout.ts`: `export const ssr = false`
- `tauri.conf.json` build: devUrl `http://localhost:5173`, frontendDist `../build`
- Plugins: dialog (open/save), opener (reveal output)

## FFmpeg strategy

- Probe: `ffprobe -v quiet -print_format json -show_format -show_streams`
- Convert: `ffmpeg -y -i input ... -progress pipe:1`
- Progress: parse `out_time_ms` / `progress=end`
- Locate binary: sidecar optional → PATH → common macOS/Homebrew paths
