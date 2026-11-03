use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use which::which_in;

use crate::error::AppError;
use crate::models::FfmpegStatus;

const EXTRA_DIRS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/usr/bin",
    "/opt/local/bin",
    "/snap/bin",
    "C:\\ffmpeg\\bin",
    "C:\\Program Files\\ffmpeg\\bin",
];

pub fn ffmpeg_status() -> FfmpegStatus {
    match (find_ffmpeg(), find_ffprobe()) {
        (Ok(ffmpeg), Ok(ffprobe)) => {
            let version = read_version(&ffmpeg).ok();
            FfmpegStatus {
                available: true,
                ffmpeg_path: Some(ffmpeg.display().to_string()),
                ffprobe_path: Some(ffprobe.display().to_string()),
                version,
                message: "FFmpeg 已就绪".into(),
            }
        }
        (Err(ffmpeg_err), _) => FfmpegStatus {
            available: false,
            ffmpeg_path: None,
            ffprobe_path: None,
            version: None,
            message: format!("未找到 ffmpeg。{ffmpeg_err}"),
        },
        (_, Err(ffprobe_err)) => FfmpegStatus {
            available: false,
            ffmpeg_path: None,
            ffprobe_path: None,
            version: None,
            message: format!("未找到 ffprobe。{ffprobe_err}"),
        },
    }
}

pub fn find_ffmpeg() -> Result<PathBuf, AppError> {
    find_tool("ffmpeg", "CODECBOX_FFMPEG")
}

pub fn find_ffprobe() -> Result<PathBuf, AppError> {
    find_tool("ffprobe", "CODECBOX_FFPROBE")
}

fn find_tool(name: &str, env_key: &str) -> Result<PathBuf, AppError> {
    if let Ok(value) = env::var(env_key) {
        let path = PathBuf::from(value);
        if is_executable(&path) {
            return Ok(path);
        }
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            for candidate in sidecar_candidates(dir, name) {
                if is_executable(&candidate) {
                    return Ok(candidate);
                }
            }
        }
    }

    let search_path = augmented_path();
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if let Ok(found) = which_in(name, Some(&search_path), cwd) {
        return Ok(found);
    }

    Err(AppError::Ffmpeg(format!(
        "请安装 FFmpeg 并确保 `{name}` 在 PATH 中，或设置 {env_key}"
    )))
}

fn sidecar_candidates(dir: &Path, name: &str) -> Vec<PathBuf> {
    let triple = env::consts::ARCH;
    let os = env::consts::OS;
    vec![
        dir.join(name),
        dir.join(format!("{name}.exe")),
        dir.join(format!("{name}-{triple}-{os}")),
        dir.join("binaries").join(name),
        dir.join("../Resources/binaries").join(name),
    ]
}

fn augmented_path() -> OsString {
    let mut paths: Vec<PathBuf> =
        env::split_paths(&env::var_os("PATH").unwrap_or_default()).collect();

    for dir in EXTRA_DIRS {
        let path = PathBuf::from(dir);
        if path.exists() && !paths.iter().any(|item| item == &path) {
            paths.push(path);
        }
    }

    env::join_paths(paths).unwrap_or_default()
}

fn is_executable(path: &Path) -> bool {
    path.is_file()
}

fn read_version(ffmpeg: &Path) -> Result<String, AppError> {
    let output = std::process::Command::new(ffmpeg)
        .arg("-version")
        .output()?;

    if !output.status.success() {
        return Err(AppError::Ffmpeg("无法读取 ffmpeg 版本".into()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().unwrap_or_default().trim();
    Ok(line.to_string())
}
