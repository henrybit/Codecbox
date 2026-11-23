import type { ConvertMode } from '$lib/types/media';

export type ModeCopy = {
	title: string;
	subtitle: string;
	hint: string;
};

export type Catalog = {
	appTitle: string;
	language: string;
	ffmpegConnected: string;
	ffmpegChecking: string;
	ffmpegUnavailable: string;
	modeKicker: string;
	sourceFile: string;
	outputFormat: string;
	encodeSettings: string;
	export: string;
	cancel: string;
	start: string;
	outputPath: string;
	outputPlaceholder: string;
	browse: string;
	hlsHint: string;
	dropAudio: string;
	dropVideo: string;
	chooseFile: string;
	replaceFile: string;
	showInFolder: string;
	working: string;
	pickInputTitle: string;
	pickOutputTitle: string;
	audioFiles: string;
	videoFiles: string;
	mediaFile: string;
	resolution: string;
	fps: string;
	quality: string;
	preset: string;
	segment: string;
	bitrate: string;
	sampleRate: string;
	channels: string;
	optEncoderDefault: string;
	optKeep: string;
	optMono: string;
	optStereo: string;
	optOriginalResolution: string;
	optOriginalFps: string;
	optHigh: string;
	optStandard: string;
	optCompact: string;
	optFaster: string;
	optBalanced: string;
	optClearer: string;
	optSeconds: string;
	modes: Record<ConvertMode, ModeCopy>;
	formats: Record<string, string>;
};

const en: Catalog = {
	appTitle: 'Audio & Video Converter',
	language: 'Language',
	ffmpegConnected: 'FFmpeg connected',
	ffmpegChecking: 'Checking FFmpeg',
	ffmpegUnavailable: 'FFmpeg unavailable',
	modeKicker: 'Mode',
	sourceFile: 'Source',
	outputFormat: 'Output format',
	encodeSettings: 'Encode settings',
	export: 'Export',
	cancel: 'Cancel',
	start: 'Convert',
	outputPath: 'Output path',
	outputPlaceholder: 'Choose or type an output path',
	browse: 'Browse',
	hlsHint: 'HLS writes an .m3u8 playlist and .ts segments in the same folder.',
	dropAudio: 'Drop audio here, or click to choose',
	dropVideo: 'Drop a video here, or click to choose',
	chooseFile: 'Choose file',
	replaceFile: 'Replace file',
	showInFolder: 'Show in folder',
	working: 'Working',
	pickInputTitle: 'Choose source file',
	pickOutputTitle: 'Choose output file',
	audioFiles: 'Audio files',
	videoFiles: 'Video files',
	mediaFile: 'Media file',
	resolution: 'Resolution',
	fps: 'Frame rate',
	quality: 'Quality',
	preset: 'Encode speed',
	segment: 'Segment length',
	bitrate: 'Audio bitrate',
	sampleRate: 'Sample rate',
	channels: 'Channels',
	optEncoderDefault: 'Encoder default',
	optKeep: 'Keep original',
	optMono: 'Mono',
	optStereo: 'Stereo',
	optOriginalResolution: 'Original resolution',
	optOriginalFps: 'Original frame rate',
	optHigh: 'High',
	optStandard: 'Standard',
	optCompact: 'Smaller file',
	optFaster: 'Faster',
	optBalanced: 'Balanced',
	optClearer: 'Clearer',
	optSeconds: '{n} sec',
	modes: {
		'audio-to-audio': {
			title: 'Audio to audio',
			subtitle: 'Transcode, compress, change format',
			hint: 'MP3 / AAC / FLAC / WAV / Opus…'
		},
		'video-to-audio': {
			title: 'Video to audio',
			subtitle: 'Extract and transcode the soundtrack',
			hint: 'Pull audio from MP4, MKV, MOV, and more'
		},
		'video-to-video': {
			title: 'Video to video',
			subtitle: 'Change container, resolution, and codec',
			hint: 'MP4 / MKV / WebM / MOV / HLS / GIF…'
		}
	},
	formats: {
		mp3: 'Widely compatible lossy audio',
		aac: 'Efficient lossy audio for phones and players',
		m4a: 'AAC in an Apple-friendly container',
		wav: 'Uncompressed PCM, larger files',
		flac: 'Lossless compression, full quality',
		ogg: 'Open lossy audio',
		opus: 'Strong quality at low bitrates',
		wma: 'Windows Media audio',
		aiff: 'Uncompressed PCM for Apple tools',
		mp4: 'The most compatible H.264 video',
		mkv: 'Open container for many codecs',
		mov: 'Common in QuickTime and editors',
		webm: 'Web-friendly VP9 video',
		avi: 'Classic container for older devices',
		wmv: 'Windows Media video',
		hls: 'Segmented stream: m3u8 plus TS',
		gif: 'Animation, no audio'
	}
};

