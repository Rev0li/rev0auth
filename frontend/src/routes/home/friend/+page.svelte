<script lang="ts">
    import type { PageData } from './$types.js';
    import { invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { slide, fade } from 'svelte/transition';
    import Chat from './Chat.svelte';
    import ThemeToggle from '$lib/ThemeToggle.svelte';
    import { EMOJIS } from '$lib/emojis.js';
    import EmojiText from '$lib/EmojiText.svelte';

    let { data }: { data: PageData } = $props();

    // Onboarding
    let showOnboarding = $state($page.url.searchParams.get('onboarding') === '1');
    let newPwd     = $state('');
    let confirmPwd = $state('');
    let onbError   = $state('');
    let onbLoading = $state(false);

    // Wall
    let wallPosts  = $state([...data.wall]);
    let wallBody   = $state('');
    let wallLoading = $state(false);
    let wallError  = $state('');

    // Demandes d'accès services (conditions : star GitHub / reco LinkedIn)
    let githubInput   = $state('');
    let linkedinInput = $state('');
    let svcLoading    = $state('');
    let svcError      = $state('');

    async function postWall() {
        wallError = '';
        const body = wallBody.trim();
        if (!body) return;
        wallLoading = true;
        try {
            const r = await fetch('/members/wall', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ body }),
            });
            if (r.ok) {
                wallBody = '';
                // Optimistic: reload wall
                const fresh = await fetch('/members/wall');
                if (fresh.ok) wallPosts = await fresh.json();
            }
        } finally { wallLoading = false; }
    }

    async function deleteWallPost(id: number) {
        wallPosts = wallPosts.filter(p => p.id !== id);
        await fetch(`/members/wall?id=${id}`, { method: 'DELETE' });
    }

    async function requestService(service: 'songsurf' | 'jellyfin') {
        svcError = '';
        const payload = service === 'songsurf'
            ? { service, github_username: githubInput.trim() }
            : { service, linkedin_name: linkedinInput.trim() };
        svcLoading = service;
        try {
            const r = await fetch('/members/access/request', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(payload),
            });
            if (!r.ok) {
                const d = await r.json();
                svcError = d.message ?? 'Erreur.';
                return;
            }
            invalidateAll();
        } finally { svcLoading = ''; }
    }

    async function submitOnboarding() {
        onbError = '';
        if (!newPwd || newPwd !== confirmPwd) { onbError = 'Les mots de passe ne correspondent pas.'; return; }
        if (newPwd.length < 8) { onbError = 'Minimum 8 caractères.'; return; }
        onbLoading = true;
        try {
            const r = await fetch('/members/password/onboarding', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ newPassword: newPwd }),
            });
            if (r.ok) { showOnboarding = false; invalidateAll(); }
            else { const d = await r.json(); onbError = d.message; }
        } finally { onbLoading = false; }
    }

    function timeAgo(date: Date | string | number) {
        const d   = typeof date === 'number' ? new Date(date * 1000) : new Date(date);
        const sec = Math.floor((Date.now() - d.getTime()) / 1000);
        if (sec < 60) return 'à l\'instant';
        if (sec < 3600) return `il y a ${Math.floor(sec / 60)} min`;
        if (sec < 86400) return `il y a ${Math.floor(sec / 3600)} h`;
        return `il y a ${Math.floor(sec / 86400)} j`;
    }

    onMount(() => {
        // Re-fetch wall on mount (data might be stale)
        fetch('/members/wall').then(r => r.json()).then(d => { if (Array.isArray(d)) wallPosts = d; });
    });
</script>

