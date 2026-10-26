<script lang="ts">
    import type { PageData } from './$types.js';
    import { onMount, untrack } from 'svelte';
    import { goto, invalidateAll } from '$app/navigation';
    import { slide } from 'svelte/transition';
    import ThemeToggle from '$lib/ThemeToggle.svelte';
    import {
        BG_COLORS, FACE_COUNT,
        faceSeeds, decodeAvatarFilename,
    } from '$lib/avatar-options.js';

    let { data }: { data: PageData } = $props();

    // ── Mon compte : tuiles façon menu de jeu, une seule ouverte à la fois ──
    type Tile = 'password' | 'avatar' | 'delete';
    const TILES: { id: Tile; icon: string; label: string; hint: string }[] = [
        { id: 'password', icon: '🔑', label: 'Mot de passe', hint: 'Changer ton mot de passe' },
        { id: 'avatar',   icon: '🎨', label: 'Avatar',       hint: 'Visage et couleur de fond' },
        { id: 'delete',   icon: '🗑️', label: 'Supprimer',    hint: 'Effacer ton compte' },
    ];
    let open = $state<Tile | null>(null);

    function toggle(id: Tile) {
        open = open === id ? null : id;
        confirmDelete = false;
    }

    // Bust du cache de l'avatar de l'en-tête après une sauvegarde
    let avatarVersion = $state(0);

    // Console admin joignable seulement par le réseau privé (AUTH-11) : on le
    // vérifie avant d'y envoyer l'admin (Caddy coupe la connexion hors tailnet).
    let consoleReachable = $state<boolean | null>(null);
    onMount(() => {
        if (data.user.role !== 'admin') return;
        const probe = new URL('/_app/version.json', new URL(data.consoleUrl, location.href));
        fetch(probe, { mode: 'no-cors', cache: 'no-store', signal: AbortSignal.timeout(4000) })
            .then(() => { consoleReachable = true; })
            .catch(() => { consoleReachable = false; });
    });

    async function logout() {
        await fetch('/auth/logout', { method: 'POST' });
        goto('/');
    }

    // ── Mot de passe ──────────────────────────────────────────────────
    let currentPwd = $state('');
    let newPwd     = $state('');
    let confirmPwd = $state('');
    let pwdLoading = $state(false);
    let pwdMsg     = $state('');
    let pwdOk      = $state(false);

    async function changePassword() {
        pwdMsg = '';
        if (newPwd !== confirmPwd) { pwdMsg = 'Les mots de passe ne correspondent pas.'; return; }
        if (newPwd.length < 8) { pwdMsg = 'Minimum 8 caractères.'; return; }
        pwdLoading = true;
        try {
            const r = await fetch('/members/password', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ currentPassword: currentPwd, newPassword: newPwd }),
            });
            const d = await r.json();
            pwdOk = d.ok;
            pwdMsg = d.message;
            if (d.ok) { currentPwd = ''; newPwd = ''; confirmPwd = ''; }
        } finally { pwdLoading = false; }
    }

    // ── Avatar : composeur DiceBear initial-face ──────────────────────
    // Le visage est dérivé du seed (l'API n'applique que backgroundColor) :
    // le stepper « Visage » fait varier le seed parmi FACE_COUNT variantes
    // déterministes du pseudo. L'état sauvegardé est restauré au chargement
    // (encodé dans avatar_filename) — pas d'aléatoire au refresh. Valeurs de
    // départ seulement : ensuite, c'est le membre qui les fait varier.
    const seeds = faceSeeds(untrack(() => data.user.pseudo));
    const saved = decodeAvatarFilename(untrack(() => data.user.avatarFilename));
    let faceIdx       = $state(Math.max(0, saved ? seeds.indexOf(saved.seed) : 0));
    let bgColor       = $state(saved?.backgroundColor ?? BG_COLORS[0]);
    let avatarLoading = $state(false);
    let avatarMsg     = $state('');
    let previewError  = $state(false);
    let previewUrl = $derived(`/avatars/${seeds[faceIdx]}?backgroundColor=${bgColor}`);

    function cycleFace(dir: 1 | -1) {
        faceIdx = (faceIdx + dir + FACE_COUNT) % FACE_COUNT;
    }

    async function saveAvatar() {
        avatarLoading = true; avatarMsg = '';
        try {
            const r = await fetch('/members/avatar', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ seed: seeds[faceIdx], options: { backgroundColor: bgColor } }),
            });
            avatarMsg = r.ok ? 'Avatar mis à jour.' : 'Erreur lors de la mise à jour.';
            if (r.ok) { avatarVersion++; await invalidateAll(); }
        } finally { avatarLoading = false; }
    }

    // ── Suppression de compte ────────────────────────────────────────
    let confirmDelete = $state(false);
    let deleteLoading = $state(false);
    let deleteMsg     = $state('');

    async function deleteAccount() {
        deleteLoading = true; deleteMsg = '';
        try {
            const r = await fetch('/members/account', { method: 'DELETE' });
            const d = await r.json();
            if (d.ok) { goto('/'); }
            else deleteMsg = d.message ?? 'Erreur lors de la suppression.';
        } finally { deleteLoading = false; }
    }
