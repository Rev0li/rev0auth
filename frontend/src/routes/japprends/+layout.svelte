<script lang="ts">
    import { page } from '$app/state';
    import AppNav from '$lib/AppNav.svelte';

    let { data, children } = $props();

    // Onglets de la console. Membres (invitations comprises) / Sécurité vivent sur le
    // dashboard (ancre) ; Activité a ses sous-onglets, chacun sa page.
    const TABS = [
        { id: 'members',     label: 'Membres',     href: '/japprends/dashboard#members' },
        { id: 'security',    label: 'Sécurité',    href: '/japprends/dashboard#security' },
        { id: 'activity',    label: 'Activité',    href: '/japprends/songsurf-activity' },
    ] as const;

    const ACTIVITY_SUBTABS = [
        { id: 'songsurf', label: 'SongSurf',   href: '/japprends/songsurf-activity', paths: ['/japprends/songsurf-activity'] },
        { id: 'journal',  label: 'Journal',    href: '/japprends/songsurf-journal',  paths: ['/japprends/songsurf-journal'] },
    ] as const;

    let path = $derived(page.url.pathname);
    let inActivity = $derived(ACTIVITY_SUBTABS.some(s => s.paths.some(p => path.startsWith(p))));
    let active = $derived.by(() => {
        if (inActivity) return 'activity';
        if (path.startsWith('/japprends/dashboard')) {
            const h = page.url.hash.slice(1);
            return TABS.some(t => t.id === h) ? h : 'members';
        }
        return null;
    });

    async function logout() {
        await fetch('/auth/logout', { method: 'POST' });
        location.href = '/';
    }
</script>

{#if data.shell && !path.startsWith('/japprends/login')}
    <AppNav
        pseudo={data.shell.pseudo}
        avatarSrc={data.shell.avatarUrl}
        badge="admin"
        space={{ current: 'console', memberUrl: data.shell.memberUrl, consoleUrl: '/japprends/dashboard' }}
        onLogout={logout}
    >
        {#snippet tabs()}
            {#each TABS as t (t.id)}
                <a aria-current={active === t.id ? 'page' : undefined} href={t.href}>{t.label}</a>
            {/each}
        {/snippet}
    </AppNav>

    {#if inActivity}
        <nav class="subtabs" aria-label="Activité">
            {#each ACTIVITY_SUBTABS as s (s.id)}
                {@const on = s.paths.some(p => path.startsWith(p))}
                <a class="subtab" class:active={on} aria-current={on ? 'page' : undefined} href={s.href}>{s.label}</a>
            {/each}
        </nav>
    {/if}
{/if}

{@render children()}

<style>
    /* Sous-onglets d'Activité : des pastilles en haut de la page, pas une
       deuxième barre (AUTH-28) */
    .subtabs {
        display: flex; gap: 6px; overflow-x: auto;
        max-width: 1100px; margin: 0 auto;
        padding: 1.25rem 1rem 0;
    }
    .subtab {
        height: 28px; padding: 0 12px;
        display: inline-flex; align-items: center;
        border: 1px solid var(--border); border-radius: var(--radius-full, 999px);
        font: 500 0.8125rem/1 var(--font-sans);
        color: var(--muted-foreground); text-decoration: none; white-space: nowrap;
        transition: color 0.15s, border-color 0.15s, background 0.15s;
    }
    .subtab:hover { color: var(--foreground); border-color: var(--primary); }
    .subtab.active { color: var(--foreground); background: var(--card); border-color: var(--primary); }
</style>
