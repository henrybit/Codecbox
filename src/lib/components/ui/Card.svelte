<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		action,
		children
	}: {
		title?: string;
		action?: Snippet;
		children: Snippet;
	} = $props();
</script>

<section class="card">
	{#if title || action}
		<header>
			{#if title}<h2>{title}</h2>{/if}
			{#if action}<div>{@render action()}</div>{/if}
		</header>
	{/if}
	{@render children()}
</section>

<style>
	.card {
		background: var(--bg-panel);
		border: 1px solid var(--line);
		border-radius: var(--radius);
		padding: 1.1rem 1.15rem;
		backdrop-filter: blur(18px);
		box-shadow: var(--shadow);
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.9rem;
	}

	h2 {
		margin: 0;
		font-size: 0.78rem;
		letter-spacing: 0.16em;
		text-transform: uppercase;
		color: var(--muted);
		font-weight: 700;
	}
</style>
