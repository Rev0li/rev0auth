<script lang="ts">
    import { onMount } from 'svelte';
    import { invalidateAll } from '$app/navigation';
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();

    // Présence rafraîchie toutes les 30 s tant que la page est visible
    onMount(() => {
        const timer = setInterval(() => {
            if (document.visibilityState === 'visible') invalidateAll();
        }, 30_000);
        return () => clearInterval(timer);
    });

    let online  = $derived(data.presence.filter((p) => p.online));
    let earlier = $derived(data.presence.filter((p) => !p.online));

    const PLACE_LABELS: Record<string, string> = {
        portail: 'Portail', console: 'Console', songsurf: 'SongSurf', jellyfin: 'Jellyfin',
    };

    function ago(epoch: number) {
        const s = Math.max(0, data.now - epoch);
        if (s < 60) return 'à l\'instant';
        if (s < 3600) return `il y a ${Math.floor(s / 60)} min`;
        return `il y a ${Math.floor(s / 3600)} h`;
    }

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
        <h2>En ligne maintenant <span class="count">{online.length}</span></h2>
        {#if online.length === 0}
            <p class="empty">Personne en ce moment.</p>
        {:else}
            <ul class="people">
                {#each online as p (p.pseudo)}
                    <li>
                        <span class="dot" aria-hidden="true"></span>
                        <img class="av" src="/members/avatar/{p.pseudo}" alt=""
                            onerror={(e) => (e.currentTarget as HTMLImageElement).style.visibility = 'hidden'} />
                        <span class="who">{p.pseudo}{#if p.role === 'admin'} <span class="role">admin</span>{/if}</span>
                        <span class="places">
                            {#each p.places as pl (pl.place)}
                                <span class="place" class:live={pl.online} title={ago(pl.last)}>
                                    {PLACE_LABELS[pl.place] ?? pl.place}
                                </span>
                            {/each}
                        </span>
                        <span class="when">{ago(p.last)}</span>
                    </li>
                {/each}
            </ul>
        {/if}
        {#if earlier.length}
            <details class="earlier">
                <summary>Vus plus tôt ({earlier.length})</summary>
                <ul class="people">
                    {#each earlier as p (p.pseudo)}
                        <li>
                            <span class="dot off" aria-hidden="true"></span>
                            <span class="who">{p.pseudo}</span>
                            <span class="places">
                                {#each p.places as pl (pl.place)}<span class="place">{PLACE_LABELS[pl.place] ?? pl.place}</span>{/each}
                            </span>
                            <span class="when">{ago(p.last)}</span>
                        </li>
                    {/each}
                </ul>
            </details>
        {/if}
        <p class="hint">Activité des 5 dernières minutes sur le portail et les services. Remis à zéro à chaque déploiement du portail.</p>
    </section>

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

    /* ── En ligne ── */
    .count {
        margin-left: .35rem; padding: .05rem .5rem; border-radius: 999px;
        background: var(--success-bg); border: 1px solid var(--success-border);
        color: #3a9e6a; font-size: .75rem; vertical-align: middle;
    }
    .people { list-style: none; margin: 0; padding: 0; }
    .people li {
        display: flex; align-items: center; gap: .6rem;
        padding: .5rem .25rem; border-bottom: 1px solid var(--border);
    }
    .people li:last-child { border-bottom: none; }
    .dot {
        width: 9px; height: 9px; border-radius: 50%; flex-shrink: 0;
        background: #3a9e6a; box-shadow: 0 0 0 3px var(--success-bg);
        animation: pulse 2s ease-in-out infinite;
    }
    .dot.off { background: var(--border); box-shadow: none; animation: none; }
    @keyframes pulse { 50% { box-shadow: 0 0 0 6px transparent; } }
    @media (prefers-reduced-motion: reduce) { .dot { animation: none; } }
    .av { width: 24px; height: 24px; border-radius: 50%; object-fit: cover; }
    .who { font-weight: 600; }
    .role {
        font-size: .625rem; font-weight: 600; text-transform: uppercase; letter-spacing: .06em;
        background: rgba(160,120,255,0.15); color: #a078ff; border-radius: 99px; padding: 1px 6px;
    }
    .places { display: flex; gap: 4px; flex-wrap: wrap; }
    .place {
        font-size: .72rem; padding: .1rem .5rem; border-radius: 999px;
        border: 1px solid var(--border); color: var(--muted-foreground);
    }
    .place.live { border-color: var(--success-border); color: #3a9e6a; background: var(--success-bg); }
    .when { margin-left: auto; font-size: .75rem; color: var(--muted-foreground); white-space: nowrap; }
    .earlier { margin-top: .5rem; }
    .earlier summary { cursor: pointer; font-size: .8125rem; color: var(--muted-foreground); padding: .25rem; }
    .hint { margin: .75rem 0 0; font-size: .75rem; color: var(--muted-foreground); }
</style>
