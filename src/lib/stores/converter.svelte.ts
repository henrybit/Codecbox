import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { open, save } from '@tauri-apps/plugin-dialog';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import * as api from '$lib/api/tauri';
import { extensionsFor, modeMeta } from '$lib/config/modes';
import type {
	AudioOptions,
	ConvertMode,
	ConvertProgress,
	FfmpegStatus,
	FormatSpec,
	MediaInfo,
	VideoOptions
} from '$lib/types/media';
import { fileExtension, invokeErrorMessage } from '$lib/utils/format';

const defaultAudio = (): AudioOptions => ({
	bitrate: '192k',
	sampleRate: '44100',
	channels: '2'
});

const defaultVideo = (): VideoOptions => ({
	resolution: 'original',
	fps: 'original',
	quality: 'standard',
	preset: 'medium',
	segmentSecs: '6'
});

class ConverterStore {
	mode = $state<ConvertMode>('audio-to-audio');
	ffmpeg = $state<FfmpegStatus | null>(null);
	formats = $state<FormatSpec[]>([]);
	input = $state<MediaInfo | null>(null);
	outputPath = $state('');
	formatId = $state('');
	audio = $state<AudioOptions>(defaultAudio());
	video = $state<VideoOptions>(defaultVideo());
	progress = $state<ConvertProgress | null>(null);
	busy = $state(false);
	error = $state<string | null>(null);
	dragActive = $state(false);
	ready = $state(false);

	currentFormat = $derived(this.formats.find((item) => item.id === this.formatId) ?? null);
	modeInfo = $derived(modeMeta(this.mode));
	canStart = $derived(
		Boolean(this.ffmpeg?.available && this.input && this.outputPath && this.formatId && !this.busy)
	);

	private unlistenProgress: UnlistenFn | undefined;

	async init() {
		if (this.ready) return;
		try {
			this.ffmpeg = await api.checkFfmpeg();
			await this.refreshFormats();
			this.unlistenProgress = await listen<ConvertProgress>('convert-progress', (event) => {
				this.progress = event.payload;
			});
			this.ready = true;
		} catch (error) {
			this.error = invokeErrorMessage(error);
		}
	}

	destroy() {
		this.unlistenProgress?.();
		this.unlistenProgress = undefined;
	}

	async setMode(mode: ConvertMode) {
		if (this.busy || this.mode === mode) return;
		this.mode = mode;
		this.error = null;
		this.progress = null;
		await this.refreshFormats();
		if (this.input && !this.inputMatchesMode(this.input.path)) {
			this.input = null;
			this.outputPath = '';
		} else if (this.input && this.currentFormat) {
			await this.refreshOutputPath();
		}
	}

	async refreshFormats() {
		this.formats = await api.listFormats(this.mode);
		if (!this.formats.some((item) => item.id === this.formatId)) {
			this.formatId = this.formats[0]?.id ?? '';
		}
		if (this.input) {
			await this.refreshOutputPath();
		}
	}

	async pickInput() {
		const selected = await open({
			multiple: false,
			title: '选择源文件',
			filters: [
				{
					name: this.modeInfo.accept === 'audio' ? '音频文件' : '视频文件',
					extensions: extensionsFor(this.mode)
				}
			]
		});
		if (typeof selected === 'string') {
			await this.loadInput(selected);
		}
	}

	async loadInput(path: string) {
		this.error = null;
		this.progress = null;
		if (!this.inputMatchesMode(path)) {
			this.error = `当前模式请选择${this.modeInfo.accept === 'audio' ? '音频' : '视频'}文件`;
			return;
		}
		try {
			this.input = await api.probeMedia(path);
			await this.refreshOutputPath();
		} catch (error) {
			this.input = null;
			this.outputPath = '';
			this.error = invokeErrorMessage(error);
		}
	}

	async selectFormat(formatId: string) {
		this.formatId = formatId;
		this.progress = null;
		if (this.input) {
			await this.refreshOutputPath();
		}
	}

	async pickOutput() {
		const extension = this.currentFormat?.extension ?? 'bin';
		const selected = await save({
			title: '选择输出文件',
			defaultPath: this.outputPath || `output.${extension}`,
			filters: [{ name: this.currentFormat?.label ?? '媒体文件', extensions: [extension] }]
		});
		if (typeof selected === 'string') {
			this.outputPath = selected;
		}
	}

	async start() {
		if (!this.canStart || !this.input || !this.currentFormat) return;
		this.error = null;
		this.busy = true;
		this.progress = {
			jobId: crypto.randomUUID(),
			ratio: 0,
			outTimeSecs: 0,
			speed: null,
			stage: 'running',
			message: '正在准备转换…'
		};
		try {
			const outcome = await api.startConvert({
				jobId: this.progress.jobId,
				mode: this.mode,
				inputPath: this.input.path,
				outputPath: this.outputPath,
				formatId: this.currentFormat.id,
				audio: this.audio,
				video: this.video
			});
			if (outcome.status === 'cancelled') {
				this.progress = {
					...this.progress,
					stage: 'cancelled',
					message: '已取消转换'
				};
			}
		} catch (error) {
			this.error = invokeErrorMessage(error);
			this.progress = {
				jobId: this.progress?.jobId ?? crypto.randomUUID(),
				ratio: 0,
				outTimeSecs: 0,
				speed: null,
				stage: 'error',
				message: this.error
			};
		} finally {
			this.busy = false;
		}
	}

	async cancel() {
		if (!this.busy) return;
		try {
			await api.cancelConvert();
		} catch (error) {
			this.error = invokeErrorMessage(error);
		}
	}

	async revealOutput() {
		if (!this.outputPath) return;
		await revealItemInDir(this.outputPath);
	}

	clearInput() {
		if (this.busy) return;
		this.input = null;
		this.outputPath = '';
		this.progress = null;
		this.error = null;
	}

	private inputMatchesMode(path: string) {
		const ext = fileExtension(path);
		return extensionsFor(this.mode).includes(ext);
	}

	private async refreshOutputPath() {
		if (!this.input || !this.currentFormat) return;
		this.outputPath = await api.suggestOutputPath(this.input.path, this.currentFormat.extension);
	}
}

export const converter = new ConverterStore();
