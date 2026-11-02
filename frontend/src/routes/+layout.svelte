<script lang="ts">
    import '../app.css';
    import { page, updated } from '$app/state';
    import { beforeNavigate, onNavigate } from '$app/navigation';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { children } = $props();

    // Après un déploiement, un onglet ouvert garderait l'ancien code des pages
    // avec les nouvelles données : on recharge la page à la navigation suivante.
    beforeNavigate(({ willUnload, to }) => {
        if (updated.current && !willUnload && to?.url) location.href = to.url.href;
    });

    // Transition native entre pages du même site (AUTH-29) : fondu du contenu,
    // barre fixe, pastille de la bascule qui glisse (noms dans app.css).
    // `navigated` coupe l'apparition en fondu du premier chargement.
    onNavigate((navigation) => {
        document.documentElement.classList.add('navigated');
        if (!document.startViewTransition || matchMedia('(prefers-reduced-motion: reduce)').matches) return;
        // Ancre seule (onglets de la console) : transition lancée par le lien
        if (navigation.from?.url.pathname === navigation.to?.url.pathname) return;
        return new Promise((resolve) => {
            document.startViewTransition(async () => {
                resolve();
                await navigation.complete;
            });
        });
    });

    // Ces pages portent le toggle dans leur navbar — pas de bouton flottant
    let hasNavbarToggle = $derived(
        page.url.pathname.startsWith('/home/friend') ||
        (page.url.pathname.startsWith('/japprends/') && !page.url.pathname.startsWith('/japprends/login'))
    );
</script>

<svelte:head>
    <!-- Polices MyCss (rv-tokens v1) : Schibsted Grotesk + JetBrains Mono -->
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
    <link href="https://fonts.googleapis.com/css2?family=Schibsted+Grotesk:wght@400..900&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />
</svelte:head>

{@render children()}

{#if !hasNavbarToggle}
    <ThemeToggle />
{/if}
