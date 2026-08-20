use std::path::{Path, PathBuf};

use tauri::{AppHandle, State};

use crate::error::AppError;
use crate::ffmpeg::{
    cancel_running, ffmpeg_status, formats_for_mode, probe_media as probe_media_file, run_convert,
};
use crate::models::{
    ConvertMode, ConvertOutcome, ConvertRequest, FfmpegStatus, FormatSpec, MediaInfo,
};
use crate::state::AppState;

#[tauri::command]
pub fn check_ffmpeg() -> FfmpegStatus {
    ffmpeg_status()
}

#[tauri::command]
pub fn list_formats(mode: ConvertMode) -> Vec<FormatSpec> {
    formats_for_mode(mode)
}

#[tauri::command]
pub fn probe_media(path: String) -> Result<MediaInfo, AppError> {
    probe_media_file(&path)
}

#[tauri::command]
pub fn suggest_output_path(input_path: String, extension: String) -> Result<String, AppError> {
    Ok(unique_output_path(&input_path, &extension)?
        .display()
        .to_string())
}

#[tauri::command]
pub async fn start_convert(
    app: AppHandle,
    state: State<'_, AppState>,
    request: ConvertRequest,
) -> Result<ConvertOutcome, AppError> {
    validate_request(&request)?;
    let media = probe_media_file(&request.input_path)?;
    validate_media(&request.mode, &media)?;

    if let Some(parent) = Path::new(&request.output_path).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    run_convert(app, state.inner(), request, media.duration_secs).await
}

#[tauri::command]
pub async fn cancel_convert(state: State<'_, AppState>) -> Result<(), AppError> {
    cancel_running(state.inner()).await
}

fn validate_request(request: &ConvertRequest) -> Result<(), AppError> {
    if request.input_path.trim().is_empty() {
        return Err(AppError::msg("请先选择源文件"));
    }
    if request.output_path.trim().is_empty() {
        return Err(AppError::msg("请指定输出路径"));
    }
    if Path::new(&request.input_path) == Path::new(&request.output_path) {
        return Err(AppError::msg("输出文件不能与源文件相同"));
    }
    Ok(())
}

fn validate_media(mode: &ConvertMode, media: &MediaInfo) -> Result<(), AppError> {
    match mode {
        ConvertMode::AudioToAudio if !media.has_audio => Err(AppError::msg("该文件没有音频轨道")),
        ConvertMode::VideoToAudio if !media.has_audio => {
            Err(AppError::msg("该视频没有可提取的音频"))
        }
        ConvertMode::VideoToVideo if !media.has_video => Err(AppError::msg("该文件没有视频轨道")),
        _ => Ok(()),
    }
}

fn unique_output_path(input_path: &str, extension: &str) -> Result<PathBuf, AppError> {
    let input = Path::new(input_path);
    let parent = input.parent().unwrap_or_else(|| Path::new("."));
    let stem = input
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("output");

    let ext = extension.trim_start_matches('.');
    let mut candidate = parent.join(format!("{stem}.{ext}"));
    let mut index = 1;
    while candidate.exists() {
        candidate = parent.join(format!("{stem}_{index}.{ext}"));
        index += 1;
    }
    Ok(candidate)
}
