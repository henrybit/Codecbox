<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="shell">
	<div class="orb teal"></div>
	<div class="orb amber"></div>
	{@render children()}
</div>

<style>
	.shell {
		position: relative;
		min-height: 100vh;
		padding: 1.15rem 1.2rem 1.3rem;
		overflow: hidden;
	}

	.orb {
		position: absolute;
		width: 280px;
		height: 280px;
		filter: blur(40px);
		opacity: 0.18;
		pointer-events: none;
	}

	.teal {
		left: -80px;
		top: -90px;
		background: #2dd4bf;
	}

	.amber {
		right: -70px;
		bottom: -80px;
		background: #fb923c;
	}
</style>
