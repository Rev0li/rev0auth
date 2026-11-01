<script lang="ts">
    import { page } from '$app/state';
    import { replaceState, invalidateAll } from '$app/navigation';
    import { untrack } from 'svelte';
    import { NO_ALBUM } from '$lib/songsurf-taste.js';
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();

    // Membre choisi ('' = tout le monde), gardé dans l'adresse (?m=pseudo)
    let selected = $state(untrack(() => page.url.searchParams.get('m') ?? ''));
    let profile = $derived(
        data.members.find((m) => m.pseudo === selected) ?? data.all,
    );

    function choose(pseudo: string) {
        selected = pseudo;
        showAll = false; filter = '';
        const url = new URL(page.url);
        if (pseudo) url.searchParams.set('m', pseudo); else url.searchParams.delete('m');
        replaceState(url, {});
    }

    // Artistes : filtre par nom, 20 premiers puis « voir tout »
    const FIRST = 20;
    let filter = $state('');
    let showAll = $state(false);
    let matching = $derived(
        filter.trim()
            ? profile.artists.filter((a) => a.artist.toLowerCase().includes(filter.trim().toLowerCase()))
            : profile.artists,
    );
    let shown = $derived(showAll || filter.trim() ? matching : matching.slice(0, FIRST));
    let maxCount = $derived(profile.artists[0]?.count ?? 1);

    const thisYear = new Date().getFullYear();
    function day(epoch: number) {
        const d = new Date(epoch * 1000);
        return d.toLocaleDateString('fr-FR', {
            day: 'numeric', month: 'short',
            ...(d.getFullYear() !== thisYear ? { year: 'numeric' } : {}),
            timeZone: 'Europe/Paris',
        });
    }
    const plural = (n: number, one: string, many = one + 's') => `${n} ${n > 1 ? many : one}`;

    // Reprise de l'ancien historique SongSurf (avant l'envoi automatique)
    let importing = $state(false);
    let importMsg = $state('');
    let importOk  = $state(false);
    async function importHistory() {
        importing = true; importMsg = '';
        try {
            const r = await fetch('/japprends/songsurf-activity/import', { method: 'POST' });
            const d = await r.json().catch(() => ({ ok: false, message: `Erreur ${r.status}` }));
            importOk = d.ok;
            importMsg = d.ok
                ? (d.imported ? `${plural(d.imported, 'titre importé', 'titres importés')}.` : 'Rien de nouveau : tout l\'historique est déjà là.')
                : d.message;
            if (d.ok && d.imported) await invalidateAll();
        } catch {
            importOk = false; importMsg = 'Le portail ne répond pas.';
        } finally { importing = false; }
    }
</script>

<svelte:head><title>Activité SongSurf — rev0auth</title></svelte:head>

