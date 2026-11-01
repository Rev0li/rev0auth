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
</script>

<nav class="switch" class:on-console={current === 'console'} aria-label="Changer d'espace">
    <span class="thumb" aria-hidden="true"></span>

    {#if current === 'member'}
        <span class="seg active" aria-current="page">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 10.5 12 3l9 7.5V20a1 1 0 0 1-1 1h-5v-6h-6v6H4a1 1 0 0 1-1-1z" /></svg>
            Mon espace
        </span>
        <a class="seg" href={consoleUrl} title="Console admin — réseau privé (Tailscale)">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2 4 5v6c0 5 3.4 9.4 8 11 4.6-1.6 8-6 8-11V5z" /></svg>
            Console
        </a>
    {:else}
        <a class="seg" href={memberUrl} title="Retour à ton espace membre">
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
    /* Pastille sous le côté actif ; elle glisse vers l'autre au survol */
    .thumb {
        position: absolute;
        z-index: -1;
        top: 3px; bottom: 3px; left: 3px;
        width: calc(50% - 3px);
        border-radius: inherit;
        background: var(--card);
        box-shadow: var(--shadow-soft), 0 0 0 1px var(--primary);
        transition: transform 0.25s cubic-bezier(.2, .8, .2, 1);
    }
    .on-console .thumb { transform: translateX(100%); }
    .switch:not(.on-console):has(a.seg:hover) .thumb { transform: translateX(12%); }
    .on-console:has(a.seg:hover) .thumb { transform: translateX(88%); }

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
