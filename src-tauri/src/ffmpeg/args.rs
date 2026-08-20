use crate::error::AppError;
use crate::ffmpeg::presets::get_format;
use crate::models::{ConvertMode, ConvertRequest, FormatSpec};

pub fn build_args(request: &ConvertRequest) -> Result<Vec<String>, AppError> {
    let spec = get_format(&request.format_id)?;
    validate_mode(&request.mode, &spec)?;

    let mut args = vec![
        "-hide_banner".into(),
        "-y".into(),
        "-i".into(),
        request.input_path.clone(),
        "-progress".into(),
        "pipe:1".into(),
        "-nostats".into(),
        "-loglevel".into(),
        "error".into(),
    ];

    match request.mode {
        ConvertMode::AudioToAudio | ConvertMode::VideoToAudio => {
            args.push("-vn".into());
            append_audio(&mut args, &spec, request);
        }
        ConvertMode::VideoToVideo => {
            append_video(&mut args, &spec, request);
            if spec.id != "gif" {
                append_audio(&mut args, &spec, request);
            }
        }
    }

    args.push(request.output_path.clone());
    Ok(args)
}

fn validate_mode(mode: &ConvertMode, spec: &FormatSpec) -> Result<(), AppError> {
    match mode {
        ConvertMode::AudioToAudio | ConvertMode::VideoToAudio if spec.video_codec.is_some() => {
            Err(AppError::msg("当前模式请选择音频格式"))
        }
        ConvertMode::VideoToVideo if spec.video_codec.is_none() => {
            Err(AppError::msg("当前模式请选择视频格式"))
        }
        _ => Ok(()),
    }
}

fn append_audio(args: &mut Vec<String>, spec: &FormatSpec, request: &ConvertRequest) {
    if spec.audio_codec == "none" {
        args.push("-an".into());
        return;
    }

    args.extend(["-c:a".into(), spec.audio_codec.clone()]);

    if spec.supports_audio_bitrate && request.audio.bitrate != "original" {
        args.extend(["-b:a".into(), request.audio.bitrate.clone()]);
    }

    if request.audio.sample_rate != "original" {
        args.extend(["-ar".into(), request.audio.sample_rate.clone()]);
    }

    if request.audio.channels != "original" {
        args.extend(["-ac".into(), request.audio.channels.clone()]);
    }

    if spec.audio_codec == "libopus" && request.audio.sample_rate == "original" {
        args.extend(["-ar".into(), "48000".into()]);
    }
}

fn append_video(args: &mut Vec<String>, spec: &FormatSpec, request: &ConvertRequest) {
    if spec.id == "gif" {
        let fps = parse_fps(&request.video.fps);
        let scale = match request.video.resolution.as_str() {
            "1080" => "-2:1080",
            "720" => "-2:720",
            "480" => "-2:480",
            _ => "-2:480",
        };
        args.extend([
            "-vf".into(),
            format!("fps={fps},scale={scale}:flags=lanczos"),
            "-an".into(),
            "-loop".into(),
            "0".into(),
        ]);
        return;
    }

    let video_codec = spec.video_codec.clone().unwrap_or_else(|| "libx264".into());
    args.extend(["-c:v".into(), video_codec.clone()]);

    match video_codec.as_str() {
        "libx264" | "libx265" => {
            args.extend([
                "-preset".into(),
                request.video.preset.clone(),
                "-crf".into(),
                crf_for_quality(&request.video.quality, false).into(),
                "-pix_fmt".into(),
                "yuv420p".into(),
            ]);
        }
        "libvpx-vp9" => {
            args.extend([
                "-b:v".into(),
                "0".into(),
                "-crf".into(),
                crf_for_quality(&request.video.quality, true).into(),
                "-row-mt".into(),
                "1".into(),
            ]);
        }
        "wmv2" => {
            args.extend([
                "-b:v".into(),
                bitrate_for_quality(&request.video.quality).into(),
            ]);
        }
        _ => {}
    }

    let mut filters: Vec<String> = Vec::new();
    if request.video.resolution != "original" {
        filters.push(format!("scale=-2:{}", request.video.resolution));
    }
    if !filters.is_empty() {
        args.extend(["-vf".into(), filters.join(",")]);
    }

    if request.video.fps != "original" {
        args.extend(["-r".into(), request.video.fps.clone()]);
    }

    if spec.id == "mp4" || spec.id == "mov" {
        args.extend(["-movflags".into(), "+faststart".into()]);
    }

    if spec.id == "hls" {
        append_hls(args, request);
    }
}

