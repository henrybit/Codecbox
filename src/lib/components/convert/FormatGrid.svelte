<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
</script>

<div class="grid">
	{#each converter.formats as format (format.id)}
		<button
			class:active={converter.formatId === format.id}
			disabled={converter.busy}
			onclick={() => converter.selectFormat(format.id)}
		>
			<strong>{format.label}</strong>
			<span>.{format.extension}</span>
			<em>{format.description}</em>
		</button>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(132px, 1fr));
		gap: 0.65rem;
	}

	button {
		text-align: left;
		padding: 0.85rem 0.8rem;
		border-radius: 16px;
		border: 1px solid var(--line);
		background: rgba(8, 12, 16, 0.35);
		min-height: 108px;
	}

	strong {
		display: block;
		font-size: 1rem;
	}

	span {
		display: block;
		margin: 0.15rem 0 0.45rem;
		color: var(--teal);
		font-family: var(--mono);
		font-size: 0.75rem;
	}

	em {
		display: block;
		font-style: normal;
		color: var(--muted);
		font-size: 0.75rem;
		line-height: 1.35;
	}

	.active {
		border-color: var(--teal);
		background: rgba(45, 212, 191, 0.1);
		box-shadow: 0 0 0 1px rgba(94, 234, 212, 0.2);
	}
</style>
