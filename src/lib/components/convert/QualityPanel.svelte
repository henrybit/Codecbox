<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import { localizeOptions, t } from '$lib/i18n';
	import Select from '$lib/components/ui/Select.svelte';
	import {
		BITRATE_OPTIONS,
		CHANNEL_OPTIONS,
		FPS_OPTIONS,
		HLS_SEGMENT_OPTIONS,
		PRESET_OPTIONS,
		QUALITY_OPTIONS,
		RESOLUTION_OPTIONS,
		SAMPLE_RATE_OPTIONS
	} from '$lib/config/modes';

	const showVideo = $derived(converter.mode === 'video-to-video');
	const showBitrate = $derived(converter.currentFormat?.supportsAudioBitrate ?? false);
	const isGif = $derived(converter.currentFormat?.id === 'gif');
	const isHls = $derived(converter.currentFormat?.id === 'hls');
	const resolutionOptions = $derived(localizeOptions(RESOLUTION_OPTIONS));
	const fpsOptions = $derived(localizeOptions(FPS_OPTIONS));
	const qualityOptions = $derived(localizeOptions(QUALITY_OPTIONS));
	const presetOptions = $derived(localizeOptions(PRESET_OPTIONS));
	const segmentOptions = $derived(localizeOptions(HLS_SEGMENT_OPTIONS));
	const bitrateOptions = $derived(localizeOptions(BITRATE_OPTIONS));
	const sampleRateOptions = $derived(localizeOptions(SAMPLE_RATE_OPTIONS));
	const channelOptions = $derived(localizeOptions(CHANNEL_OPTIONS));
</script>

<div class="panel">
	{#if showVideo}
		<Select label={t('resolution')} items={resolutionOptions} bind:value={converter.video.resolution} />
		<Select label={t('fps')} items={fpsOptions} bind:value={converter.video.fps} />
		<Select label={t('quality')} items={qualityOptions} bind:value={converter.video.quality} />
		{#if !isGif}
			<Select label={t('preset')} items={presetOptions} bind:value={converter.video.preset} />
		{/if}
		{#if isHls}
			<Select label={t('segment')} items={segmentOptions} bind:value={converter.video.segmentSecs} />
		{/if}
	{/if}

	{#if !isGif}
		{#if showBitrate}
			<Select label={t('bitrate')} items={bitrateOptions} bind:value={converter.audio.bitrate} />
		{/if}
		<Select label={t('sampleRate')} items={sampleRateOptions} bind:value={converter.audio.sampleRate} />
		<Select label={t('channels')} items={channelOptions} bind:value={converter.audio.channels} />
	{/if}
</div>

<style>
	.panel {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
		gap: 0.75rem;
	}
</style>
