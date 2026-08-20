export type ConvertMode = 'audio-to-audio' | 'video-to-audio' | 'video-to-video';

export type FormatCategory = 'audio' | 'video';

export type ConvertStage = 'idle' | 'running' | 'done' | 'error' | 'cancelled';

export type FormatSpec = {
	id: string;
	label: string;
	extension: string;
	category: FormatCategory;
	description: string;
	audioCodec: string;
	videoCodec: string | null;
	supportsAudioBitrate: boolean;
};

export type AudioOptions = {
	bitrate: string;
	sampleRate: string;
	channels: string;
};

export type VideoOptions = {
	resolution: string;
	fps: string;
	quality: string;
	preset: string;
	segmentSecs: string;
};

export type ConvertRequest = {
	jobId: string;
	mode: ConvertMode;
	inputPath: string;
	outputPath: string;
	formatId: string;
	audio: AudioOptions;
	video: VideoOptions;
};

export type ConvertProgress = {
	jobId: string;
	ratio: number;
	outTimeSecs: number;
	speed: string | null;
	stage: ConvertStage;
	message: string | null;
};

export type ConvertOutcome = {
	status: 'done' | 'cancelled';
	outputPath: string | null;
};

export type StreamInfo = {
	codec: string;
	bitRate: string | null;
	sampleRate: string | null;
	channels: number | null;
	width: number | null;
	height: number | null;
	frameRate: string | null;
};

export type MediaInfo = {
	path: string;
	fileName: string;
	fileSize: number;
	durationSecs: number;
	formatName: string;
	hasAudio: boolean;
	hasVideo: boolean;
	audio: StreamInfo | null;
	video: StreamInfo | null;
};

export type FfmpegStatus = {
	available: boolean;
	ffmpegPath: string | null;
	ffprobePath: string | null;
	version: string | null;
	message: string;
};
