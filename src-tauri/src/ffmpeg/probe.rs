use std::path::Path;

use serde::Deserialize;

use crate::error::AppError;
use crate::ffmpeg::binary::find_ffprobe;
use crate::models::{MediaInfo, StreamInfo};

#[derive(Debug, Deserialize)]
struct ProbeJson {
    streams: Option<Vec<RawStream>>,
    format: Option<RawFormat>,
}

#[derive(Debug, Deserialize)]
struct RawFormat {
    filename: Option<String>,
    duration: Option<String>,
    format_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    bit_rate: Option<String>,
    sample_rate: Option<String>,
    channels: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    avg_frame_rate: Option<String>,
}

pub fn probe_media(path: &str) -> Result<MediaInfo, AppError> {
    let input = Path::new(path);
    if !input.exists() {
        return Err(AppError::msg("文件不存在"));
    }

    let ffprobe = find_ffprobe()?;
    let output = std::process::Command::new(ffprobe)
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            path,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::msg(format!(
            "无法解析媒体信息：{}",
            stderr.trim()
        )));
    }

    let parsed: ProbeJson = serde_json::from_slice(&output.stdout)
        .map_err(|err| AppError::msg(format!("ffprobe 输出无效：{err}")))?;

    let format = parsed.format.unwrap_or(RawFormat {
        filename: None,
        duration: None,
        format_name: None,
    });

    let file_size = std::fs::metadata(path)?.len();
    let duration_secs = format
        .duration
        .as_deref()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.0);

    let mut audio = None;
    let mut video = None;

    for stream in parsed.streams.unwrap_or_default() {
        let codec = stream.codec_name.unwrap_or_else(|| "unknown".into());
        let info = StreamInfo {
            codec,
            bit_rate: stream.bit_rate,
            sample_rate: stream.sample_rate,
            channels: stream.channels,
            width: stream.width,
            height: stream.height,
            frame_rate: stream
                .avg_frame_rate
                .filter(|rate| rate != "0/0" && rate != "0"),
        };

        match stream.codec_type.as_deref() {
            Some("audio") if audio.is_none() => audio = Some(info),
            Some("video") if video.is_none() => video = Some(info),
            _ => {}
        }
    }

    Ok(MediaInfo {
        path: format.filename.unwrap_or_else(|| path.to_string()),
        file_name: input
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        file_size,
        duration_secs,
        format_name: format.format_name.unwrap_or_else(|| "unknown".into()),
        has_audio: audio.is_some(),
        has_video: video.is_some(),
        audio,
        video,
    })
}
