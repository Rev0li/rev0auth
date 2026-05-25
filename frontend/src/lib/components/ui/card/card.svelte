<script lang="ts">
	import { cn } from '$lib/utils.js';

	let {
		class: className = '',
		children,
	}: {
		class?: string;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<div class={cn('card-root', className)}>
	{@render children?.()}
</div>

<style>
	.card-root {
		background: var(--card);
		border: 1px solid var(--border);
		border-radius: var(--radius-xl);
		box-shadow: var(--shadow-soft);
	}
</style>
