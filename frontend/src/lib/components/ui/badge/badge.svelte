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
		border-radius: var(--rv-radius-control);
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid transparent;
		white-space: nowrap;
	}

	.badge--default {
		background: var(--rv-turq-soft);
		color: var(--rv-ink);
		border-color: var(--rv-line);
	}
	.badge--success {
		background: var(--rv-success-soft);
		color: var(--rv-success-ink);
		border-color: var(--rv-success);
	}
	.badge--destructive {
		background: var(--rv-danger-soft);
		color: var(--rv-danger-ink);
		border-color: var(--rv-danger);
	}
	.badge--warning {
		background: var(--rv-warning-soft);
		color: var(--rv-warning-ink);
		border-color: var(--rv-warning);
	}
	.badge--outline {
		background: transparent;
		color: var(--rv-muted);
		border-color: var(--rv-line);
	}
</style>
