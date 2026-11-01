<script lang="ts">
    // Bascule admin entre l'espace membre (portail public) et la console
    // (nom privé, AUTH-11). Les deux côtés sont toujours visibles : celui où
    // l'on est est surligné, l'autre est un lien. Pas de test de joignabilité :
    // le navigateur bloque les requêtes d'une page publique vers une adresse
    // Tailscale (Local Network Access), le test répondait toujours « non ».
    let { current, memberUrl, consoleUrl }: {
        current: 'member' | 'console';
        memberUrl: string;
        consoleUrl: string;
    } = $props();

    // Transition (AUTH-29). Même site (console → espace membre servi sur le
    // nom de la console) : SvelteKit navigue sans recharger et la transition
    // native fait glisser la pastille (view-transition-name). Autre site
    // (portail public → console) : on fait glisser la pastille et s'effacer la
    // page, puis seulement le navigateur change de page ; l'autre côté apparaît
    // en fondu (app.css). Rien de tout ça si le système demande moins d'animations.
    let leaving = $state(false);

    function go(e: MouseEvent) {
        const a = e.currentTarget as HTMLAnchorElement;
        if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
        if (new URL(a.href).origin === location.origin) return;
        if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
        e.preventDefault();
        leaving = true;
        document.documentElement.classList.add('page-leaving');
        setTimeout(() => { location.href = a.href; }, 220);
    }

    // Retour arrière depuis le cache du navigateur : la page revient telle
    // qu'on l'a quittée (effacée) ; on la remet en état.
    $effect(() => {
        const restore = (e: PageTransitionEvent) => {
            if (!e.persisted) return;
            leaving = false;
            document.documentElement.classList.remove('page-leaving');
        };
        addEventListener('pageshow', restore);
        return () => removeEventListener('pageshow', restore);
    });
</script>

<nav class="switch" class:on-console={(current === 'console') !== leaving} class:leaving aria-label="Changer d'espace">
    <span class="thumb" aria-hidden="true"></span>

    {#if current === 'member'}
        <span class="seg active" aria-current="page">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z" /></svg>
            Mon espace
        </span>
        <a class="seg" href={consoleUrl} onclick={go} title="Console admin — réseau privé (Tailscale)">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2 4 5v6c0 5 3.4 9.4 8 11 4.6-1.6 8-6 8-11V5z" /></svg>
            Console
        </a>
    {:else}
        <a class="seg" href={memberUrl} onclick={go} title="Retour à ton espace membre">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z" /></svg>
            Mon espace
        </a>
        <span class="seg active" aria-current="page">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2 4 5v6c0 5 3.4 9.4 8 11 4.6-1.6 8-6 8-11V5z" /></svg>
            Console
        </span>
    {/if}
</nav>

<style>
    .switch {
        position: relative;
        display: inline-grid;
        grid-template-columns: 1fr 1fr;
        padding: 3px;
        border: 1px solid var(--border);
        border-radius: var(--radius-full, 999px);
        background: var(--muted);
        isolation: isolate;
    }
    /* Pastille sous le côté actif ; elle ne glisse qu'au changement d'espace
       (AUTH-30 : plus de début de course au survol) */
    .thumb {
        position: absolute;
        z-index: -1;
        top: 3px; bottom: 3px; left: 3px;
        width: calc(50% - 3px);
        border-radius: inherit;
        background: var(--card);
        box-shadow: var(--shadow-soft), 0 0 0 1px var(--primary);
        transition: transform 0.25s cubic-bezier(.2, .8, .2, 1);
        view-transition-name: space-thumb;
    }
    .on-console .thumb { transform: translateX(100%); }

    .seg {
        display: inline-flex; align-items: center; justify-content: center; gap: 6px;
        height: 28px; padding: 0 14px;
        border-radius: inherit;
        font: 600 0.8125rem/1 var(--font-sans);
        white-space: nowrap;
        color: var(--muted-foreground);
        text-decoration: none;
        transition: color 0.15s;
    }
    .seg.active { color: var(--foreground); cursor: default; }
    a.seg:hover, a.seg:focus-visible { color: var(--primary-hover); outline: none; }
    a.seg:focus-visible { box-shadow: 0 0 0 2px var(--primary); }

    svg {
        width: 14px; height: 14px; flex-shrink: 0;
        fill: none; stroke: currentColor; stroke-width: 2;
        stroke-linecap: round; stroke-linejoin: round;
    }
    .seg.active svg { color: var(--primary-hover); }

    @media (max-width: 520px) {
        .seg { padding: 0 10px; }
    }
</style>
