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
		background: var(--rv-surface);
		border: 1px solid var(--rv-line);
		border-radius: var(--rv-radius-window);
		box-shadow: none;
	}
</style>
