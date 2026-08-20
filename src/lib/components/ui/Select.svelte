<script lang="ts">
	type Item = { value: string; label: string };

	let {
		label,
		items,
		value = $bindable()
	}: {
		label: string;
		items: Item[];
		value: string;
	} = $props();
</script>

<label class="field">
	<span>{label}</span>
	<select bind:value>
		{#each items as item (item.value)}
			<option value={item.value}>{item.label}</option>
		{/each}
	</select>
</label>

<style>
	.field {
		display: grid;
		gap: 0.4rem;
		font-size: 0.82rem;
		color: var(--muted);
	}

	select {
		appearance: none;
		border: 1px solid var(--line);
		background:
			linear-gradient(45deg, transparent 50%, var(--muted) 50%) right 12px center / 5px 5px no-repeat,
			linear-gradient(135deg, transparent 50%, var(--muted) 50%) right 7px center / 5px 5px no-repeat,
			rgba(8, 12, 16, 0.65);
		color: var(--text);
		border-radius: 14px;
		padding: 0.7rem 2rem 0.7rem 0.85rem;
	}
</style>
