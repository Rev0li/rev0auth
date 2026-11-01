<script lang="ts">
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();

    const TYPE_LABELS: Record<string, string> = {
        download_failed:  'Échec téléchargement',
        zip_export:       'Export ZIP',
        container_start:  'Conteneur démarré',
        container_stop:   'Conteneur arrêté',
    };

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR', { timeZone: 'Europe/Paris' });
    }

    function detailText(e: { eventType: string; detail: string }): string {
        let d: Record<string, unknown> = {};
        try { const p = JSON.parse(e.detail); if (p && typeof p === 'object') d = p; } catch { /* détail illisible */ }
        if (e.eventType === 'download_failed' && d.error) return String(d.error);
        if (e.eventType === 'zip_export') {
            const parts = [];
            if (d.count !== undefined) parts.push(`${d.count} fichier(s)`);
            if (d.size_mb !== undefined) parts.push(`${d.size_mb} MB`);
            if (d.library_purged) parts.push('bibliothèque purgée');
            return parts.join(' · ');
        }
        if (e.eventType === 'container_stop' && d.idle_seconds !== undefined) return `inactif ${d.idle_seconds}s`;
        if (e.eventType === 'login_rejected' && d.reason) return String(d.reason);
        return '';
    }
</script>

<svelte:head><title>Journal SongSurf — rev0auth</title></svelte:head>

<main class="journal">
    <section>
        <h2>Connexions récentes</h2>
        {#if data.logins.length === 0}
            <p class="empty">Aucune connexion enregistrée.</p>
        {:else}
            <table>
                <thead><tr><th>Date</th><th>Membre</th><th>Rôle</th><th>IP</th><th>Statut</th></tr></thead>
                <tbody>
                    {#each data.logins as e (e.id)}
                        <tr>
                            <td class="ts">{fmt(e.receivedAt)}</td>
                            <td>{e.pseudo || '—'}</td>
                            <td>{e.role || '—'}</td>
                            <td class="ip">{e.ip || '—'}</td>
                            <td>
                                {#if e.eventType === 'login_success'}
                                    <span class="badge ok">OK</span>
                                {:else}
                                    <span class="badge ko">Refusée</span>
                                {/if}
                                {#if detailText(e)}<span class="detail">{detailText(e)}</span>{/if}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </section>

    <section>
        <h2>Évènements</h2>
        {#if data.others.length === 0}
            <p class="empty">Aucun évènement.</p>
        {:else}
            <table>
                <thead><tr><th>Date</th><th>Type</th><th>Membre</th><th>Détail</th></tr></thead>
                <tbody>
                    {#each data.others as e (e.id)}
                        <tr>
                            <td class="ts">{fmt(e.receivedAt)}</td>
                            <td>
                                <span class="badge" class:ko={e.eventType === 'download_failed'} class:zip={e.eventType === 'zip_export'}>
                                    {TYPE_LABELS[e.eventType] ?? e.eventType}
                                </span>
                            </td>
                            <td>{e.pseudo || '—'}</td>
                            <td class="detail">{detailText(e) || '—'}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </section>
</main>

<style>
    .journal { max-width: 1100px; margin: 0 auto; padding: 1.5rem 1rem 3rem; }
    h2 { font-size: 1.05rem; font-weight: 600; margin: 1rem 0 .6rem; }
    section {
        background: var(--card); border: 1px solid var(--border);
        border-radius: var(--radius-md); padding: .25rem 1rem 1rem;
        margin-bottom: 1rem; overflow-x: auto;
    }
    table { width: 100%; border-collapse: collapse; font-size: .85rem; }
    th, td { text-align: left; padding: .55rem .6rem; border-bottom: 1px solid var(--border); }
    th {
        font-weight: 600; color: var(--muted-foreground);
        text-transform: uppercase; font-size: .6875rem; letter-spacing: .04em;
    }
    tbody tr:last-child td { border-bottom: none; }
    tbody tr:hover { background: var(--muted); }
    .ts { white-space: nowrap; color: var(--muted-foreground); font-variant-numeric: tabular-nums; }
    .ip { font-family: var(--font-mono, monospace); color: var(--muted-foreground); }
    .detail { color: var(--muted-foreground); font-size: .8125rem; margin-left: .4rem; }
    .badge {
        display: inline-block; padding: .12rem .55rem; border-radius: 999px;
        font-size: .72rem; font-weight: 600;
        background: var(--muted); color: var(--muted-foreground);
        border: 1px solid var(--border);
    }
    .badge.ok { background: var(--success-bg); border-color: var(--success-border); color: #3a9e6a; }
    .badge.ko { background: var(--destructive-bg); border-color: var(--destructive-border); color: var(--destructive); }
    .badge.zip { background: rgba(217,150,70,.14); border-color: rgba(217,150,70,.35); color: #c8862f; }
    .empty { color: var(--muted-foreground); padding: 1.75rem; text-align: center; }
</style>
