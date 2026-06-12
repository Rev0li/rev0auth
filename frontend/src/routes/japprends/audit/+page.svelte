<script lang="ts">
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();
    let entries = $state([...data.entries]);
    let filter = $state('');

    let filtered = $derived(
        filter
            ? entries.filter(e =>
                e.action.includes(filter) ||
                e.actorPseudo.includes(filter) ||
                e.target.includes(filter))
            : entries
    );

    async function refresh() {
        const r = await fetch('/japprends/audit/data');
        if (r.ok) entries = await r.json();
    }

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR');
    }
</script>

<svelte:head><title>Audit log — rev0auth</title></svelte:head>

<main class="audit-page">
    <header>
        <h1>Audit log admin</h1>
        <div class="actions">
            <input type="text" placeholder="Filtrer (action / acteur / cible)" bind:value={filter} />
            <button onclick={refresh}>Rafraîchir</button>
            <a href="/japprends/dashboard">← Dashboard</a>
        </div>
    </header>

    {#if filtered.length === 0}
        <p class="empty">Aucune entrée.</p>
    {:else}
        <table>
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Action</th>
                    <th>Acteur</th>
                    <th>Cible</th>
                    <th>Détail</th>
                </tr>
            </thead>
            <tbody>
                {#each filtered as e (e.id)}
                    <tr>
                        <td class="ts">{fmt(e.createdAt)}</td>
                        <td class="action">{e.action}</td>
                        <td>{e.actorPseudo}</td>
                        <td>{e.target || '—'}</td>
                        <td class="detail">{e.detail || '—'}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</main>

<style>
    .audit-page { max-width: 1100px; margin: 2rem auto; padding: 0 1rem; }
    header { display: flex; align-items: center; justify-content: space-between; gap: 1rem; flex-wrap: wrap; margin-bottom: 1.5rem; }
    h1 { margin: 0; font-size: 1.4rem; }
    .actions { display: flex; gap: .5rem; align-items: center; }
    .actions input { padding: .4rem .6rem; border: 1px solid #ccc; border-radius: 4px; min-width: 220px; }
    .actions button { padding: .4rem .8rem; cursor: pointer; }
    .actions a { font-size: .9rem; }
    table { width: 100%; border-collapse: collapse; font-size: .9rem; }
    th, td { text-align: left; padding: .5rem .6rem; border-bottom: 1px solid #eee; }
    th { background: #f7f7f7; font-weight: 600; }
    .ts { white-space: nowrap; color: #666; font-variant-numeric: tabular-nums; }
    .action { font-family: monospace; color: #0066cc; }
    .detail { color: #666; }
    .empty { color: #888; padding: 2rem; text-align: center; }
</style>
