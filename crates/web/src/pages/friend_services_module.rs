pub const JS_FRIEND_SERVICES_MODULE: &str = r#"
function createFriendServicesModule(ctx) {
    const { pseudo } = ctx;

    const state = {
        songsurf: false, jellyfin: false,
        reqSongsurf: false, reqJellyfin: false,
        githubUsername: null, linkedinName: null
    };

    const msgEl = document.getElementById('service-msg');

    function setMsg(ok, text) {
        if (!msgEl) return;
        msgEl.className = 'service-msg ' + (ok ? 'ok' : 'error');
        msgEl.textContent = text;
        msgEl.style.display = 'block';
        if (ok) setTimeout(() => { msgEl.style.display = 'none'; }, 3500);
    }

    function renderSongsurf() {
        const body = document.getElementById('songsurf-body');
        if (!body) return;
        if (state.songsurf) {
            body.innerHTML =
                '<p class="svc-state svc-open">✓ Accès ouvert</p>'
                + '<a class="svc-btn" href="%%SONGSURF_URL%%" target="_blank" rel="noopener">Ouvrir Songsurf →</a>';
        } else if (state.reqSongsurf) {
            body.innerHTML =
                '<p class="svc-state">⏳ Demande envoyée — en attente de validation.</p>'
                + (state.githubUsername
                    ? '<p class="svc-submitted">GitHub : <strong>@' + escapeHtml(state.githubUsername) + '</strong></p>'
                    : '');
        } else {
            body.innerHTML =
                '<ol class="svc-steps">'
                + '<li>⭐ <a href="https://github.com/Rev0li/SongSurf" target="_blank" rel="noopener">Star le repo SongSurf</a></li>'
                + '<li>👤 <a href="https://github.com/Rev0li" target="_blank" rel="noopener">Follow Rev0li sur GitHub</a></li>'
                + '<li>Entre ton pseudo GitHub ci-dessous</li>'
                + '</ol>'
                + '<input id="songsurf-gh-input" class="svc-input" placeholder="Ton pseudo GitHub"'
                + ' value="' + escapeHtml(state.githubUsername || '') + '" />'
                + '<button class="svc-btn svc-btn-request" id="songsurf-req-btn">Envoyer ma demande</button>';
            document.getElementById('songsurf-req-btn').addEventListener('click', submitSongsurf);
        }
    }

    function renderJellyfin() {
        const body = document.getElementById('jellyfin-body');
        if (!body) return;
        if (state.jellyfin) {
            body.innerHTML =
                '<p class="svc-state svc-open">✓ Accès ouvert</p>'
                + '<a class="svc-btn" href="https://revoli-jellyfin.duckdns.org" target="_blank" rel="noopener">Ouvrir Jellyfin →</a>';
        } else if (state.reqJellyfin) {
            body.innerHTML =
                '<p class="svc-state">⏳ Demande envoyée — en attente de validation.</p>'
                + (state.linkedinName
                    ? '<p class="svc-submitted">LinkedIn : <strong>' + escapeHtml(state.linkedinName) + '</strong></p>'
                    : '');
        } else {
            body.innerHTML =
                '<ol class="svc-steps">'
                + '<li>🤝 <a href="https://linkedin.com/in/oliver-kientzler" target="_blank" rel="noopener">Se connecter sur LinkedIn</a></li>'
                + '<li>⭐ Me recommander sur LinkedIn</li>'
                + '<li>Entre ton nom LinkedIn ci-dessous</li>'
                + '</ol>'
                + '<input id="jellyfin-li-input" class="svc-input" placeholder="Ton nom LinkedIn"'
                + ' value="' + escapeHtml(state.linkedinName || '') + '" />'
                + '<button class="svc-btn svc-btn-request" id="jellyfin-req-btn">Envoyer ma demande</button>';
            document.getElementById('jellyfin-req-btn').addEventListener('click', submitJellyfin);
        }
    }

    async function submitSongsurf() {
        const input = document.getElementById('songsurf-gh-input');
        const username = input ? input.value.trim() : '';
        if (!username) { setMsg(false, 'Entre ton pseudo GitHub avant d\'envoyer.'); return; }
        try {
            const res = await fetch('/members/access/request', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, service: 'songsurf', github_username: username })
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Réponse reçue.');
            if (data.ok) await loadState();
        } catch (err) { setMsg(false, 'Erreur: ' + err.message); }
    }

    async function submitJellyfin() {
        const input = document.getElementById('jellyfin-li-input');
        const name = input ? input.value.trim() : '';
        if (!name) { setMsg(false, 'Entre ton nom LinkedIn avant d\'envoyer.'); return; }
        try {
            const res = await fetch('/members/access/request', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, service: 'jellyfin', linkedin_name: name })
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Réponse reçue.');
            if (data.ok) await loadState();
        } catch (err) { setMsg(false, 'Erreur: ' + err.message); }
    }

    async function loadState() {
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
            const data = await res.json();
            state.songsurf     = !!data.access_songsurf;
            state.jellyfin     = !!data.access_jellyfin;
            state.reqSongsurf  = !!data.request_songsurf;
            state.reqJellyfin  = !!data.request_jellyfin;
            state.githubUsername = data.github_username || null;
            state.linkedinName   = data.linkedin_name || null;
            renderSongsurf();
            renderJellyfin();
        } catch (err) {
            setMsg(false, 'Impossible de charger l\'état des accès.');
        }
    }

    loadState();
    return { loadState };
}
"#;

