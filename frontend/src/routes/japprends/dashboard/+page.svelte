<script lang="ts">
    import { page }       from '$app/state';
    import MembersTab     from './MembersTab.svelte';
    import InvitationsTab from './InvitationsTab.svelte';
    import SecurityTab    from './SecurityTab.svelte';

    // Barre et onglets dans japprends/+layout.svelte : l'onglet ouvert suit
    // l'ancre de l'adresse (#members, #invitations, #security).
    let { data } = $props();
    const TABS = ['members', 'invitations', 'security'];
    let active = $derived(TABS.includes(page.url.hash.slice(1)) ? page.url.hash.slice(1) : 'members');
</script>

<svelte:head>
    <title>Dashboard admin — rev0auth</title>
</svelte:head>

<main class="dash-content">
    {#if active === 'members'}
        <MembersTab users={data.users} />
    {:else if active === 'invitations'}
        <InvitationsTab signupUrl={data.signupUrl} />
    {:else if active === 'security'}
        <SecurityTab />
    {/if}
</main>

<style>
    .dash-content {
        padding: 1.5rem;
        max-width: 900px; width: 100%; margin: 0 auto;
    }

    @media (max-width: 600px) {
        .dash-content { padding: 1rem; }
    }
</style>