const zh: Catalog = {
	appTitle: '音视频转换工具',
	language: '语言',
	ffmpegConnected: 'FFmpeg 已连接',
	ffmpegChecking: '正在检测 FFmpeg',
	ffmpegUnavailable: 'FFmpeg 不可用',
	modeKicker: '工作模式',
	sourceFile: '源文件',
	outputFormat: '输出格式',
	encodeSettings: '转码参数',
	export: '导出',
	cancel: '取消转换',
	start: '开始转换',
	outputPath: '输出路径',
	outputPlaceholder: '选择或输入输出文件路径',
	browse: '浏览',
	hlsHint: 'HLS 会生成 .m3u8 播放列表，以及同目录下的 .ts 分片文件。',
	dropAudio: '拖入音频，或点击选择',
	dropVideo: '拖入视频，或点击选择',
	chooseFile: '选择文件',
	replaceFile: '更换文件',
	showInFolder: '在文件夹中显示',
	working: '处理中',
	pickInputTitle: '选择源文件',
	pickOutputTitle: '选择输出文件',
	audioFiles: '音频文件',
	videoFiles: '视频文件',
	mediaFile: '媒体文件',
	resolution: '分辨率',
	fps: '帧率',
	quality: '画质',
	preset: '编码速度',
	segment: '分片时长',
	bitrate: '音频码率',
	sampleRate: '采样率',
	channels: '声道',
	optEncoderDefault: '编码器默认',
	optKeep: '保持原样',
	optMono: '单声道',
	optStereo: '立体声',
	optOriginalResolution: '原始分辨率',
	optOriginalFps: '原始帧率',
	optHigh: '高质量',
	optStandard: '标准',
	optCompact: '更小体积',
	optFaster: '更快',
	optBalanced: '均衡',
	optClearer: '更清晰',
	optSeconds: '{n} 秒',
	modes: {
		'audio-to-audio': {
			title: '音频转音频',
			subtitle: '转码、压缩、换格式',
			hint: 'MP3 / AAC / FLAC / WAV / Opus…'
		},
		'video-to-audio': {
			title: '视频转音频',
			subtitle: '抽取音轨并转码',
			hint: '从 MP4、MKV、MOV 等提取音频'
		},
		'video-to-video': {
			title: '视频转视频',
			subtitle: '改封装、分辨率与编码',
			hint: 'MP4 / MKV / WebM / MOV / HLS / GIF…'
		}
	},
	formats: {
		mp3: '兼容性最好的有损音频',
		aac: '高效有损音频，适合移动设备',
		m4a: 'AAC 封装，苹果生态常用',
		wav: '无损 PCM，体积较大',
		flac: '无损压缩，音质完整',
		ogg: '开源有损音频',
		opus: '低码率下音质出色',
		wma: 'Windows Media 音频',
		aiff: '苹果平台无损 PCM',
		mp4: '最通用的 H.264 视频',
		mkv: '开放容器，兼容多种编码',
		mov: 'QuickTime / 剪辑软件常用',
		webm: '网页友好的 VP9 视频',
		avi: '传统容器，兼容老设备',
		wmv: 'Windows Media 视频',
		hls: '切片流媒体，生成 m3u8 + TS',
		gif: '动画图，不含音频'
	}
};

export const catalogs = { en, zh } satisfies Record<string, Catalog>;

export type MessageKey = {
	[K in keyof Catalog]: Catalog[K] extends string ? K : never;
}[keyof Catalog];