pub const CSS_FRIEND_SERVICES_STYLES: &str = r#"
        .services-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
            gap: 10px;
        }
        .svc-card {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            box-shadow: var(--shadow-soft);
            transition: box-shadow 0.15s;
        }
        .svc-card:hover { box-shadow: var(--shadow-soft); }
        .svc-card-banner {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 12px 14px;
            font-weight: 700;
            font-size: 0.875rem;
        }
        .svc-banner-songsurf { background: #0f0f0f; color: #fafafa; }
        .svc-banner-jellyfin { background: #00a4dc; color: #fff; }
        .svc-icon { font-size: 1rem; }
        .svc-card-body {
            padding: 12px 14px 14px;
            display: flex;
            flex-direction: column;
            gap: 8px;
            flex: 1;
        }
        .svc-steps {
            margin: 0;
            padding-left: 16px;
            display: flex;
            flex-direction: column;
            gap: 5px;
            font-size: 0.875rem;
            color: var(--muted-foreground);
            line-height: 1.5;
        }
        .svc-steps a {
            color: var(--foreground);
            font-weight: 600;
            text-decoration: underline;
            text-underline-offset: 2px;
        }
        .svc-input {
            width: 100%;
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 7px 10px;
            font: inherit;
            font-size: 0.875rem;
            background: var(--muted);
            color: var(--foreground);
            outline: none;
            transition: border-color 0.15s;
        }
        .svc-input:focus { border-color: var(--foreground); background: var(--card); }
        .svc-btn {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            height: 34px;
            border-radius: var(--radius-md);
            border: none;
            font: 600 0.875rem/1 var(--font-sans);
            cursor: pointer;
            text-decoration: none;
            background: var(--primary);
            color: var(--primary-foreground);
            transition: background 0.15s;
        }
        .svc-btn:hover { background: var(--primary-hover); }
        .svc-btn-request {
            background: var(--card);
            color: var(--foreground);
            border: 1px solid var(--border);
        }
        .svc-btn-request:hover { background: var(--muted); }
        .svc-state          { font-size: 0.8125rem; color: var(--muted-foreground); margin: 0; }
        .svc-state.svc-open { color: var(--success); font-weight: 600; }
        .svc-submitted      { font-size: 0.8125rem; color: var(--muted-foreground); margin: 0; }
        .service-msg {
            margin-top: 8px;
            padding: 7px 11px;
            border-radius: var(--radius-md);
            font-size: 0.875rem;
            display: none;
        }
        .service-msg.ok    { display: block; background: var(--success-bg); color: var(--success); border: 1px solid var(--success-border); }
        .service-msg.error { display: block; background: var(--destructive-bg);  color: var(--destructive);  border: 1px solid var(--destructive-border); }
"#;
