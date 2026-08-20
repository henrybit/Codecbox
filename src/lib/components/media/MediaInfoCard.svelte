<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatBytes, formatDuration } from '$lib/utils/format';
</script>

{#if converter.input}
	<article>
		<div>
			<p class="name">{converter.input.fileName}</p>
			<p class="meta">
				{formatBytes(converter.input.fileSize)}
				· {formatDuration(converter.input.durationSecs)}
				{#if converter.input.video}
					· {converter.input.video.width}×{converter.input.video.height}
					· {converter.input.video.codec}
				{/if}
				{#if converter.input.audio}
					· {converter.input.audio.codec}
					{#if converter.input.audio.sampleRate}
						· {converter.input.audio.sampleRate} Hz
					{/if}
				{/if}
			</p>
		</div>
		<Button variant="ghost" onclick={() => converter.clearInput()}>更换文件</Button>
	</article>
{/if}

<style>
	article {
		margin-top: 0.9rem;
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		align-items: center;
		padding: 0.9rem 1rem;
		border-radius: 16px;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid var(--line);
	}

	.name {
		margin: 0 0 0.25rem;
		font-weight: 700;
		word-break: break-all;
	}

	.meta {
		margin: 0;
		color: var(--muted);
		font-size: 0.82rem;
		font-family: var(--mono);
	}
</style>
