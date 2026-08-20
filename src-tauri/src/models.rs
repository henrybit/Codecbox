use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ConvertMode {
    AudioToAudio,
    VideoToAudio,
    VideoToVideo,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FormatCategory {
    Audio,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatSpec {
    pub id: String,
    pub label: String,
    pub extension: String,
    pub category: FormatCategory,
    pub description: String,
    pub audio_codec: String,
    pub video_codec: Option<String>,
    pub supports_audio_bitrate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOptions {
    pub bitrate: String,
    pub sample_rate: String,
    pub channels: String,
}

impl Default for AudioOptions {
    fn default() -> Self {
        Self {
            bitrate: "192k".into(),
            sample_rate: "44100".into(),
            channels: "2".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOptions {
    pub resolution: String,
    pub fps: String,
    pub quality: String,
    pub preset: String,
    #[serde(default = "default_segment_secs")]
    pub segment_secs: String,
}

fn default_segment_secs() -> String {
    "6".into()
}

impl Default for VideoOptions {
    fn default() -> Self {
        Self {
            resolution: "original".into(),
            fps: "original".into(),
            quality: "standard".into(),
            preset: "medium".into(),
            segment_secs: default_segment_secs(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRequest {
    pub job_id: String,
    pub mode: ConvertMode,
    pub input_path: String,
    pub output_path: String,
    pub format_id: String,
    pub audio: AudioOptions,
    pub video: VideoOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertProgress {
    pub job_id: String,
    pub ratio: f64,
    pub out_time_secs: f64,
    pub speed: Option<String>,
    pub stage: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertOutcome {
    pub status: String,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamInfo {
    pub codec: String,
    pub bit_rate: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub path: String,
    pub file_name: String,
    pub file_size: u64,
    pub duration_secs: f64,
    pub format_name: String,
    pub has_audio: bool,
    pub has_video: bool,
    pub audio: Option<StreamInfo>,
    pub video: Option<StreamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FfmpegStatus {
    pub available: bool,
    pub ffmpeg_path: Option<String>,
    pub ffprobe_path: Option<String>,
    pub version: Option<String>,
    pub message: String,
}
