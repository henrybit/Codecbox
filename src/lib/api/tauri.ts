import { invoke } from '@tauri-apps/api/core';
import type {
	ConvertMode,
	ConvertOutcome,
	ConvertRequest,
	FfmpegStatus,
	FormatSpec,
	MediaInfo
} from '$lib/types/media';

export function checkFfmpeg() {
	return invoke<FfmpegStatus>('check_ffmpeg');
}

export function listFormats(mode: ConvertMode) {
	return invoke<FormatSpec[]>('list_formats', { mode });
}

export function probeMedia(path: string) {
	return invoke<MediaInfo>('probe_media', { path });
}

export function suggestOutputPath(inputPath: string, extension: string) {
	return invoke<string>('suggest_output_path', { inputPath, extension });
}

export function startConvert(request: ConvertRequest) {
	return invoke<ConvertOutcome>('start_convert', { request });
}

export function cancelConvert() {
	return invoke<void>('cancel_convert');
}
