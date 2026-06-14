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

    // Regroupement en cascade Membre → Artiste → Album → titres : les libellés
    // répétés (pseudo/artiste/album) sont factorisés, chaque niveau est repliable.
    type Track    = { title: string; receivedAt: number; eventType: string; detail: string };
    type AlbumGrp = { album: string; tracks: Track[]; last: number };
    type ArtistGrp = { artist: string; albums: AlbumGrp[]; count: number; last: number };
    type MemberGrp = { pseudo: string; artists: ArtistGrp[]; count: number; last: number };

    function groupDownloads(list: typeof downloads): MemberGrp[] {
        const members = new Map<string, Map<string, Map<string, Track[]>>>();
        for (const e of list) {
            const p = e.pseudo || '—', ar = e.artist || '—', al = e.album || '—';
            if (!members.has(p)) members.set(p, new Map());
            const arts = members.get(p)!;
            if (!arts.has(ar)) arts.set(ar, new Map());
            const albs = arts.get(ar)!;
            if (!albs.has(al)) albs.set(al, []);
            albs.get(al)!.push({ title: e.title || '—', receivedAt: e.receivedAt, eventType: e.eventType, detail: e.detail });
        }
        const out: MemberGrp[] = [];
        for (const [pseudo, arts] of members) {
            const artists: ArtistGrp[] = [];
            let mCount = 0, mLast = 0;
            for (const [artist, albs] of arts) {
                const albums: AlbumGrp[] = [];
                let aCount = 0, aLast = 0;
                for (const [album, tracks] of albs) {
                    tracks.sort((x, y) => y.receivedAt - x.receivedAt);
                    const last = tracks[0]?.receivedAt ?? 0;
                    albums.push({ album, tracks, last });
                    aCount += tracks.length; aLast = Math.max(aLast, last);
                }
                albums.sort((x, y) => y.last - x.last);
                artists.push({ artist, albums, count: aCount, last: aLast });
                mCount += aCount; mLast = Math.max(mLast, aLast);
            }
            artists.sort((x, y) => y.last - x.last);
            out.push({ pseudo, artists, count: mCount, last: mLast });
        }
        return out.sort((x, y) => y.last - x.last);
    }

    let grouped = $derived(groupDownloads(downloads));

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR', { timeZone: 'Europe/Paris' });
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
        {#if grouped.length === 0}
            <p class="empty">Aucun événement.</p>
        {:else}
            <div class="tree">
                {#each grouped as m (m.pseudo)}
                    <details class="grp grp-member" open>
                        <summary>
                            <span class="chevron" aria-hidden="true">▸</span>
                            <span class="grp-name">{m.pseudo}</span>
                            <span class="count-pill">{m.count} téléchargement{m.count > 1 ? 's' : ''}</span>
                        </summary>
                        {#each m.artists as a (a.artist)}
                            <details class="grp grp-artist" open>
                                <summary>
                                    <span class="chevron" aria-hidden="true">▸</span>
                                    <span class="grp-name">{a.artist}</span>
                                    <span class="count-pill subtle">{a.count}</span>
                                </summary>
                                {#each a.albums as al (al.album)}
                                    <details class="grp grp-album">
                                        <summary>
                                            <span class="chevron" aria-hidden="true">▸</span>
                                            <span class="grp-name">{al.album}</span>
                                            <span class="count-pill subtle">{al.tracks.length} titre{al.tracks.length > 1 ? 's' : ''}</span>
                                            <span class="grp-last">{fmt(al.last)}</span>
                                        </summary>
                                        <ul class="track-list">
                                            {#each al.tracks as t, i (i)}
                                                <li>
                                                    {#if t.eventType !== 'download_success'}
                                                        <span class="badge" class:ko={t.eventType === 'download_failed'} class:zip={t.eventType === 'zip_export'}>
                                                            {TYPE_LABELS[t.eventType] ?? t.eventType}
                                                        </span>
                                                    {/if}
                                                    <span class="t-title">{t.title}</span>
                                                    {#if detailText(t)}<span class="detail">{detailText(t)}</span>{/if}
                                                    <span class="t-date ts">{fmt(t.receivedAt)}</span>
                                                </li>
                                            {/each}
                                        </ul>
                                    </details>
                                {/each}
                            </details>
                        {/each}
                    </details>
                {/each}
            </div>
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
    .activity-page {
        max-width: 1200px; margin: 0 auto; padding: 1.5rem 1rem 3rem;
        color: var(--foreground);
    }
    header {
        display: flex; align-items: center; justify-content: space-between;
        gap: 1rem; flex-wrap: wrap; margin-bottom: 1.5rem;
        padding-bottom: 1rem; border-bottom: 1px solid var(--border);
    }
    h1 { margin: 0; font-size: 1.4rem; font-weight: 700; }
    h2 { font-size: 1.05rem; font-weight: 600; margin: 1.75rem 0 .6rem; }

    .actions { display: flex; gap: .5rem; align-items: center; flex-wrap: wrap; }
    .actions select {
        padding: .45rem .6rem; min-width: 180px;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--background); color: var(--foreground);
        font: 500 0.8125rem/1 var(--font-sans); cursor: pointer;
    }
    .actions select:focus { outline: none; border-color: var(--primary); }
    .actions a {
        font-size: .8125rem; color: var(--primary-hover);
        text-decoration: none; font-weight: 500; padding: .3rem .2rem;
    }
    .actions a:hover { text-decoration: underline; }

    section {
        background: var(--card); border: 1px solid var(--border);
        border-radius: var(--radius-md); padding: .25rem 1rem 1rem;
        margin-top: 1rem; overflow-x: auto;
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
    .detail { color: var(--muted-foreground); font-size: .8125rem; }

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

    /* ── Cascade Membre → Artiste → Album → titres ── */
    .tree { display: flex; flex-direction: column; gap: .35rem; padding: .5rem 0; }
    .grp > summary {
        display: flex; align-items: center; gap: .5rem;
        padding: .45rem .55rem; cursor: pointer; list-style: none;
        border-radius: var(--radius-sm); user-select: none;
    }
    .grp > summary::-webkit-details-marker { display: none; }
    .grp > summary:hover { background: var(--muted); }
    .chevron {
        font-size: .7rem; color: var(--muted-foreground); flex-shrink: 0;
        transition: transform .15s;
    }
    .grp[open] > summary > .chevron { transform: rotate(90deg); }
    .grp-name { font-weight: 600; }
    .grp-member > summary > .grp-name { font-size: .95rem; }
    .grp-artist > summary > .grp-name { font-weight: 500; }
    .grp-album  > summary > .grp-name { font-weight: 400; color: var(--muted-foreground); }

    .count-pill {
        font-size: .7rem; font-weight: 600; padding: .1rem .5rem; border-radius: 999px;
        background: var(--muted); color: var(--muted-foreground); border: 1px solid var(--border);
    }
    .count-pill.subtle { background: none; border: none; padding: 0; }
    .grp-last { margin-left: auto; font-size: .75rem; color: var(--muted-foreground); font-variant-numeric: tabular-nums; }

    /* indentation par niveau */
    .grp-member > :not(summary) { margin-left: 1rem; border-left: 1px solid var(--border); padding-left: .4rem; }
    .grp-artist > :not(summary) { margin-left: 1rem; border-left: 1px solid var(--border); padding-left: .4rem; }

    .track-list { list-style: none; margin: 0; padding: .2rem 0 .4rem 1.5rem; display: flex; flex-direction: column; gap: 1px; }
    .track-list li {
        display: flex; align-items: baseline; gap: .6rem;
        padding: .3rem .5rem; border-radius: var(--radius-sm); font-size: .85rem;
    }
    .track-list li:hover { background: var(--muted); }
    .t-title { font-weight: 500; }
    .t-date { margin-left: auto; }

    .more { text-align: center; margin: 1.5rem 0; }
    .more button {
        padding: .5rem 1rem; cursor: pointer;
        background: var(--muted); color: var(--foreground);
        border: 1px solid var(--border); border-radius: var(--radius-md);
        font: 500 0.8125rem/1 var(--font-sans);
    }
    .more button:hover { border-color: var(--primary); }
</style>
