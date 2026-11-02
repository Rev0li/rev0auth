<script lang="ts">
	import { cn } from '$lib/utils.js';

	let {
		type = 'text',
		value = $bindable(''),
		placeholder = '',
		disabled = false,
		required = false,
		name = '',
		id = '',
		autocomplete = '',
		class: className = '',
		oninput,
		onkeydown,
	}: {
		type?: string;
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		required?: boolean;
		name?: string;
		id?: string;
		autocomplete?: import('svelte/elements').HTMLInputAttributes['autocomplete'];
		class?: string;
		oninput?: (e: Event) => void;
		onkeydown?: (e: KeyboardEvent) => void;
	} = $props();
</script>

<input
	{type}
	bind:value
	{placeholder}
	{disabled}
	{required}
	{name}
	{id}
	{autocomplete}
	class={cn('inp', className)}
	{oninput}
	{onkeydown}
/>

<style>
	.inp {
		width: 100%;
		border: 1px solid var(--rv-line);
		border-radius: var(--rv-radius-sm);
		padding: 8px 12px;
		font: inherit;
		font-size: 0.9375rem;
		background: var(--rv-panel);
		color: var(--rv-ink);
		outline: none;
		transition: border-color 0.15s, box-shadow 0.15s;
	}
	.inp:focus {
		border-color: var(--rv-turq-ink);
		box-shadow: 0 0 0 3px var(--rv-turq-soft);
		background: var(--rv-surface);
	}
	.inp:disabled { opacity: 0.5; cursor: not-allowed; }
	.inp::placeholder { color: var(--rv-muted); }
</style>
