<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		variant?: 'primary' | 'ghost' | 'danger' | 'soft';
		disabled?: boolean;
		onclick?: () => void;
		children: Snippet;
		type?: 'button' | 'submit';
		block?: boolean;
	};

	let {
		variant = 'primary',
		disabled = false,
		onclick,
		children,
		type = 'button',
		block = false
	}: Props = $props();
</script>

<button {type} class="btn {variant}" class:block {disabled} {onclick}>
	{@render children()}
</button>

<style>
	.btn {
		border: 0;
		border-radius: 999px;
		padding: 0.7rem 1.1rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.45rem;
		transition:
			transform 0.18s ease,
			background 0.18s ease,
			opacity 0.18s ease;
	}

	.btn:hover:not(:disabled) {
		transform: translateY(-1px);
	}

	.btn:disabled {
		opacity: 0.45;
		cursor: not-allowed;
	}

	.block {
		width: 100%;
	}

	.primary {
		background: linear-gradient(135deg, #5eead4, #2dd4bf 55%, #fb923c);
		color: #06201d;
		font-weight: 700;
	}

	.ghost {
		background: transparent;
		border: 1px solid var(--line-strong);
		color: var(--text);
	}

	.soft {
		background: rgba(94, 234, 212, 0.12);
		color: var(--teal);
	}

	.danger {
		background: rgba(251, 113, 133, 0.16);
		color: var(--danger);
	}
</style>
