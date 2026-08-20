<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import Button from '$lib/components/ui/Button.svelte';
</script>

<label>
	<span>输出路径</span>
	<div class="row">
		<input bind:value={converter.outputPath} placeholder="选择或输入输出文件路径" disabled={converter.busy} />
		<Button variant="ghost" onclick={() => converter.pickOutput()}>浏览</Button>
	</div>
	{#if converter.currentFormat?.id === 'hls'}
		<p class="hint">HLS 会生成 .m3u8 播放列表，以及同目录下的 .ts 分片文件。</p>
	{/if}
</label>

<style>
	label {
		display: grid;
		gap: 0.45rem;
		color: var(--muted);
		font-size: 0.82rem;
	}

	.row {
		display: grid;
		grid-template-columns: 1fr auto;
		gap: 0.6rem;
	}

	input {
		border: 1px solid var(--line);
		background: rgba(8, 12, 16, 0.65);
		border-radius: 14px;
		padding: 0.75rem 0.9rem;
	}

	.hint {
		margin: 0;
		font-size: 0.75rem;
		line-height: 1.4;
	}
</style>

