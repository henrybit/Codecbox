<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { formatDuration } from '$lib/utils/format';

	const percent = $derived(Math.round((converter.progress?.ratio ?? 0) * 100));
	const done = $derived(converter.progress?.stage === 'done');
</script>

{#if converter.progress}
	<div class="progress" class:done>
		<div class="bar">
			<span style:width="{percent}%"></span>
		</div>
		<div class="copy">
			<strong>{converter.progress.message ?? '处理中'}</strong>
			<span>
				{percent}%
				{#if converter.progress.outTimeSecs}
					· {formatDuration(converter.progress.outTimeSecs)}
				{/if}
				{#if converter.progress.speed}
					· {converter.progress.speed}
				{/if}
			</span>
		</div>
		{#if done}
			<Button variant="soft" onclick={() => converter.revealOutput()}>在文件夹中显示</Button>
		{/if}
	</div>
{/if}

<style>
	.progress {
		display: grid;
		gap: 0.7rem;
	}

	.bar {
		height: 8px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.06);
		overflow: hidden;
	}

	.bar span {
		display: block;
		height: 100%;
		border-radius: inherit;
		background: linear-gradient(90deg, var(--teal), var(--amber));
		transition: width 0.2s ease;
	}

	.copy {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.86rem;
	}

	.copy span {
		color: var(--muted);
		font-family: var(--mono);
	}

	.done strong {
		color: var(--ok);
	}
</style>
