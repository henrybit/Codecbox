<script lang="ts">
	import { t } from '$lib/i18n';
	import { converter } from '$lib/stores/converter.svelte';
	import Button from '$lib/components/ui/Button.svelte';
</script>

<label>
	<span>{t('outputPath')}</span>
	<div class="row">
		<input bind:value={converter.outputPath} placeholder={t('outputPlaceholder')} disabled={converter.busy} />
		<Button variant="ghost" onclick={() => converter.pickOutput()}>{t('browse')}</Button>
	</div>
	{#if converter.currentFormat?.id === 'hls'}
		<p class="hint">{t('hlsHint')}</p>
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

