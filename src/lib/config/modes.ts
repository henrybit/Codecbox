import type { ConvertMode } from '$lib/types/media';

export type ModeMeta = {
	id: ConvertMode;
	title: string;
	subtitle: string;
	hint: string;
	accent: 'teal' | 'amber' | 'violet';
	accept: 'audio' | 'video';
};

export const AUDIO_EXTENSIONS = ['mp3', 'wav', 'flac', 'aac', 'm4a', 'ogg', 'opus', 'wma', 'aiff', 'aif'];
export const VIDEO_EXTENSIONS = [
	'mp4',
	'mkv',
	'mov',
	'webm',
	'avi',
	'wmv',
	'm4v',
	'mpeg',
	'mpg',
	'flv',
	'ts',
	'3gp',
	'm3u8'
];

export const MODES: ModeMeta[] = [
	{
		id: 'audio-to-audio',
		title: '音频转音频',
		subtitle: '转码、压缩、换格式',
		hint: 'MP3 / AAC / FLAC / WAV / Opus…',
		accent: 'teal',
		accept: 'audio'
	},
	{
		id: 'video-to-audio',
		title: '视频转音频',
		subtitle: '抽取音轨并转码',
		hint: '从 MP4、MKV、MOV 等提取音频',
		accent: 'amber',
		accept: 'video'
	},
	{
		id: 'video-to-video',
		title: '视频转视频',
		subtitle: '改封装、分辨率与编码',
		hint: 'MP4 / MKV / WebM / MOV / HLS / GIF…',
		accent: 'violet',
		accept: 'video'
	}
];

export const BITRATE_OPTIONS = [
	{ value: 'original', label: '编码器默认' },
	{ value: '128k', label: '128 kbps' },
	{ value: '192k', label: '192 kbps' },
	{ value: '256k', label: '256 kbps' },
	{ value: '320k', label: '320 kbps' }
];

export const SAMPLE_RATE_OPTIONS = [
	{ value: 'original', label: '保持原样' },
	{ value: '44100', label: '44.1 kHz' },
	{ value: '48000', label: '48 kHz' }
];

export const CHANNEL_OPTIONS = [
	{ value: 'original', label: '保持原样' },
	{ value: '1', label: '单声道' },
	{ value: '2', label: '立体声' }
];

export const RESOLUTION_OPTIONS = [
	{ value: 'original', label: '原始分辨率' },
	{ value: '1080', label: '1080p' },
	{ value: '720', label: '720p' },
	{ value: '480', label: '480p' }
];

export const FPS_OPTIONS = [
	{ value: 'original', label: '原始帧率' },
	{ value: '24', label: '24 fps' },
	{ value: '30', label: '30 fps' },
	{ value: '60', label: '60 fps' }
];

export const QUALITY_OPTIONS = [
	{ value: 'high', label: '高质量' },
	{ value: 'standard', label: '标准' },
	{ value: 'compact', label: '更小体积' }
];

export const PRESET_OPTIONS = [
	{ value: 'veryfast', label: '更快' },
	{ value: 'medium', label: '均衡' },
	{ value: 'slow', label: '更清晰' }
];

export const HLS_SEGMENT_OPTIONS = [
	{ value: '2', label: '2 秒' },
	{ value: '4', label: '4 秒' },
	{ value: '6', label: '6 秒' },
	{ value: '10', label: '10 秒' }
];

export function modeMeta(mode: ConvertMode) {
	return MODES.find((item) => item.id === mode) ?? MODES[0];
}

export function extensionsFor(mode: ConvertMode) {
	return modeMeta(mode).accept === 'audio' ? AUDIO_EXTENSIONS : VIDEO_EXTENSIONS;
}
