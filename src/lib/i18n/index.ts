import type { Choice } from '$lib/config/modes';
import type { ConvertMode } from '$lib/types/media';
import { catalogs, type MessageKey } from './messages';
import { locale, type Locale } from './locale.svelte';

export type { Locale, MessageKey };
export { locale };

export function t(key: MessageKey, vars?: Record<string, string | number>) {
	const template = catalogs[locale.current][key];
	if (!vars) return template;
	return template.replace(/\{(\w+)\}/g, (_, name: string) => String(vars[name] ?? ''));
}

export function modeCopy(id: ConvertMode) {
	return catalogs[locale.current].modes[id];
}

export function formatBlurb(id: string, fallback: string) {
	return catalogs[locale.current].formats[id] ?? fallback;
}

export function localizeOptions(items: Choice[]) {
	return items.map((item) => ({
		value: item.value,
		label: item.labelKey ? t(item.labelKey, { n: item.value }) : (item.label ?? item.value)
	}));
}

const exactEnglish: Record<string, string> = {
	'FFmpeg 已就绪': 'FFmpeg is ready',
	'正在准备转换…': 'Preparing conversion…',
	'正在转换…': 'Converting…',
	处理中: 'Working',
	已取消转换: 'Conversion cancelled',
	转换完成: 'Conversion complete',
	'FFmpeg 转换失败': 'FFmpeg conversion failed',
	发生未知错误: 'Something went wrong',
	请先选择源文件: 'Choose a source file first',
	请指定输出路径: 'Choose an output path',
	输出文件不能与源文件相同: 'Output cannot be the same file as the source',
	该文件没有音频轨道: 'This file has no audio track',
	该视频没有可提取的音频: 'This video has no audio to extract',
	该文件没有视频轨道: 'This file has no video track',
	文件不存在: 'File does not exist',
	已取消: 'Cancelled',
	已有转换任务正在进行: 'A conversion is already running',
	当前模式请选择音频文件: 'Choose an audio file for this mode',
	当前模式请选择视频文件: 'Choose a video file for this mode',
	当前模式请选择音频格式: 'Choose an audio format for this mode',
	当前模式请选择视频格式: 'Choose a video format for this mode',
	'无法读取 ffmpeg 版本': 'Could not read the FFmpeg version'
};

const phraseReplacements: [string, string][] = [
	['未找到 ffmpeg。', 'ffmpeg was not found. '],
	['未找到 ffprobe。', 'ffprobe was not found. '],
	['FFmpeg 不可用：', 'FFmpeg is unavailable: '],
	['请安装 FFmpeg 并确保 `', 'Install FFmpeg and make sure `'],
	['` 在 PATH 中，或设置 ', '` is on PATH, or set '],
	['无法读取 ffmpeg 版本', 'Could not read the FFmpeg version'],
	['不支持的格式：', 'Unsupported format: '],
	['已有转换任务正在进行', 'A conversion is already running'],
	['请先选择源文件', 'Choose a source file first'],
	['请指定输出路径', 'Choose an output path'],
	['输出文件不能与源文件相同', 'Output cannot be the same file as the source'],
	['该文件没有音频轨道', 'This file has no audio track'],
	['该视频没有可提取的音频', 'This video has no audio to extract'],
	['该文件没有视频轨道', 'This file has no video track'],
	['当前模式请选择音频格式', 'Choose an audio format for this mode'],
	['当前模式请选择视频格式', 'Choose a video format for this mode'],
	['当前模式请选择音频文件', 'Choose an audio file for this mode'],
	['当前模式请选择视频文件', 'Choose a video file for this mode'],
	['文件不存在', 'File does not exist'],
	['无法解析媒体信息：', 'Could not read media info: '],
	['ffprobe 输出无效：', 'Invalid ffprobe output: '],
	['IO 错误：', 'IO error: '],
	['发生未知错误', 'Something went wrong'],
	['FFmpeg 转换失败', 'FFmpeg conversion failed'],
	['正在准备转换…', 'Preparing conversion…'],
	['正在转换…', 'Converting…'],
	['已取消转换', 'Conversion cancelled'],
	['转换完成', 'Conversion complete'],
	['FFmpeg 已就绪', 'FFmpeg is ready']
];

export function localizeMessage(message: string | null | undefined) {
	if (!message) return '';
	if (locale.current === 'zh') return message;
	const exact = exactEnglish[message];
	if (exact) return exact;
	let text = message;
	for (const [from, to] of phraseReplacements) {
		text = text.replaceAll(from, to);
	}
	return text;
}
