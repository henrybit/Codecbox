<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import FileDropZone from '$lib/components/media/FileDropZone.svelte';
	import MediaInfoCard from '$lib/components/media/MediaInfoCard.svelte';
	import FormatGrid from '$lib/components/convert/FormatGrid.svelte';
	import QualityPanel from '$lib/components/convert/QualityPanel.svelte';
	import OutputField from '$lib/components/convert/OutputField.svelte';
	import ProgressPanel from '$lib/components/convert/ProgressPanel.svelte';
</script>

<div class="workspace">
	{#if converter.ffmpeg && !converter.ffmpeg.available}
		<p class="banner">{converter.ffmpeg.message}</p>
	{/if}

	<Card title="源文件">
		<FileDropZone />
		<MediaInfoCard />
	</Card>

	<Card title="输出格式">
		<FormatGrid />
	</Card>

	<Card title="转码参数">
		<QualityPanel />
	</Card>

	<Card title="导出">
		<div class="export">
			<OutputField />
			<div class="actions">
				{#if converter.busy}
					<Button variant="danger" onclick={() => converter.cancel()}>取消转换</Button>
				{:else}
					<Button block disabled={!converter.canStart} onclick={() => converter.start()}>
						开始转换
					</Button>
				{/if}
			</div>
			<ProgressPanel />
			{#if converter.error}
				<p class="error">{converter.error}</p>
			{/if}
		</div>
	</Card>
</div>

<style>
	.workspace {
		display: grid;
		gap: 0.9rem;
	}

	.export {
		display: grid;
		gap: 0.9rem;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
	}

	.banner,
	.error {
		margin: 0;
		padding: 0.8rem 1rem;
		border-radius: 14px;
		font-size: 0.9rem;
	}

	.banner {
		background: rgba(251, 146, 60, 0.12);
		color: var(--amber);
		border: 1px solid rgba(251, 146, 60, 0.28);
	}

	.error {
		background: rgba(251, 113, 133, 0.1);
		color: var(--danger);
	}
</style>
