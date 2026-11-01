<script lang="ts">
    import { page }    from '$app/state';
    import MembersTab  from './MembersTab.svelte';
    import SecurityTab from './SecurityTab.svelte';

    // Barre et onglets dans japprends/+layout.svelte : l'onglet ouvert suit
    // l'ancre de l'adresse (#members, #security). Les invitations vivent dans
    // Membres (AUTH-12) : l'ancienne ancre #invitations y mène aussi.
    let { data } = $props();
    let active = $derived(page.url.hash === '#security' ? 'security' : 'members');
</script>

<svelte:head>
    <title>Dashboard admin — rev0auth</title>
</svelte:head>

<main class="dash-content">
    {#if active === 'security'}
        <SecurityTab />
    {:else}
        <MembersTab
            users={data.users}
            invites={data.invites}
            me={data.shell?.pseudo ?? ''}
            signupUrl={data.signupUrl}
            avatarBase={data.avatarBase}
        />
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
