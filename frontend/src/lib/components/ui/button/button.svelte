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
		font-family: var(--font-sans);
		font-weight: 600;
		border: 1px solid transparent;
		border-radius: var(--radius-full);
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
		color: var(--primary-foreground);
		background: var(--primary);
		box-shadow: var(--shadow-soft);
	}
	.btn--primary:hover {
		background: var(--primary-hover);
		box-shadow: var(--shadow-hover);
		transform: scale(1.02);
	}

	.btn--secondary {
		color: var(--foreground);
		background: var(--card);
		border-color: var(--border);
	}
	.btn--secondary:hover {
		background: var(--muted);
		box-shadow: var(--shadow-soft);
	}

	.btn--ghost {
		color: var(--foreground);
		background: transparent;
	}
	.btn--ghost:hover { background: var(--overlay); }

	.btn--destructive {
		color: white;
		background: var(--destructive);
	}
	.btn--destructive:hover { opacity: 0.88; }
</style>
