<script lang="ts">
	import { locale, t, type Locale } from '$lib/i18n';

	const options: { id: Locale; label: string }[] = [
		{ id: 'en', label: 'EN' },
		{ id: 'zh', label: '中文' }
	];
</script>

<div class="lang" role="group" aria-label={t('language')}>
	{#each options as option (option.id)}
		<button
			type="button"
			class:active={locale.current === option.id}
			aria-pressed={locale.current === option.id}
			onclick={() => locale.set(option.id)}
		>
			{option.label}
		</button>
	{/each}
</div>

<style>
	.lang {
		display: inline-flex;
		padding: 3px;
		border-radius: 999px;
		border: 1px solid var(--line);
		background: rgba(8, 12, 16, 0.45);
	}

	button {
		border: 0;
		background: transparent;
		color: var(--muted);
		border-radius: 999px;
		padding: 0.28rem 0.7rem;
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.04em;
	}

	button.active {
		background: rgba(94, 234, 212, 0.16);
		color: var(--teal);
	}
</style>