</script>

<!-- ── Navbar ── -->
<nav class="navbar">
    <span class="navbar-brand">rev0auth</span>
    <div class="navbar-right">
        <img
            src="/members/avatar/{data.user.pseudo}?v={avatarVersion}"
            alt="Avatar"
            class="nav-avatar"
            onerror={(e) => (e.currentTarget as HTMLImageElement).style.display = 'none'}
        />
        <span class="nav-pseudo">{data.user.pseudo}</span>
        {#if data.user.role === 'admin'}
            {#if consoleReachable === false}
                <span class="nav-btn nav-btn-off" title="La console n'est joignable que par le réseau privé">
                    Console : active Tailscale
                </span>
            {:else}
                <a class="nav-btn" href={data.consoleUrl}>Console admin</a>
            {/if}
        {/if}
        <ThemeToggle inline />
        <button class="nav-btn nav-btn-logout" onclick={logout}>Déconnexion</button>
    </div>
</nav>

<!-- ── Hero ── -->
<section class="hero">
    <div class="hero-inner">
        <h1 class="hero-title">Bienvenue, <span class="hero-name">{data.user.pseudo}</span></h1>
        <p class="hero-sub">Ton espace privé — hébergé chez nous, sans publicité, sans tracking, sans tiers. Tu accèdes à des services sélectionnés et gardes le contrôle.</p>
    </div>
</section>

<!-- ── Services ── -->
<section class="section">
    <div class="section-inner">
        <h2 class="section-heading">Services</h2>
        <div class="services-grid">

            <div class="svc-card">
                <div class="svc-banner svc-songsurf">
                    <span>🎵</span><span>Songsurf</span>
                </div>
                <div class="svc-body">
                    {#if data.songsurfUrl}
                        <a class="btn-primary svc-launch" href={data.songsurfUrl} target="_blank" rel="noopener">
                            Ouvrir SongSurf →
                        </a>
                    {:else}
                        <p class="meta">Service indisponible pour le moment.</p>
                    {/if}
                </div>
            </div>

            <div class="svc-card">
                <div class="svc-banner svc-jellyfin">
                    <span>🎬</span><span>Jellyfin</span>
                </div>
                <div class="svc-body">
                    <a class="btn-primary svc-launch" href="https://jellyfin.rev0li.duckdns.org" target="_blank" rel="noopener">
                        Ouvrir Jellyfin →
                    </a>
                </div>
            </div>

        </div>
    </div>
</section>

<!-- ── Mon compte ── -->
<section class="section section-last">
    <div class="section-inner">
        <h2 class="section-heading">Mon compte</h2>
        <div class="tiles">
            {#each TILES as t (t.id)}
                <button
                    class="tile"
                    class:open={open === t.id}
                    class:tile-danger={t.id === 'delete'}
                    onclick={() => toggle(t.id)}
                    aria-expanded={open === t.id}
                    aria-controls="tile-panel"
                >
                    <span class="tile-cursor" aria-hidden="true">▶</span>
                    <span class="tile-icon" aria-hidden="true">{t.icon}</span>
                    <span class="tile-text">
                        <span class="tile-label">{t.label}</span>
                        <span class="tile-hint">{t.hint}</span>
                    </span>
                </button>
            {/each}
        </div>

        <!-- Contenu de la tuile choisie, déplié sous la rangée -->
        {#if open}
            <div id="tile-panel" class="tile-panel" class:tile-danger={open === 'delete'} transition:slide={{ duration: 180 }}>
                    {#if open === 'password'}
                        <div class="field">
                            <label for="cur-pwd">Mot de passe actuel</label>
                            <input id="cur-pwd" type="password" bind:value={currentPwd} placeholder="••••••••" autocomplete="current-password" />
                        </div>
                        <div class="field">
                            <label for="new-pwd">Nouveau mot de passe</label>
                            <input id="new-pwd" type="password" bind:value={newPwd} placeholder="••••••••" autocomplete="new-password" />
                        </div>
                        <div class="field">
                            <label for="confirm-pwd">Confirmer</label>
                            <input id="confirm-pwd" type="password" bind:value={confirmPwd} placeholder="••••••••" autocomplete="new-password" />
                        </div>
                        {#if pwdMsg}<p class={pwdOk ? 'chip-ok' : 'chip-error'}>{pwdMsg}</p>{/if}
                        <button class="btn-primary" onclick={changePassword} disabled={pwdLoading}>
                            {pwdLoading ? '…' : 'Mettre à jour'}
                        </button>

                    {:else if open === 'avatar'}
                        <div class="composer">
                            <div class="composer-preview">
                                <img
                                    src={previewUrl}
                                    alt="Aperçu avatar"
                                    class="composer-img"
                                    onload={() => { previewError = false; }}
                                    onerror={() => { previewError = true; }}
                                />
                                {#if previewError}<p class="chip-error">Aperçu indisponible (génération DiceBear) — réessaie.</p>{/if}
                                <button class="btn-primary" onclick={saveAvatar} disabled={avatarLoading}>
                                    {avatarLoading ? '…' : 'Sauvegarder'}
                                </button>
                                {#if avatarMsg}<p class="meta">{avatarMsg}</p>{/if}
                            </div>
                            <div class="composer-sections">
                                <div class="composer-row">
                                    <span class="composer-label">Visage</span>
                                    <button class="composer-arrow" onclick={() => cycleFace(-1)} aria-label="Visage précédent">◀</button>
                                    <span class="composer-value">{faceIdx + 1}/{FACE_COUNT}</span>
                                    <button class="composer-arrow" onclick={() => cycleFace(1)} aria-label="Visage suivant">▶</button>
                                </div>
                                <div class="composer-row">
                                    <span class="composer-label">Fond</span>
                                    <div class="swatches">
                                        {#each BG_COLORS as c (c)}
                                            <button
                                                class="swatch"
                                                class:selected={bgColor === c}
                                                style="background:#{c}"
                                                onclick={() => { bgColor = c; }}
                                                aria-label="Fond #{c}"
                                            ></button>
                                        {/each}
                                    </div>
                                </div>
                            </div>
                        </div>

                    {:else}
                        <p class="meta">La suppression est définitive : ton compte, ton mot de passe et ton avatar seront effacés.</p>
                        {#if !confirmDelete}
                            <button class="btn-danger" onclick={() => confirmDelete = true}>Supprimer mon compte</button>
                        {:else}
                            <p class="chip-error">Tu es sûr ? Cette action est irréversible.</p>
                            <div class="danger-actions">
                                <button class="btn-danger" onclick={deleteAccount} disabled={deleteLoading}>
                                    {deleteLoading ? '…' : 'Oui, supprimer définitivement'}
                                </button>
                                <button class="btn-secondary" onclick={() => confirmDelete = false}>Annuler</button>
                            </div>
                        {/if}
                        {#if deleteMsg}<p class="chip-error">{deleteMsg}</p>{/if}
                    {/if}
            </div>
        {/if}
    </div>
</section>

<style>
    /* ── Navbar ── */
    .navbar {
        position: sticky;
        top: 0;
        z-index: 100;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 1.5rem;
        height: 56px;
        background: var(--card);
        border-bottom: 1px solid var(--border);
        box-shadow: var(--shadow-soft);
    }
    .navbar-brand { font-weight: 700; font-size: 1rem; letter-spacing: -0.01em; }
    .navbar-right { display: flex; align-items: center; gap: 10px; }
    .nav-avatar {
        width: 28px; height: 28px;
        border-radius: 50%;
        object-fit: cover;
        border: 1px solid var(--border);
    }
    .nav-pseudo { font-size: 0.875rem; font-weight: 600; }
    .nav-btn {
        height: 30px; padding: 0 12px;
        border: 1px solid var(--border);
        border-radius: var(--radius-full);
        background: transparent;
        font: 500 0.8125rem/1 var(--font-sans);
        color: var(--foreground);
        text-decoration: none;
        display: inline-flex; align-items: center;
        cursor: pointer;
        transition: background 0.15s;
    }
    .nav-btn:hover { background: var(--muted); }
    .nav-btn-logout { color: var(--destructive); border-color: var(--destructive-border); }
    .nav-btn-off { color: var(--muted-foreground); border-style: dashed; cursor: help; }
    .nav-btn-off:hover { background: transparent; }

    /* ── Hero ── */
    .hero { background: var(--card); border-bottom: 1px solid var(--border); }
    .hero-inner { max-width: 800px; margin: 0 auto; padding: 3rem 1.5rem 2.5rem; }
    .hero-title { font-size: clamp(1.5rem, 4vw, 2.25rem); font-weight: 700; margin: 0 0 0.75rem; }
    .hero-name { color: var(--primary-hover); }
    .hero-sub { color: var(--muted-foreground); max-width: 560px; margin: 0; line-height: 1.6; }

    /* ── Sections ── */
    .section { padding: 2rem 0; border-bottom: 1px solid var(--border); }
    .section-last { border-bottom: none; padding-bottom: 6rem; }
    .section-inner { max-width: 800px; margin: 0 auto; padding: 0 1.5rem; }
    .section-heading {
        font-size: 0.8125rem; font-weight: 700;
        text-transform: uppercase; letter-spacing: 0.06em;
        color: var(--muted-foreground); margin: 0 0 1rem;
    }

    /* ── Services ── */
    .services-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; }
    .svc-card { border: 1px solid var(--border); border-radius: var(--radius-lg); overflow: hidden; background: var(--card); }
    .svc-banner {
        display: flex; align-items: center; gap: 8px;
        padding: 12px 16px; font-weight: 600; font-size: 0.9375rem;
        color: var(--card);
    }
    .svc-songsurf { background: linear-gradient(135deg, #E8B7C4, #DCA2B5); }
    .svc-jellyfin  { background: linear-gradient(135deg, #6EDAD3, #4EC9C1); }
    .svc-body { padding: 1rem; }
    .svc-launch {
        display: inline-flex; align-items: center; justify-content: center;
        margin-top: 0.625rem; text-decoration: none;
    }

    /* ── Mon compte : tuiles façon menu de jeu ── */
    .tiles { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 0.75rem; }
    .tile {
        display: flex; align-items: center; gap: 0.75rem;
        padding: 1rem 1.125rem;
        border: 1px solid var(--border); border-radius: var(--radius-lg);
        background: var(--card); color: var(--foreground);
        text-align: left; font: inherit; cursor: pointer;
        transition: border-color 0.15s, box-shadow 0.15s, transform 0.15s;
    }
    .tile:hover, .tile:focus-visible { border-color: var(--primary); transform: translateY(-2px); outline: none; }
    .tile.open { border-color: var(--primary); box-shadow: 0 0 0 1px var(--primary); transform: none; }
    .tile-danger:hover, .tile-danger:focus-visible, .tile-danger.open { border-color: var(--destructive); }
    .tile-danger.open { box-shadow: 0 0 0 1px var(--destructive); }
    .tile-cursor {
        font-size: 0.625rem; color: var(--primary-hover);
        opacity: 0; transform: translateX(-4px);
        transition: opacity 0.15s, transform 0.15s;
    }
    .tile:hover .tile-cursor, .tile:focus-visible .tile-cursor, .tile.open .tile-cursor { opacity: 1; transform: none; }
    .tile-danger .tile-cursor { color: var(--destructive); }
    .tile-icon { font-size: 1.5rem; line-height: 1; }
    .tile-text { display: flex; flex-direction: column; gap: 2px; }
    .tile-label { font-weight: 700; font-size: 0.9375rem; }
    .tile-danger .tile-label { color: var(--destructive); }
    .tile-hint { font-size: 0.8125rem; color: var(--muted-foreground); }
    .tile-panel {
        margin-top: 0.75rem; padding: 1.25rem 1.25rem 1.5rem;
        border: 1px solid var(--primary); border-radius: var(--radius-lg);
        background: var(--card);
    }
    .tile-panel.tile-danger { border-color: var(--destructive); }

    /* ── Suppression ── */
    .danger-actions { display: flex; gap: 8px; flex-wrap: wrap; margin-top: 0.5rem; }
    .btn-danger {
        height: 34px; padding: 0 16px;
        background: var(--destructive); color: #fff;
        border: none; border-radius: var(--radius-md);
        font: 600 0.8125rem/1 var(--font-sans);
        cursor: pointer;
    }
    .btn-danger:hover:not(:disabled) { opacity: 0.9; }
    .btn-danger:disabled { opacity: 0.6; cursor: default; }

    /* ── Composeur d'avatar ── */
    .composer { display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: flex-start; }
    .composer-preview {
        display: flex; flex-direction: column; align-items: center; gap: 0.75rem;
        flex-shrink: 0;
    }
    .composer-img {
        width: 120px; height: 120px; border-radius: 50%;
        object-fit: cover; border: 2px solid var(--border); background: var(--muted);
    }
    .composer-sections { flex: 1; min-width: 240px; display: flex; flex-direction: column; gap: 8px; }
    .composer-row { display: flex; align-items: center; gap: 8px; }
    .composer-label {
        width: 70px; flex-shrink: 0;
        font-size: 0.8125rem; font-weight: 600; color: var(--muted-foreground);
    }
    .composer-arrow {
        width: 28px; height: 28px;
        border: 1px solid var(--border); border-radius: var(--radius-sm);
        background: var(--muted); cursor: pointer;
        font-size: 0.6875rem; color: var(--foreground);
        transition: border-color 0.12s;
    }
    .composer-arrow:hover { border-color: var(--primary); }
    .composer-value {
        min-width: 56px; text-align: center;
        font-size: 0.8125rem; color: var(--foreground);
        font-variant-numeric: tabular-nums;
    }
    .swatches { display: flex; gap: 5px; flex-wrap: wrap; }
    .swatch {
        width: 22px; height: 22px; border-radius: 50%;
        border: 2px solid var(--border); cursor: pointer;
        transition: transform 0.12s, border-color 0.12s;
    }
    .swatch:hover { transform: scale(1.15); }
    .swatch.selected { border-color: var(--primary); box-shadow: 0 0 0 2px var(--primary); }
</style>
