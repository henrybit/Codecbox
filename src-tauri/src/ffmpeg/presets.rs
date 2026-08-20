use crate::error::AppError;
use crate::models::{ConvertMode, FormatCategory, FormatSpec};

pub fn all_formats() -> Vec<FormatSpec> {
    vec![
        audio(
            "mp3",
            "MP3",
            "mp3",
            "libmp3lame",
            "兼容性最好的有损音频",
            true,
        ),
        audio(
            "aac",
            "AAC",
            "aac",
            "aac",
            "高效有损音频，适合移动设备",
            true,
        ),
        audio("m4a", "M4A", "m4a", "aac", "AAC 封装，苹果生态常用", true),
        audio(
            "wav",
            "WAV",
            "wav",
            "pcm_s16le",
            "无损 PCM，体积较大",
            false,
        ),
        audio("flac", "FLAC", "flac", "flac", "无损压缩，音质完整", false),
        audio("ogg", "OGG", "ogg", "libvorbis", "开源有损音频", true),
        audio("opus", "Opus", "opus", "libopus", "低码率下音质出色", true),
        audio("wma", "WMA", "wma", "wmav2", "Windows Media 音频", true),
        audio(
            "aiff",
            "AIFF",
            "aiff",
            "pcm_s16be",
            "苹果平台无损 PCM",
            false,
        ),
        video("mp4", "MP4", "mp4", "libx264", "aac", "最通用的 H.264 视频"),
        video(
            "mkv",
            "MKV",
            "mkv",
            "libx264",
            "aac",
            "开放容器，兼容多种编码",
        ),
        video(
            "mov",
            "MOV",
            "mov",
            "libx264",
            "aac",
            "QuickTime / 剪辑软件常用",
        ),
        video(
            "webm",
            "WebM",
            "webm",
            "libvpx-vp9",
            "libopus",
            "网页友好的 VP9 视频",
        ),
        video(
            "avi",
            "AVI",
            "avi",
            "libx264",
            "libmp3lame",
            "传统容器，兼容老设备",
        ),
        video("wmv", "WMV", "wmv", "wmv2", "wmav2", "Windows Media 视频"),
        video(
            "hls",
            "HLS",
            "m3u8",
            "libx264",
            "aac",
            "切片流媒体，生成 m3u8 + TS",
        ),
        video("gif", "GIF", "gif", "gif", "none", "动画图，不含音频"),
    ]
}

pub fn formats_for_mode(mode: ConvertMode) -> Vec<FormatSpec> {
    all_formats()
        .into_iter()
        .filter(|spec| match mode {
            ConvertMode::AudioToAudio | ConvertMode::VideoToAudio => {
                spec.category == FormatCategory::Audio
            }
            ConvertMode::VideoToVideo => spec.category == FormatCategory::Video,
        })
        .collect()
}

pub fn get_format(format_id: &str) -> Result<FormatSpec, AppError> {
    all_formats()
        .into_iter()
        .find(|spec| spec.id == format_id)
        .ok_or_else(|| AppError::msg(format!("不支持的格式：{format_id}")))
}

fn audio(
    id: &str,
    label: &str,
    extension: &str,
    codec: &str,
    description: &str,
    supports_audio_bitrate: bool,
) -> FormatSpec {
    FormatSpec {
        id: id.into(),
        label: label.into(),
        extension: extension.into(),
        category: FormatCategory::Audio,
        description: description.into(),
        audio_codec: codec.into(),
        video_codec: None,
        supports_audio_bitrate,
    }
}

fn video(
    id: &str,
    label: &str,
    extension: &str,
    video_codec: &str,
    audio_codec: &str,
    description: &str,
) -> FormatSpec {
    FormatSpec {
        id: id.into(),
        label: label.into(),
        extension: extension.into(),
        category: FormatCategory::Video,
        description: description.into(),
        audio_codec: audio_codec.into(),
        video_codec: Some(video_codec.into()),
        supports_audio_bitrate: audio_codec != "none",
    }
}
