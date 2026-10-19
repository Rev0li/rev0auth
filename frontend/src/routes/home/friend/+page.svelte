<script lang="ts">
    import type { PageData } from './$types.js';
    import { invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import { fade } from 'svelte/transition';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { data }: { data: PageData } = $props();

    // Onboarding
    let showOnboarding = $state($page.url.searchParams.get('onboarding') === '1');
    let newPwd     = $state('');
    let confirmPwd = $state('');
    let onbError   = $state('');
    let onbLoading = $state(false);

    // Demandes d'accès services (conditions : star GitHub / reco LinkedIn)
    let githubInput   = $state('');
    let linkedinInput = $state('');
    let svcLoading    = $state('');
    let svcError      = $state('');

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
                    {#if data.user.accessSongsurf}
                        <a class="btn-primary svc-launch" href="/members/songsurf/launch" target="_blank" rel="noopener">
                            Ouvrir SongSurf →
                        </a>
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
                        <a class="btn-primary svc-launch" href="https://revoli-jellyfin.duckdns.org" target="_blank" rel="noopener">
                            Ouvrir Jellyfin →
                        </a>
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
<section class="section section-last">
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
    .svc-launch {
        display: inline-flex; align-items: center; justify-content: center;
        margin-top: 0.625rem; text-decoration: none;
    }

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