fn append_hls(args: &mut Vec<String>, request: &ConvertRequest) {
    let secs = parse_hls_time(&request.video.segment_secs);
    args.extend([
        "-profile:v".into(),
        "main".into(),
        "-level".into(),
        "4.0".into(),
        "-force_key_frames".into(),
        format!("expr:gte(t,n_forced*{secs})"),
        "-hls_time".into(),
        secs.to_string(),
        "-hls_playlist_type".into(),
        "vod".into(),
        "-hls_list_size".into(),
        "0".into(),
        "-hls_flags".into(),
        "independent_segments".into(),
        "-hls_segment_filename".into(),
        hls_segment_pattern(&request.output_path),
        "-f".into(),
        "hls".into(),
    ]);
}

fn hls_segment_pattern(output_path: &str) -> String {
    let path = std::path::Path::new(output_path);
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("segment");
    parent.join(format!("{stem}_%03d.ts")).display().to_string()
}

fn parse_hls_time(value: &str) -> u32 {
    match value {
        "2" | "4" | "6" | "10" => value.parse().unwrap_or(6),
        _ => 6,
    }
}

fn parse_fps(value: &str) -> u32 {
    match value {
        "24" | "25" | "30" | "60" => value.parse().unwrap_or(12),
        _ => 12,
    }
}

fn crf_for_quality(quality: &str, vp9: bool) -> &'static str {
    match (quality, vp9) {
        ("high", false) => "18",
        ("compact", false) => "28",
        ("high", true) => "24",
        ("compact", true) => "36",
        (_, true) => "32",
        _ => "23",
    }
}

fn bitrate_for_quality(quality: &str) -> &'static str {
    match quality {
        "high" => "4M",
        "compact" => "1M",
        _ => "2M",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AudioOptions, VideoOptions};

    fn request(mode: ConvertMode, format_id: &str) -> ConvertRequest {
        ConvertRequest {
            job_id: "job".into(),
            mode,
            input_path: "/tmp/in.mp4".into(),
            output_path: "/tmp/out.mp3".into(),
            format_id: format_id.into(),
            audio: AudioOptions::default(),
            video: VideoOptions::default(),
        }
    }

    #[test]
    fn audio_mode_strips_video_and_sets_codec() {
        let args = build_args(&request(ConvertMode::VideoToAudio, "mp3")).unwrap();
        assert!(args.contains(&"-vn".into()));
        assert!(args.windows(2).any(|pair| pair == ["-c:a", "libmp3lame"]));
        assert!(args.windows(2).any(|pair| pair == ["-b:a", "192k"]));
    }

    #[test]
    fn flac_skips_audio_bitrate() {
        let mut req = request(ConvertMode::AudioToAudio, "flac");
        req.output_path = "/tmp/out.flac".into();
        let args = build_args(&req).unwrap();
        assert!(!args.contains(&"-b:a".into()));
        assert!(args.windows(2).any(|pair| pair == ["-c:a", "flac"]));
    }

    #[test]
    fn mp4_uses_h264_and_faststart() {
        let mut req = request(ConvertMode::VideoToVideo, "mp4");
        req.output_path = "/tmp/out.mp4".into();
        req.video.resolution = "720".into();
        let args = build_args(&req).unwrap();
        assert!(args.windows(2).any(|pair| pair == ["-c:v", "libx264"]));
        assert!(args.windows(2).any(|pair| pair == ["-vf", "scale=-2:720"]));
        assert!(args.contains(&"+faststart".into()));
    }

    #[test]
    fn rejects_video_format_in_audio_mode() {
        let err = build_args(&request(ConvertMode::AudioToAudio, "mp4")).unwrap_err();
        assert!(err.to_string().contains("音频格式"));
    }

    #[test]
    fn hls_writes_playlist_and_ts_segments() {
        let mut req = request(ConvertMode::VideoToVideo, "hls");
        req.output_path = "/tmp/movie.m3u8".into();
        req.video.segment_secs = "4".into();
        let args = build_args(&req).unwrap();
        assert!(args.windows(2).any(|pair| pair == ["-c:v", "libx264"]));
        assert!(args.windows(2).any(|pair| pair == ["-c:a", "aac"]));
        assert!(args.windows(2).any(|pair| pair == ["-f", "hls"]));
        assert!(args.windows(2).any(|pair| pair == ["-hls_time", "4"]));
        assert!(args
            .windows(2)
            .any(|pair| pair == ["-hls_segment_filename", "/tmp/movie_%03d.ts"]));
        assert_eq!(args.last(), Some(&"/tmp/movie.m3u8".into()));
    }
}