<!-- ── Onboarding modal ── -->
{#if showOnboarding}
    <div class="modal-bg" transition:fade={{ duration: 200 }}>
        <div class="card onboarding-card">
            <h2>Première connexion</h2>
            <p class="meta">Choisis ton mot de passe avant de continuer.</p>
            <div class="field" style="margin-top:1rem">
                <label for="ob-pwd">Nouveau mot de passe</label>
                <input id="ob-pwd" type="password" bind:value={newPwd} placeholder="••••••••" />
            </div>
            <div class="field">
                <label for="ob-confirm">Confirmer</label>
                <input id="ob-confirm" type="password" bind:value={confirmPwd} placeholder="••••••••" />
            </div>
            {#if onbError}<p class="chip-error">{onbError}</p>{/if}
            <button class="btn-primary btn-full" onclick={submitOnboarding} disabled={onbLoading}>
                {onbLoading ? '…' : 'Valider et continuer'}
            </button>
        </div>
    </div>
{/if}

<!-- ── Navbar ── -->
<nav class="navbar">
    <span class="navbar-brand">rev0auth</span>
    <div class="navbar-right">
        <img
            src="/members/avatar/{data.user.pseudo}"
            alt="Avatar"
            class="nav-avatar"
            onerror={(e) => (e.currentTarget as HTMLImageElement).style.display = 'none'}
        />
        <span class="nav-pseudo">{data.user.pseudo}</span>
        <a class="nav-btn" href="/members/profile">Profil</a>
        <ThemeToggle inline />
    </div>
</nav>

<!-- ── Hero ── -->
<section class="hero">
    <div class="hero-inner">
        <h1 class="hero-title">Bienvenue, <span class="hero-name">{data.user.pseudo}</span></h1>
        <p class="hero-sub">Ton espace privé — hébergé chez nous, sans publicité, sans tracking, sans tiers. Tu accèdes à des services sélectionnés, partages avec la communauté et gardes le contrôle.</p>
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
                    {#if data.user.accessSongsurf}
                        <p class="chip-ok">✓ Accès accordé</p>
                        {#if data.user.githubUsername}
                            <p class="meta">@{data.user.githubUsername}</p>
                        {/if}
                    {:else if data.user.requestSongsurf}
                        <p class="meta">Demande en attente…</p>
                    {:else}
                        <ol class="svc-steps">
                            <li>Mets une étoile au repo
                                <a href="https://github.com/Rev0li/SongSurf" target="_blank" rel="noopener">⭐ SongSurf</a>
                            </li>
                            <li>Renseigne ton pseudo GitHub :</li>
                        </ol>
                        <input class="svc-input" bind:value={githubInput} placeholder="ton_pseudo_github" />
                        <button
                            class="btn-secondary"
                            onclick={() => requestService('songsurf')}
                            disabled={svcLoading === 'songsurf' || !githubInput.trim()}
                        >{svcLoading === 'songsurf' ? '…' : 'Envoyer la demande'}</button>
                    {/if}
                </div>
            </div>

            <div class="svc-card">
                <div class="svc-banner svc-jellyfin">
                    <span>🎬</span><span>Jellyfin</span>
                </div>
                <div class="svc-body">
                    {#if data.user.accessJellyfin}
                        <p class="chip-ok">✓ Accès accordé</p>
                    {:else if data.user.requestJellyfin}
                        <p class="meta">Demande en attente…</p>
                    {:else}
                        <ol class="svc-steps">
                            <li>Laisse une recommandation sur
                                <a href="https://linkedin.com/in/oliver-kientzler" target="_blank" rel="noopener">LinkedIn</a>
                            </li>
                            <li>Renseigne ton nom LinkedIn :</li>
                        </ol>
                        <input class="svc-input" bind:value={linkedinInput} placeholder="ton_profil_linkedin" />
                        <button
                            class="btn-secondary"
                            onclick={() => requestService('jellyfin')}
                            disabled={svcLoading === 'jellyfin' || !linkedinInput.trim()}
                        >{svcLoading === 'jellyfin' ? '…' : 'Envoyer la demande'}</button>
                    {/if}
                </div>
            </div>

        </div>
        {#if svcError}<p class="chip-error" style="margin-top:0.75rem">{svcError}</p>{/if}
    </div>
</section>

<!-- ── Membres en ligne ── -->
<section class="section">
    <div class="section-inner">
        <h2 class="section-heading">Membres</h2>
        <div class="members-row">
            {#each data.members as m}
                <div class="member-chip" title={m.bio ?? m.pseudo}>
                    <img
                        src="/members/avatar/{m.pseudo}"
                        alt={m.pseudo}
                        class="member-avatar"
                        onerror={(e) => { (e.currentTarget as HTMLImageElement).style.display='none'; }}
                    />
                    <span class="member-pseudo">{m.pseudo}</span>
                </div>
            {/each}
        </div>
    </div>
</section>

<!-- ── Wall ── -->
<section class="section section-last">
    <div class="section-inner">
        <h2 class="section-heading">Mur</h2>
        <p class="wall-hint">Partage tes idées d'amélioration, ou les films et séries que tu aimerais voir ajoutés à Jellyfin 🍿</p>

        <div class="wall-emojis">
            {#each EMOJIS as e (e.char)}
                <button class="emoji-btn" onclick={() => { wallBody += e.char; }} aria-label="Ajouter {e.name}" title={e.name}>
                    <img src={e.src} alt={e.char} />
                </button>
            {/each}
        </div>
        <div class="wall-compose">
            <textarea
                bind:value={wallBody}
                placeholder="Écris quelque chose…"
                rows={2}
                class="wall-input"
                onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); postWall(); } }}
            ></textarea>
            <button class="btn-primary" onclick={postWall} disabled={wallLoading || !wallBody.trim()}>
                {wallLoading ? '…' : 'Publier'}
            </button>
        </div>

        <div class="wall-posts">
            {#each wallPosts as post (post.id)}
                <div class="wall-post" transition:slide={{ duration: 200 }}>
                    <div class="post-header">
                        <strong class="post-author">{post.pseudo}</strong>
                        <span class="post-time meta">{timeAgo(post.createdAt)}</span>
                        {#if post.pseudo === data.user.pseudo}
                            <button class="post-delete" onclick={() => deleteWallPost(post.id)} aria-label="Supprimer">×</button>
                        {/if}
                    </div>
                    <p class="post-body"><EmojiText text={post.body} /></p>
                </div>
            {:else}
                <p class="meta" style="text-align:center;padding:2rem 0">Aucun post pour l'instant.</p>
            {/each}
        </div>
    </div>
</section>

<!-- ── Messagerie ── -->
<Chat
    myPseudo={data.user.pseudo}
    adminPseudo={data.adminPseudo}
    members={data.members}
    initialUnread={data.unreadCount}
/>

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
    .svc-steps {
        font-size: 0.8125rem; color: var(--muted-foreground);
        margin: 0 0 0.75rem; padding-left: 1.1rem;
        display: flex; flex-direction: column; gap: 4px;
    }
    .svc-steps a { color: var(--primary-hover); font-weight: 600; text-decoration: none; }
    .svc-steps a:hover { text-decoration: underline; }
    .svc-input {
        width: 100%; margin-bottom: 0.625rem;
        padding: 7px 10px; font-size: 0.8125rem;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--background); color: var(--foreground);
    }
    .svc-input:focus { outline: none; border-color: var(--primary); }

    /* ── Members ── */
    .members-row { display: flex; flex-wrap: wrap; gap: 10px; }
    .member-chip {
        display: flex; align-items: center; gap: 6px;
        padding: 6px 10px; border: 1px solid var(--border);
        border-radius: var(--radius-full); background: var(--muted);
        font-size: 0.8125rem;
    }
    .member-avatar { width: 22px; height: 22px; border-radius: 50%; object-fit: cover; }
    .member-pseudo { font-weight: 500; }

    /* ── Wall ── */
    .wall-hint { font-size: 0.875rem; color: var(--muted-foreground); margin: -0.5rem 0 0.75rem; }
    .wall-emojis { display: flex; gap: 2px; flex-wrap: wrap; margin-bottom: 6px; }
    .emoji-btn {
        background: none; border: none; cursor: pointer;
        padding: 2px 4px; border-radius: var(--radius-sm);
        transition: background 0.12s, transform 0.12s;
        line-height: 0;
    }
    .emoji-btn img { width: 24px; height: 24px; }
    .emoji-btn:hover { background: var(--muted); transform: scale(1.2); }
    .wall-compose { display: flex; gap: 8px; align-items: flex-end; margin-bottom: 1.25rem; }
    .wall-input { flex: 1; min-height: 60px; resize: none; }
    .wall-posts { display: flex; flex-direction: column; gap: 10px; }
    .wall-post { background: var(--card); border: 1px solid var(--border); border-radius: var(--radius-lg); padding: 12px 14px; }
    .post-header { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
    .post-author { font-size: 0.875rem; }
    .post-time { margin-left: auto; }
    .post-delete {
        background: none; border: none; cursor: pointer;
        color: var(--muted-foreground); font-size: 1.125rem; padding: 0 2px;
        transition: color 0.15s;
    }
    .post-delete:hover { color: var(--destructive); }
    .post-body { margin: 0; font-size: 0.9375rem; line-height: 1.5; white-space: pre-wrap; }

    /* ── Onboarding modal ── */
    .modal-bg {
        position: fixed; inset: 0; z-index: 500;
        background: rgba(0,0,0,0.5);
        display: flex; align-items: center; justify-content: center;
        padding: 1rem;
    }
    .onboarding-card {
        width: 100%; max-width: 400px;
        padding: 2rem;
        background: var(--card);
    }
    .onboarding-card h2 { margin: 0 0 0.5rem; }
</style>
