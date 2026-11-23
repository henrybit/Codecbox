import type { MessageKey } from '$lib/i18n/messages';
import type { ConvertMode } from '$lib/types/media';

export type Choice = {
	value: string;
	label?: string;
	labelKey?: MessageKey;
};

export type ModeMeta = {
	id: ConvertMode;
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
	{ id: 'audio-to-audio', accent: 'teal', accept: 'audio' },
	{ id: 'video-to-audio', accent: 'amber', accept: 'video' },
	{ id: 'video-to-video', accent: 'violet', accept: 'video' }
];

export const BITRATE_OPTIONS: Choice[] = [
	{ value: 'original', labelKey: 'optEncoderDefault' },
	{ value: '128k', label: '128 kbps' },
	{ value: '192k', label: '192 kbps' },
	{ value: '256k', label: '256 kbps' },
	{ value: '320k', label: '320 kbps' }
];

export const SAMPLE_RATE_OPTIONS: Choice[] = [
	{ value: 'original', labelKey: 'optKeep' },
	{ value: '44100', label: '44.1 kHz' },
	{ value: '48000', label: '48 kHz' }
];

export const CHANNEL_OPTIONS: Choice[] = [
	{ value: 'original', labelKey: 'optKeep' },
	{ value: '1', labelKey: 'optMono' },
	{ value: '2', labelKey: 'optStereo' }
];

export const RESOLUTION_OPTIONS: Choice[] = [
	{ value: 'original', labelKey: 'optOriginalResolution' },
	{ value: '1080', label: '1080p' },
	{ value: '720', label: '720p' },
	{ value: '480', label: '480p' }
];

export const FPS_OPTIONS: Choice[] = [
	{ value: 'original', labelKey: 'optOriginalFps' },
	{ value: '24', label: '24 fps' },
	{ value: '30', label: '30 fps' },
	{ value: '60', label: '60 fps' }
];

export const QUALITY_OPTIONS: Choice[] = [
	{ value: 'high', labelKey: 'optHigh' },
	{ value: 'standard', labelKey: 'optStandard' },
	{ value: 'compact', labelKey: 'optCompact' }
];

export const PRESET_OPTIONS: Choice[] = [
	{ value: 'veryfast', labelKey: 'optFaster' },
	{ value: 'medium', labelKey: 'optBalanced' },
	{ value: 'slow', labelKey: 'optClearer' }
];

export const HLS_SEGMENT_OPTIONS: Choice[] = [
	{ value: '2', labelKey: 'optSeconds' },
	{ value: '4', labelKey: 'optSeconds' },
	{ value: '6', labelKey: 'optSeconds' },
	{ value: '10', labelKey: 'optSeconds' }
];

export function modeMeta(mode: ConvertMode) {
	return MODES.find((item) => item.id === mode) ?? MODES[0];
}

export function extensionsFor(mode: ConvertMode) {
	return modeMeta(mode).accept === 'audio' ? AUDIO_EXTENSIONS : VIDEO_EXTENSIONS;
}
