<script lang="ts">
    import { goto } from '$app/navigation';
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();

    const TYPE_LABELS: Record<string, string> = {
        login_success:    'Connexion',
        login_rejected:   'Connexion refusée',
        download_success: 'Téléchargement',
        download_failed:  'Échec téléchargement',
        zip_export:       'Export ZIP',
        container_start:  'Container démarré',
        container_stop:   'Container arrêté',
    };

    const DOWNLOAD_TYPES = new Set(['download_success', 'download_failed', 'zip_export']);

    let downloads = $derived(data.events.filter(e => DOWNLOAD_TYPES.has(e.eventType)));
    let others    = $derived(data.events.filter(e => !DOWNLOAD_TYPES.has(e.eventType)));

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR');
    }

    function parseDetail(raw: string): Record<string, unknown> {
        try {
            const d = JSON.parse(raw);
            return typeof d === 'object' && d !== null ? d : {};
        } catch {
            return {};
        }
    }

    function detailText(e: { eventType: string; detail: string }): string {
        const d = parseDetail(e.detail);
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

    function applyFilters(pseudo: string, type: string, limit: number) {
        const params = new URLSearchParams();
        if (pseudo) params.set('pseudo', pseudo);
        if (type) params.set('type', type);
        if (limit !== 100) params.set('limit', String(limit));
        goto(`/japprends/songsurf-activity${params.size ? '?' + params : ''}`);
    }
</script>

<svelte:head><title>Activité SongSurf — rev0auth</title></svelte:head>

<main class="activity-page">
    <header>
        <h1>Activité SongSurf</h1>
        <div class="actions">
            <select
                value={data.filterPseudo}
                onchange={(ev) => applyFilters(ev.currentTarget.value, data.filterType, data.limit)}
            >
                <option value="">Tous les membres</option>
                {#each data.pseudos as p}
                    <option value={p}>{p}</option>
                {/each}
            </select>
            <select
                value={data.filterType}
                onchange={(ev) => applyFilters(data.filterPseudo, ev.currentTarget.value, data.limit)}
            >
                <option value="">Tous les événements</option>
                {#each data.eventTypes as t}
                    <option value={t}>{TYPE_LABELS[t] ?? t}</option>
                {/each}
            </select>
            <a href="/japprends/songsurf-logs">Logs bruts (pull)</a>
            <a href="/japprends/dashboard">← Dashboard</a>
        </div>
    </header>

    {#if !data.filterPseudo && !data.filterType}
        <section>
            <h2>Connexions récentes</h2>
            {#if data.logins.length === 0}
                <p class="empty">Aucune connexion enregistrée.</p>
            {:else}
                <table>
                    <thead>
                        <tr><th>Date</th><th>Membre</th><th>Rôle</th><th>IP</th><th>Statut</th></tr>
                    </thead>
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
    {/if}

    <section>
        <h2>Téléchargements & exports</h2>
        {#if downloads.length === 0}
            <p class="empty">Aucun événement.</p>
        {:else}
            <table>
                <thead>
                    <tr><th>Date</th><th>Membre</th><th>Type</th><th>Artiste</th><th>Album</th><th>Titre</th><th>Détail</th></tr>
                </thead>
                <tbody>
                    {#each downloads as e (e.id)}
                        <tr>
                            <td class="ts">{fmt(e.receivedAt)}</td>
                            <td>{e.pseudo || '—'}</td>
                            <td>
                                <span class="badge" class:ok={e.eventType === 'download_success'}
                                      class:ko={e.eventType === 'download_failed'}
                                      class:zip={e.eventType === 'zip_export'}>
                                    {TYPE_LABELS[e.eventType] ?? e.eventType}
                                </span>
                            </td>
                            <td>{e.artist || '—'}</td>
                            <td>{e.album || '—'}</td>
                            <td>{e.title || '—'}</td>
                            <td class="detail">{detailText(e) || '—'}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </section>

    {#if others.length > 0 && (data.filterType === '' || !DOWNLOAD_TYPES.has(data.filterType))}
        <section>
            <h2>Autres événements</h2>
            <table>
                <thead>
                    <tr><th>Date</th><th>Source</th><th>Type</th><th>Membre</th><th>Détail</th></tr>
                </thead>
                <tbody>
                    {#each others as e (e.id)}
                        <tr>
                            <td class="ts">{fmt(e.receivedAt)}</td>
                            <td>{e.source || '—'}</td>
                            <td><span class="badge">{TYPE_LABELS[e.eventType] ?? e.eventType}</span></td>
                            <td>{e.pseudo || '—'}</td>
                            <td class="detail">{detailText(e) || '—'}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </section>
    {/if}

    {#if data.events.length >= data.limit}
        <p class="more">
            <button onclick={() => applyFilters(data.filterPseudo, data.filterType, Math.min(data.limit * 2, 500))}>
                Afficher plus ({data.limit} → {Math.min(data.limit * 2, 500)})
            </button>
        </p>
    {/if}
</main>

<style>
    .activity-page { max-width: 1200px; margin: 2rem auto; padding: 0 1rem; }
    header { display: flex; align-items: center; justify-content: space-between; gap: 1rem; flex-wrap: wrap; margin-bottom: 1.5rem; }
    h1 { margin: 0; font-size: 1.4rem; }
    h2 { font-size: 1.05rem; margin: 1.5rem 0 .6rem; }
    .actions { display: flex; gap: .5rem; align-items: center; flex-wrap: wrap; }
    .actions select { padding: .4rem .6rem; border: 1px solid #ccc; border-radius: 4px; }
    .actions a { font-size: .9rem; }
    table { width: 100%; border-collapse: collapse; font-size: .9rem; }
    th, td { text-align: left; padding: .5rem .6rem; border-bottom: 1px solid #eee; }
    th { background: #f7f7f7; font-weight: 600; }
    .ts { white-space: nowrap; color: #666; font-variant-numeric: tabular-nums; }
    .ip { font-family: monospace; color: #666; }
    .detail { color: #666; font-size: .85rem; }
    .badge { display: inline-block; padding: .1rem .5rem; border-radius: 999px; font-size: .78rem; background: #eef; color: #336; }
    .badge.ok { background: #e6f6e6; color: #1a7f1a; }
    .badge.ko { background: #fdeaea; color: #b02a2a; }
    .badge.zip { background: #fff3e0; color: #a05a00; }
    .empty { color: #888; padding: 1.5rem; text-align: center; }
    .more { text-align: center; margin: 1.5rem 0; }
    .more button { padding: .4rem .9rem; cursor: pointer; }
</style>
