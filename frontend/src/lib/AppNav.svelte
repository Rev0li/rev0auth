<script lang="ts">
    // Barre du haut commune à l'espace membre et à la console : mêmes places
    // pour l'avatar, le pseudo, la bascule d'espace, le thème et la déconnexion.
    import { onMount } from 'svelte';
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

    let { pseudo, avatarSrc, badge = null, space = null, onLogout }: {
        pseudo: string;
        avatarSrc: string;
        badge?: string | null;
        // Bascule « Mon espace | Console », admins seulement
        space?: { current: 'member' | 'console'; memberUrl: string; consoleUrl: string } | null;
        onLogout: () => void;
    } = $props();
</script>

<nav class="navbar">
    <span class="navbar-brand">rev0auth{#if badge}<span class="navbar-badge">{badge}</span>{/if}</span>
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
        background: var(--card);
        border-bottom: 1px solid var(--border);
        box-shadow: var(--shadow-soft);
    }
    .navbar-brand { font-weight: 700; font-size: 1rem; letter-spacing: -0.01em; white-space: nowrap; }
    .navbar-badge {
        margin-left: 8px;
        font-size: 0.6875rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.06em; background: rgba(160,120,255,0.15);
        color: #a078ff; border-radius: 99px; padding: 2px 7px;
        vertical-align: middle;
    }
    .navbar-right { display: flex; align-items: center; gap: 10px; }
    .nav-avatar {
        width: 28px; height: 28px;
        border-radius: 50%;
        object-fit: cover;
        border: 1px solid var(--border);
    }
    .nav-pseudo { font-size: 0.875rem; font-weight: 600; }
    .nav-btn {
        height: 30px; padding: 0 12px;
        border: 1px solid var(--border);
        border-radius: var(--radius-full);
        background: transparent;
        font: 500 0.8125rem/1 var(--font-sans);
        color: var(--foreground);
        cursor: pointer;
        transition: background 0.15s;
    }
    .nav-btn:hover { background: var(--muted); }
    .nav-btn-logout { color: var(--destructive); border-color: var(--destructive-border); }

    @media (max-width: 640px) {
        .navbar { padding: 0 0.75rem; }
        .nav-avatar, .nav-pseudo { display: none; }
    }
</style>
