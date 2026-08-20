<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { converter } from '$lib/stores/converter.svelte';

	let unlisten: (() => void) | undefined;

	onMount(async () => {
		try {
			unlisten = await getCurrentWebviewWindow().onDragDropEvent((event) => {
				if (event.payload.type === 'over') {
					converter.dragActive = true;
				} else if (event.payload.type === 'drop') {
					converter.dragActive = false;
					const path = event.payload.paths[0];
					if (path) void converter.loadInput(path);
				} else {
					converter.dragActive = false;
				}
			});
		} catch {
			// Browser preview has no Tauri drag-drop API.
		}
	});

	onDestroy(() => unlisten?.());
</script>

<button
	class="drop"
	class:active={converter.dragActive}
	onclick={() => converter.pickInput()}
	disabled={converter.busy}
>
	<div class="wave" aria-hidden="true">
		<svg viewBox="0 0 200 48">
			<path d="M0 24c16 0 16-16 32-16s16 32 32 32 16-32 32-32 16 32 32 32 16-32 32-32 16 16 32 16" />
		</svg>
	</div>
	<p class="title">拖入{converter.modeInfo.accept === 'audio' ? '音频' : '视频'}，或点击选择</p>
	<p class="hint">{converter.modeInfo.hint}</p>
	<span class="cta">选择文件</span>
</button>

<style>
	.drop {
		position: relative;
		width: 100%;
		min-height: 168px;
		border: 1px dashed var(--line-strong);
		border-radius: 20px;
		background: rgba(6, 10, 14, 0.35);
		overflow: hidden;
		padding: 1.4rem 1.2rem 1.2rem;
		text-align: left;
		display: grid;
		align-content: center;
		gap: 0.35rem;
	}

	.drop.active {
		border-color: var(--teal);
		background: rgba(45, 212, 191, 0.08);
	}

	.wave {
		position: absolute;
		right: -12px;
		top: 18px;
		opacity: 0.35;
		color: var(--teal);
		width: 180px;
	}

	.wave path {
		fill: none;
		stroke: currentColor;
		stroke-width: 2;
	}

	.title {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 700;
	}

	.hint {
		margin: 0 0 0.6rem;
		color: var(--muted);
		font-size: 0.88rem;
	}

	.cta {
		pointer-events: none;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.55rem 0.95rem;
		border-radius: 999px;
		background: rgba(94, 234, 212, 0.12);
		color: var(--teal);
		width: fit-content;
		font-weight: 700;
		font-size: 0.88rem;
	}
</style>
