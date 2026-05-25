<script lang="ts">
    import { onMount } from 'svelte';
    import MembersTab     from './MembersTab.svelte';
    import WallTab        from './WallTab.svelte';
    import MessagesTab    from './MessagesTab.svelte';
    import DonationsTab   from './DonationsTab.svelte';
    import InvitationsTab from './InvitationsTab.svelte';

    const TABS = ['members', 'wall', 'messages', 'donations', 'invitations'] as const;
    type Tab = typeof TABS[number];

    const TAB_LABELS: Record<Tab, string> = {
        members:     'Membres',
        wall:        'Mur',
        messages:    'Messages',
        donations:   'Donations',
        invitations: 'Invitations',
    };

    let { data } = $props();
    let active = $state<Tab>('members');

    onMount(() => {
        const hash = location.hash.slice(1) as Tab;
        if (TABS.includes(hash)) active = hash;
    });

    function setTab(tab: Tab) {
        active = tab;
        history.replaceState(null, '', `#${tab}`);
    }

    async function logout() {
        await fetch('/japprends/logout', { method: 'POST' });
        location.href = '/japprends/login';
    }
</script>

<svelte:head>
    <title>Dashboard admin — rev0auth</title>
</svelte:head>

<div class="dashboard">
    <header class="dash-header">
        <span class="dash-brand">rev0auth <span class="dash-role">admin</span></span>
        {#if data.songsurfUrl}
            <a class="btn-songsurf" href={data.songsurfUrl} target="_blank" rel="noopener">
                SongSurf ↗
            </a>
        {/if}
        <span class="dash-pseudo">{data.admin}</span>
        <button class="btn-logout" onclick={logout}>Déconnexion</button>
    </header>

    <nav class="tabs-nav">
        {#each TABS as tab}
            <button
                class="tab-btn"
                class:active={active === tab}
                onclick={() => setTab(tab)}
            >
                {TAB_LABELS[tab]}
            </button>
        {/each}
    </nav>

    <main class="dash-content">
        {#if active === 'members'}
            <MembersTab users={data.users} />
        {:else if active === 'wall'}
            <WallTab />
        {:else if active === 'messages'}
            <MessagesTab />
        {:else if active === 'donations'}
            <DonationsTab />
        {:else if active === 'invitations'}
            <InvitationsTab />
        {/if}
    </main>
</div>

<style>
    .dashboard {
        min-height: 100vh; display: flex; flex-direction: column;
        background: var(--background); color: var(--foreground);
    }

    .dash-header {
        display: flex; align-items: center; gap: 1rem; flex-wrap: wrap;
        padding: 0.875rem 1.5rem; border-bottom: 1px solid var(--border);
        background: var(--card);
    }
    .dash-brand {
        font-weight: 700; font-size: 1rem; letter-spacing: -0.01em;
    }
    .dash-role {
        font-size: 0.6875rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.06em; background: rgba(160,120,255,0.15);
        color: #a078ff; border-radius: 99px; padding: 2px 7px;
        vertical-align: middle;
    }
    .btn-songsurf {
        font: 500 0.8125rem/1 var(--font-sans);
        background: var(--muted); border: 1px solid var(--border);
        border-radius: var(--radius-sm); padding: 5px 12px;
        color: var(--foreground); text-decoration: none;
        transition: border-color 0.15s;
    }
    .btn-songsurf:hover { border-color: var(--primary); color: var(--primary); }

    .dash-pseudo {
        font-size: 0.875rem; color: var(--muted-foreground);
        margin-left: auto;
    }
    .btn-logout {
        background: var(--muted); border: 1px solid var(--border);
        border-radius: var(--radius-sm); padding: 5px 12px;
        font: 500 0.8125rem/1 var(--font-sans); cursor: pointer;
        color: var(--foreground);
    }
    .btn-logout:hover { border-color: var(--destructive); color: var(--destructive); }

    .tabs-nav {
        display: flex; gap: 0; overflow-x: auto;
        border-bottom: 1px solid var(--border); background: var(--card);
        padding: 0 1rem;
    }
    .tab-btn {
        background: none; border: none; border-bottom: 2px solid transparent;
        padding: 0.75rem 1rem; font: 500 0.875rem/1 var(--font-sans);
        color: var(--muted-foreground); cursor: pointer; white-space: nowrap;
        transition: color 0.15s, border-color 0.15s;
    }
    .tab-btn:hover { color: var(--foreground); }
    .tab-btn.active { color: var(--foreground); border-bottom-color: var(--primary); }

    .dash-content {
        flex: 1; padding: 1.5rem;
        max-width: 900px; width: 100%; margin: 0 auto;
    }

    @media (max-width: 600px) {
        .dash-header { padding: 0.75rem 1rem; }
        .dash-content { padding: 1rem; }
    }
</style>
