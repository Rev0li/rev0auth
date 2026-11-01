<script lang="ts">
    import { page } from '$app/state';
    import AppNav from '$lib/AppNav.svelte';

    let { data, children } = $props();

    // Onglets de la console. Membres / Invitations / Sécurité vivent sur le
    // dashboard (ancre) ; Activité a ses sous-onglets, chacun sa page.
    const TABS = [
        { id: 'members',     label: 'Membres',     href: '/japprends/dashboard#members' },
        { id: 'invitations', label: 'Invitations', href: '/japprends/dashboard#invitations' },
        { id: 'security',    label: 'Sécurité',    href: '/japprends/dashboard#security' },
        { id: 'activity',    label: 'Activité',    href: '/japprends/songsurf-activity' },
    ] as const;

    const ACTIVITY_SUBTABS = [
        { id: 'songsurf', label: 'SongSurf', href: '/japprends/songsurf-activity',
          paths: ['/japprends/songsurf-activity', '/japprends/songsurf-logs'] },
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
    />

    <nav class="tabs" aria-label="Sections de la console">
        {#each TABS as t (t.id)}
            <a class="tab" class:active={active === t.id} aria-current={active === t.id ? 'page' : undefined} href={t.href}>
                {t.label}
            </a>
        {/each}
    </nav>

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
    .tabs {
        display: flex; overflow-x: auto;
        padding: 0 1rem;
        background: var(--card);
        border-bottom: 1px solid var(--border);
    }
    .tab {
        padding: 0.75rem 1rem;
        border-bottom: 2px solid transparent;
        font: 500 0.875rem/1 var(--font-sans);
        color: var(--muted-foreground);
        text-decoration: none; white-space: nowrap;
        transition: color 0.15s, border-color 0.15s;
    }
    .tab:hover { color: var(--foreground); }
    .tab.active { color: var(--foreground); border-bottom-color: var(--primary); }

    .subtabs {
        display: flex; gap: 6px; overflow-x: auto;
        padding: 0.625rem 1.5rem;
        border-bottom: 1px solid var(--border);
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
