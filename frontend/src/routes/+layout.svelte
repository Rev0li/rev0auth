<script lang="ts">
    import '../app.css';
    import { page, updated } from '$app/state';
    import { beforeNavigate } from '$app/navigation';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { children } = $props();

    // Après un déploiement, un onglet ouvert garderait l'ancien code des pages
    // avec les nouvelles données : on recharge la page à la navigation suivante.
    beforeNavigate(({ willUnload, to }) => {
        if (updated.current && !willUnload && to?.url) location.href = to.url.href;
    });

    // Ces pages portent le toggle dans leur navbar — pas de bouton flottant
    let hasNavbarToggle = $derived(
        page.url.pathname.startsWith('/home/friend') ||
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
