<script lang="ts">
    // Barre du haut commune à l'espace membre et à la console : mêmes places
    // pour l'avatar, le pseudo, la bascule d'espace, le thème et la déconnexion.
    import { onMount, type Snippet } from 'svelte';
    import ThemeToggle from '$lib/ThemeToggle.svelte';
    import SpaceSwitch from '$lib/SpaceSwitch.svelte';

    // Présence (Activité › Journal) : un battement par minute tant que
    // l'onglet est visible, pour qu'une page ouverte compte comme « en ligne ».
    onMount(() => {
        const ping = () => {
            if (document.visibilityState === 'visible') fetch('/auth/ping', { method: 'POST' }).catch(() => {});
        };
        const timer = setInterval(ping, 60_000);
        document.addEventListener('visibilitychange', ping);
        return () => { clearInterval(timer); document.removeEventListener('visibilitychange', ping); };
    });

    let { pseudo, avatarSrc, badge = null, space = null, tabs, onLogout }: {
        pseudo: string;
        avatarSrc: string;
        badge?: string | null;
        // Bascule « Mon espace | Console », admins seulement
        space?: { current: 'member' | 'console'; memberUrl: string; consoleUrl: string } | null;
        // Onglets de la page (console, AUTH-28) : dans la barre, après le logo ;
        // sur téléphone, sur une deuxième ligne de la même barre.
        tabs?: Snippet;
        onLogout: () => void;
    } = $props();
</script>

<nav class="navbar" class:has-tabs={!!tabs}>
    <span class="navbar-brand">rev0auth{#if badge}<span class="navbar-badge">{badge}</span>{/if}</span>
    {#if tabs}
        <div class="navbar-tabs">{@render tabs()}</div>
    {/if}
    <div class="navbar-right">
        <img
            src={avatarSrc}
            alt="Avatar"
            class="nav-avatar"
            onerror={(e) => (e.currentTarget as HTMLImageElement).style.display = 'none'}
        />
        <span class="nav-pseudo">{pseudo}</span>
        {#if space}
            <SpaceSwitch {...space} />
        {/if}
        <ThemeToggle inline />
        <button class="nav-btn nav-btn-logout" onclick={onLogout}>Déconnexion</button>
    </div>
</nav>

<style>
    .navbar {
        position: sticky;
        top: 0;
        z-index: 100;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 1rem;
        padding: 0 1.5rem;
        height: 56px;
        background: var(--rv-surface);
        border-bottom: 1px solid var(--rv-line);
        box-shadow: none;
    }
    .navbar-brand { font-weight: 700; font-size: 1rem; letter-spacing: -0.01em; white-space: nowrap; }
    .navbar-badge {
        margin-left: 8px;
        font-size: 0.6875rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.06em; background: var(--rv-rose);
        color: var(--rv-on-accent); border-radius: 99px; padding: 2px 7px;
        vertical-align: middle;
    }
    .navbar-right { display: flex; align-items: center; gap: 10px; }

    /* Onglets : collés au logo, soulignés en bas de la barre */
    .navbar.has-tabs { justify-content: flex-start; }
    .navbar.has-tabs .navbar-right { margin-left: auto; }
    .navbar-tabs {
        align-self: stretch;
        display: flex; align-items: stretch;
        margin-left: 1rem;
        overflow-x: auto; scrollbar-width: none;
    }
    .navbar-tabs :global(a) {
        display: flex; align-items: center;
        padding: 0 0.875rem;
        border-bottom: 2px solid transparent;
        font: 500 0.875rem/1 var(--rv-font-sans);
        color: var(--rv-muted);
        text-decoration: none; white-space: nowrap;
        transition: color 0.15s, border-color 0.15s;
    }
    .navbar-tabs :global(a:hover) { color: var(--rv-ink); }
    .navbar-tabs :global(a[aria-current='page']) { color: var(--rv-ink); border-bottom-color: var(--rv-turq-ink); }
    .nav-avatar {
        width: 28px; height: 28px;
        border-radius: 50%;
        object-fit: cover;
        border: 1px solid var(--rv-line);
    }
    .nav-pseudo { font-size: 0.875rem; font-weight: 600; }
    .nav-btn {
        height: 30px; padding: 0 12px;
        border: 1px solid var(--rv-line);
        border-radius: var(--rv-radius-control);
        background: transparent;
        font: 500 0.8125rem/1 var(--rv-font-sans);
        color: var(--rv-ink);
        cursor: pointer;
        transition: background 0.15s;
    }
    .nav-btn:hover { background: var(--rv-panel); }
    .nav-btn-logout { color: var(--rv-danger-ink); border-color: var(--rv-danger); }

    @media (max-width: 640px) {
        .navbar { padding: 0 0.75rem; }
        .nav-avatar, .nav-pseudo { display: none; }

        /* Une seule barre, deux lignes : logo + boutons, puis les onglets */
        .navbar.has-tabs { flex-wrap: wrap; height: auto; row-gap: 0; padding-top: 0.75rem; }
        .navbar.has-tabs .navbar-tabs { order: 3; flex-basis: 100%; margin: 0.5rem -0.75rem 0; padding: 0 0.25rem; }
        .navbar-tabs :global(a) { padding: 0.75rem 0.75rem; }
    }
</style>
