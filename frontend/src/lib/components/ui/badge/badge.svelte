<script lang="ts">
	import { cn } from '$lib/utils.js';

	type Variant = 'default' | 'success' | 'destructive' | 'warning' | 'outline';

	let {
		variant = 'default' as Variant,
		class: className = '',
		children,
	}: {
		variant?: Variant;
		class?: string;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<span class={cn('badge', `badge--${variant}`, className)}>
	{@render children?.()}
</span>

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 2px 8px;
		border-radius: var(--radius-full);
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid transparent;
		white-space: nowrap;
	}

	.badge--default {
		background: var(--accent-soft);
		color: var(--foreground);
		border-color: var(--border);
	}
	.badge--success {
		background: var(--success-bg);
		color: var(--success);
		border-color: var(--success-border);
	}
	.badge--destructive {
		background: var(--destructive-bg);
		color: var(--destructive);
		border-color: var(--destructive-border);
	}
	.badge--warning {
		background: rgba(217, 168, 76, 0.12);
		color: var(--warning);
		border-color: rgba(217, 168, 76, 0.35);
	}
	.badge--outline {
		background: transparent;
		color: var(--muted-foreground);
		border-color: var(--border);
	}
</style>
