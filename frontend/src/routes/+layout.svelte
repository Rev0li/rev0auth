<script lang="ts">
    import '../app.css';
    import { page } from '$app/state';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { children } = $props();

    // Ces pages portent le toggle dans leur navbar — pas de bouton flottant
    let hasNavbarToggle = $derived(
        page.url.pathname.startsWith('/home/friend') ||
        page.url.pathname.startsWith('/members/profile') ||
        page.url.pathname.startsWith('/japprends/dashboard')
    );
</script>

<svelte:head>
    <link rel="preconnect" href="https://fonts.bunny.net" />
    <link href="https://fonts.bunny.net/css?family=geist:400,500,600,700|geist-mono:400,500" rel="stylesheet" />
</svelte:head>

{@render children()}

{#if !hasNavbarToggle}
    <ThemeToggle />
{/if}
