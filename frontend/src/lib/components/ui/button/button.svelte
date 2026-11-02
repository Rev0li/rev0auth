<script lang="ts">
	import { cn } from '$lib/utils.js';

	type Variant = 'primary' | 'secondary' | 'ghost' | 'destructive';
	type Size = 'sm' | 'md' | 'lg';

	let {
		variant = 'primary' as Variant,
		size = 'md' as Size,
		disabled = false,
		fullWidth = false,
		type = 'button' as 'button' | 'submit' | 'reset',
		class: className = '',
		onclick,
		children,
	}: {
		variant?: Variant;
		size?: Size;
		disabled?: boolean;
		fullWidth?: boolean;
		type?: 'button' | 'submit' | 'reset';
		class?: string;
		onclick?: (e: MouseEvent) => void;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<button
	{type}
	{disabled}
	class={cn('btn', `btn--${variant}`, `btn--${size}`, fullWidth && 'btn--full', className)}
	{onclick}
>
	{@render children?.()}
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		font-family: var(--rv-font-sans);
		font-weight: 600;
		border: 1px solid transparent;
		border-radius: var(--rv-radius-control);
		cursor: pointer;
		white-space: nowrap;
		text-decoration: none;
		transition: background 0.15s, box-shadow 0.15s, transform 0.12s;
	}
	.btn--full { width: 100%; }
	.btn:disabled { opacity: 0.45; pointer-events: none; }
	.btn:active   { transform: scale(0.98); }

	/* Sizes */
	.btn--sm { height: 32px; padding: 0 14px; font-size: 0.8125rem; }
	.btn--md { height: 40px; padding: 0 22px; font-size: 0.9375rem; }
	.btn--lg { height: 48px; padding: 0 28px; font-size: 1rem; }

	/* Variants */
	.btn--primary {
		color: var(--rv-on-accent);
		background: var(--rv-turq);
		box-shadow: none;
	}
	.btn--primary:hover {
		box-shadow: var(--rv-shadow-gel);
		transform: scale(1.02);
	}

	.btn--secondary {
		color: var(--rv-ink);
		background: var(--rv-surface);
		border-color: var(--rv-line);
	}
	.btn--secondary:hover {
		background: var(--rv-panel);
		box-shadow: none;
	}

	.btn--ghost {
		color: var(--rv-ink);
		background: transparent;
	}
	.btn--ghost:hover { background: var(--rv-line); }

	.btn--destructive {
		color: white;
		background: var(--rv-danger);
	}
	.btn--destructive:hover { opacity: 0.88; }
</style>