<main class="taste">
    {#if data.all.titles === 0}
        <p class="empty">Aucun téléchargement pour le moment.</p>
    {:else}
        <!-- Choix du membre -->
        <div class="chips" role="tablist" aria-label="Membre">
            <button class="chip" class:on={selected === ''} role="tab" aria-selected={selected === ''} onclick={() => choose('')}>
                Tout le monde <span class="chip-n">{data.all.titles}</span>
            </button>
            {#each data.members as m (m.pseudo)}
                <button class="chip" class:on={selected === m.pseudo} role="tab" aria-selected={selected === m.pseudo} onclick={() => choose(m.pseudo ?? '')}>
                    <img class="chip-av" src="/members/avatar/{m.pseudo}" alt=""
                        onerror={(e) => (e.currentTarget as HTMLImageElement).style.display = 'none'} />
                    {m.pseudo} <span class="chip-n">{m.titles}</span>
                </button>
            {/each}
        </div>

        <!-- En-tête de la fiche -->
        <header class="head">
            <h1>{profile.pseudo ?? 'Tout le monde'}</h1>
            <p class="stats">
                {plural(profile.titles, 'titre')} · {plural(profile.artistCount, 'artiste')} ·
                {plural(profile.albumCount, 'album')} · dernier le {day(profile.last)}
            </p>
        </header>

        <!-- Derniers albums -->
        {#if profile.recentAlbums.length}
            <section>
                <h2>Derniers albums</h2>
                <div class="albums">
                    {#each profile.recentAlbums as al (al.artist + '/' + al.album)}
                        <article class="album">
                            <div class="album-art" aria-hidden="true">{al.album.slice(0, 1).toUpperCase()}</div>
                            <div class="album-text">
                                <span class="album-name" title={al.album}>{al.album}</span>
                                <span class="album-artist" title={al.artist}>{al.artist}</span>
                                <span class="album-meta">
                                    {plural(al.count, 'titre')} · {day(al.last)}
                                    {#if !profile.pseudo} · {al.who.join(', ')}{/if}
                                </span>
                            </div>
                        </article>
                    {/each}
                </div>
            </section>
        {/if}

        <!-- Artistes classés -->
        <section>
            <div class="artists-head">
                <h2>{profile.pseudo ? 'Ses artistes' : 'Artistes'}</h2>
                <input class="filter" type="search" placeholder="Chercher un artiste…" bind:value={filter} />
            </div>

            {#if shown.length === 0}
                <p class="empty">Aucun artiste ne correspond.</p>
            {:else}
                <ol class="artists">
                    {#each shown as a (a.artist)}
                        <li>
                            <details>
                                <summary>
                                    <span class="chevron" aria-hidden="true">▸</span>
                                    <span class="artist-name">{a.artist}</span>
                                    <span class="bar" aria-hidden="true"><span style="width:{Math.max(4, (a.count / maxCount) * 100)}%"></span></span>
                                    <span class="artist-n">{a.count}</span>
                                </summary>
                                <ul class="artist-albums">
                                    {#each a.albums as al (al.album)}
                                        <li>
                                            <span class="al-name" class:al-single={al.album === NO_ALBUM}>{al.album}</span>
                                            <span class="al-meta">
                                                {plural(al.count, 'titre')} · {day(al.last)}
                                                {#if !profile.pseudo} · {al.who.join(', ')}{/if}
                                            </span>
                                        </li>
                                    {/each}
                                </ul>
                            </details>
                        </li>
                    {/each}
                </ol>
                {#if !showAll && !filter.trim() && matching.length > FIRST}
                    <button class="more" onclick={() => (showAll = true)}>Voir les {matching.length} artistes</button>
                {/if}
            {/if}
        </section>
    {/if}

    <!-- Historique stocké sur le portail -->
    <footer class="store">
        <p>
            {#if data.status.total}
                Historique stocké sur le portail : {plural(data.status.total, 'titre')}
                depuis le {day(data.status.since ?? 0)}{#if data.status.imported}, dont {data.status.imported} repris de l'ancien journal{/if}.
                Toujours lisible, même NAS éteint.
            {:else}
                Aucun téléchargement stocké sur le portail pour le moment.
            {/if}
        </p>
        {#if data.canImport}
            <button class="import" onclick={importHistory} disabled={importing}>
                {importing ? 'Import en cours…' : 'Reprendre l\'ancien historique de SongSurf'}
            </button>
            {#if importMsg}<p class="import-msg" class:ok={importOk}>{importMsg}</p>{/if}
        {/if}
    </footer>
</main>

<style>
    .taste { max-width: 1000px; margin: 0 auto; padding: 1.5rem 1rem 4rem; }

    .empty { color: var(--muted-foreground); padding: 2rem; text-align: center; }

    /* ── Membres ── */
    .chips { display: flex; gap: 8px; flex-wrap: wrap; margin-bottom: 1.5rem; }
    .chip {
        display: inline-flex; align-items: center; gap: 6px;
        height: 34px; padding: 0 12px 0 10px;
        border: 1px solid var(--border); border-radius: 999px;
        background: var(--card); color: var(--foreground);
        font: 600 .8125rem/1 var(--font-sans); cursor: pointer;
        transition: border-color .15s, transform .15s;
    }
    .chip:hover { border-color: var(--primary); transform: translateY(-1px); }
    .chip.on { border-color: var(--primary); box-shadow: 0 0 0 1px var(--primary); background: var(--muted); }
    .chip-av { width: 22px; height: 22px; border-radius: 50%; object-fit: cover; }
    .chip-n { font-weight: 500; color: var(--muted-foreground); font-variant-numeric: tabular-nums; }

    /* ── En-tête ── */
    .head { margin-bottom: 1.25rem; }
    h1 { margin: 0; font-size: 1.6rem; font-weight: 700; letter-spacing: -.01em; }
    .stats { margin: .25rem 0 0; color: var(--muted-foreground); font-size: .875rem; }

    section {
        background: var(--card); border: 1px solid var(--border);
        border-radius: var(--radius-lg); padding: 1rem 1.25rem 1.25rem;
        margin-bottom: 1rem;
    }
    h2 {
        margin: 0 0 .875rem; font-size: .8125rem; font-weight: 700;
        text-transform: uppercase; letter-spacing: .06em; color: var(--muted-foreground);
    }

    /* ── Derniers albums ── */
    .albums { display: grid; grid-template-columns: repeat(auto-fill, minmax(230px, 1fr)); gap: .75rem; }
    .album {
        display: flex; gap: .75rem; align-items: center; min-width: 0;
        padding: .625rem; border: 1px solid var(--border); border-radius: var(--radius-md);
    }
    .album-art {
        width: 48px; height: 48px; flex-shrink: 0; border-radius: var(--radius-sm);
        display: grid; place-items: center;
        font-weight: 700; font-size: 1.25rem; color: var(--card);
        background: linear-gradient(135deg, #E8B7C4, #6EDAD3);
    }
    .album-text { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
    .album-name, .album-artist { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .album-name { font-weight: 600; font-size: .9rem; }
    .album-artist { font-size: .8125rem; }
    .album-meta { font-size: .75rem; color: var(--muted-foreground); }

    /* ── Artistes ── */
    .artists-head { display: flex; align-items: center; justify-content: space-between; gap: 1rem; margin-bottom: .5rem; }
    .artists-head h2 { margin: 0; }
    .filter { max-width: 240px; height: 32px; font-size: .8125rem; }

    .artists { list-style: none; margin: 0; padding: 0; }
    .artists > li + li { border-top: 1px solid var(--border); }
    summary {
        display: grid; grid-template-columns: 14px minmax(0, 1fr) minmax(80px, 2fr) 3ch;
        align-items: center; gap: .75rem;
        padding: .55rem .4rem; cursor: pointer; list-style: none;
        border-radius: var(--radius-sm);
    }
    summary::-webkit-details-marker { display: none; }
    summary:hover { background: var(--muted); }
    .chevron { font-size: .7rem; color: var(--muted-foreground); transition: transform .15s; }
    details[open] .chevron { transform: rotate(90deg); }
    .artist-name { font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .bar { height: 8px; border-radius: 99px; background: var(--muted); overflow: hidden; }
    .bar > span { display: block; height: 100%; border-radius: inherit; background: var(--primary); }
    .artist-n { text-align: right; font-variant-numeric: tabular-nums; color: var(--muted-foreground); font-size: .8125rem; }

    .artist-albums { list-style: none; margin: 0 0 .5rem; padding: 0 0 0 1.9rem; }
    .artist-albums li {
        display: flex; align-items: baseline; justify-content: space-between; gap: 1rem;
        padding: .3rem .4rem; font-size: .85rem;
    }
    .al-single { font-style: italic; color: var(--muted-foreground); }
    .al-meta { color: var(--muted-foreground); font-size: .75rem; white-space: nowrap; }

    .more {
        display: block; margin: .75rem auto 0; height: 32px; padding: 0 14px;
        border: 1px solid var(--border); border-radius: 999px;
        background: transparent; color: var(--foreground);
        font: 500 .8125rem/1 var(--font-sans); cursor: pointer;
    }
    .more:hover { border-color: var(--primary); }

    /* ── Historique stocké ── */
    .store {
        margin-top: 2rem; padding-top: 1rem; border-top: 1px dashed var(--border);
        display: flex; flex-direction: column; align-items: flex-start; gap: .5rem;
        font-size: .8125rem; color: var(--muted-foreground);
    }
    .store p { margin: 0; }
    .import {
        height: 30px; padding: 0 12px;
        border: 1px solid var(--border); border-radius: 999px;
        background: transparent; color: var(--foreground);
        font: 500 .8125rem/1 var(--font-sans); cursor: pointer;
    }
    .import:hover:not(:disabled) { border-color: var(--primary); }
    .import:disabled { opacity: .6; cursor: default; }
    .import-msg { color: var(--destructive); }
    .import-msg.ok { color: #3a9e6a; }

    @media (max-width: 560px) {
        summary { grid-template-columns: 14px minmax(0, 1fr) 3ch; }
        .bar { display: none; }
        .artists-head { flex-wrap: wrap; }
        .filter { max-width: none; }
    }
</style>
