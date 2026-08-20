<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import '$lib/styles/app.css';
	import { onDestroy, onMount } from 'svelte';
	import { converter } from '$lib/stores/converter.svelte';

	let { children } = $props();

	onMount(() => {
		void converter.init();
	});

	onDestroy(() => converter.destroy());
</script>

<svelte:head>
	<title>音视频转换器</title>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children()}
