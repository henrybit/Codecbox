<script lang="ts">
	import AppShell from '$lib/components/layout/AppShell.svelte';
	import LanguageSwitch from '$lib/components/layout/LanguageSwitch.svelte';
	import ModeRail from '$lib/components/layout/ModeRail.svelte';
	import ConverterWorkspace from '$lib/components/convert/ConverterWorkspace.svelte';
	import { t } from '$lib/i18n';
	import { converter } from '$lib/stores/converter.svelte';
</script>

<AppShell>
	<div class="page">
		<header>
			<div>
				<p class="brand">CodecBox</p>
				<h1>{t('appTitle')}</h1>
			</div>
			<div class="tools">
				<LanguageSwitch />
				<div class="status" class:ok={converter.ffmpeg?.available}>
					<span></span>
					{#if !converter.ffmpeg}
						{t('ffmpegChecking')}
					{:else if converter.ffmpeg.available}
						{t('ffmpegConnected')}
					{:else}
						{t('ffmpegUnavailable')}
					{/if}
				</div>
			</div>
		</header>

		<main>
			<ModeRail />
			<ConverterWorkspace />
		</main>
	</div>
</AppShell>

<style>
	.page {
		position: relative;
		z-index: 1;
		display: grid;
		gap: 1rem;
		max-width: 1180px;
		margin: 0 auto;
	}

	header {
		display: flex;
		justify-content: space-between;
		align-items: end;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.tools {
		display: flex;
		align-items: center;
		gap: 0.85rem;
	}

	.brand {
		margin: 0 0 0.2rem;
		letter-spacing: 0.28em;
		font-size: 0.7rem;
		color: var(--teal);
	}

	h1 {
		margin: 0;
		font-size: 1.8rem;
		letter-spacing: -0.03em;
	}

	.status {
		display: flex;
		align-items: center;
		gap: 0.45rem;
		color: var(--muted);
		font-size: 0.82rem;
	}

	.status span {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--amber);
	}

	.status.ok span {
		background: var(--ok);
		box-shadow: 0 0 10px var(--ok);
	}

	main {
		display: grid;
		grid-template-columns: 250px minmax(0, 1fr);
		gap: 1rem;
		align-items: start;
	}

	@media (max-width: 860px) {
		main {
			grid-template-columns: 1fr;
		}
	}
</style>
