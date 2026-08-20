<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
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
</script>

<div class="panel">
	{#if showVideo}
		<Select label="分辨率" items={RESOLUTION_OPTIONS} bind:value={converter.video.resolution} />
		<Select label="帧率" items={FPS_OPTIONS} bind:value={converter.video.fps} />
		<Select label="画质" items={QUALITY_OPTIONS} bind:value={converter.video.quality} />
		{#if !isGif}
			<Select label="编码速度" items={PRESET_OPTIONS} bind:value={converter.video.preset} />
		{/if}
		{#if isHls}
			<Select label="分片时长" items={HLS_SEGMENT_OPTIONS} bind:value={converter.video.segmentSecs} />
		{/if}
	{/if}

	{#if !isGif}
		{#if showBitrate}
			<Select label="音频码率" items={BITRATE_OPTIONS} bind:value={converter.audio.bitrate} />
		{/if}
		<Select label="采样率" items={SAMPLE_RATE_OPTIONS} bind:value={converter.audio.sampleRate} />
		<Select label="声道" items={CHANNEL_OPTIONS} bind:value={converter.audio.channels} />
	{/if}
</div>

<style>
	.panel {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
		gap: 0.75rem;
	}
</style>
