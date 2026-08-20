<script lang="ts">
	import { converter } from '$lib/stores/converter.svelte';
	import { MODES } from '$lib/config/modes';

	const icons: Record<string, string> = {
		'audio-to-audio': 'M4 14c3-6 6-9 10-10 4 1 7 4 10 10-3 6-6 9-10 10-4-1-7-4-10-10zm10-3.5a3.5 3.5 0 1 1 0 7 3.5 3.5 0 0 1 0-7z',
		'video-to-audio': 'M4 7h10l6-3v16l-6-3H4V7zm3 11.5v-2h2.2c.7 2 2.3 3.5 4.8 3.5 2.7 0 5-2.2 5-5s-2.3-5-5-5c-1.4 0-2.6.5-3.5 1.4',
		'video-to-video': 'M3 6.5A2.5 2.5 0 0 1 5.5 4h8A2.5 2.5 0 0 1 16 6.5v11A2.5 2.5 0 0 1 13.5 20h-8A2.5 2.5 0 0 1 3 17.5v-11zM17 9l5-3v12l-5-3V9z'
	};
</script>

<aside>
	<p class="kicker">工作模式</p>
	{#each MODES as mode (mode.id)}
		<button
			class="mode {mode.accent}"
			class:active={converter.mode === mode.id}
			disabled={converter.busy}
			onclick={() => converter.setMode(mode.id)}
		>
			<span class="icon">
				<svg viewBox="0 0 24 24" aria-hidden="true">
					<path d={icons[mode.id]} />
				</svg>
			</span>
			<span>
				<strong>{mode.title}</strong>
				<em>{mode.subtitle}</em>
			</span>
		</button>
	{/each}
</aside>

<style>
	aside {
		display: grid;
		align-content: start;
		gap: 0.7rem;
	}

	.kicker {
		margin: 0 0.2rem 0.2rem;
		font-size: 0.72rem;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		color: var(--muted);
	}

	.mode {
		display: grid;
		grid-template-columns: 42px 1fr;
		gap: 0.8rem;
		align-items: center;
		text-align: left;
		padding: 0.9rem;
		border-radius: 18px;
		border: 1px solid transparent;
		background: rgba(255, 255, 255, 0.03);
		color: var(--text);
	}

	.mode em {
		display: block;
		font-style: normal;
		color: var(--muted);
		font-size: 0.8rem;
		margin-top: 0.15rem;
	}

	.mode strong {
		font-size: 0.98rem;
	}

	.icon {
		width: 42px;
		height: 42px;
		border-radius: 14px;
		display: grid;
		place-items: center;
		background: rgba(255, 255, 255, 0.04);
	}

	svg {
		width: 20px;
		height: 20px;
		fill: none;
		stroke: currentColor;
		stroke-width: 1.7;
		stroke-linecap: round;
		stroke-linejoin: round;
	}

	.active {
		border-color: var(--line-strong);
		background: rgba(94, 234, 212, 0.08);
		box-shadow: inset 0 0 0 1px rgba(94, 234, 212, 0.08);
	}

	.amber.active {
		background: rgba(251, 146, 60, 0.1);
	}

	.violet.active {
		background: rgba(196, 181, 253, 0.1);
	}
</style>
