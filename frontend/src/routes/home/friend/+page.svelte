<script lang="ts">
    import type { PageData } from './$types.js';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { data }: { data: PageData } = $props();
</script>

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
        {#if data.user.role === 'admin'}
            <a class="nav-btn" href={data.consoleUrl}>Console admin</a>
        {/if}
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
</style>
