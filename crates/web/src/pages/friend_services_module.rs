pub const JS_FRIEND_SERVICES_MODULE: &str = r##"
function createFriendServicesModule(ctx) {
    const { pseudo } = ctx;

    const state = {
        songsurf: false, jellyfin: false,
        reqSongsurf: false, reqJellyfin: false,
        githubUsername: null, linkedinName: null,
        songsurfUrl: null, approved: false
    };

    const msgEl = document.getElementById('service-msg');

    function setMsg(ok, text) {
        if (!msgEl) return;
        msgEl.className = 'service-msg ' + (ok ? 'ok' : 'error');
        msgEl.textContent = text;
        msgEl.style.display = 'block';
        if (ok) setTimeout(() => { msgEl.style.display = 'none'; }, 3500);
    }

    // ---- SongSurf onboarding modal ----

    const SS_SLIDES = [
        {
            title: 'Ouvre ta musique',
            desc: 'Va sur ton site de streaming préféré. Navigue vers un album.',
            image: '/static/tuto/step1.webp',
            placeholder: 'Capture d\'écran — page album avec URL visible dans la barre d\'adresse',
            instruction: '📂 Ouvre une page album'
        },
        {
            title: 'Copie le lien',
            desc: 'Dans la barre d\'adresse de ton navigateur, sélectionne et copie l\'URL complète de la page.',
            image: '/static/tuto/step2.webp',
            placeholder: 'Capture d\'écran — barre d\'adresse encadrée en rouge, URL sélectionnée',
            instruction: '📋 Ctrl+C — ou clic droit → Copier l\'adresse'
        },
        {
            title: 'Colle dans SongSurf',
            desc: 'Colle l\'URL dans le champ de SongSurf. Le téléchargement démarre automatiquement.',
            image: '/static/tuto/step3.webp',
            placeholder: 'Capture d\'écran — champ SongSurf avec URL collée, bouton valider',
            instruction: '▶ Ctrl+V puis Entrée'
        },
        {
            title: 'Récupère ta musique',
            desc: 'Une fois terminé, le zip contient tes fichiers audio. Tes bibliothèques en ligne peuvent disparaître ;)',
            image: '/static/tuto/step4.webp',
            placeholder: 'Capture d\'écran — dossier avec les fichiers téléchargés',
            instruction: '🎼 N\'oublie pas de passer par un éditeur de métadonnées pour tout remettre en ordre'
        }
    ];

    let ssCurrentSlide = 0;
    let ssModalReady = false;

    function injectModal() {
        if (ssModalReady) return;
        ssModalReady = true;

        const overlay = document.createElement('div');
        overlay.id = 'ss-modal';
        overlay.className = 'ss-modal-overlay';
        overlay.innerHTML =
            '<div class="ss-modal-card" role="dialog" aria-modal="true">'
            + '<button class="ss-close" id="ss-close" aria-label="Fermer">✕</button>'
            + '<div class="ss-modal-header">'
            +   '<span class="ss-logo-text">♪ SongSurf</span>'
            +   '<span class="ss-counter" id="ss-counter">1 / ' + SS_SLIDES.length + '</span>'
            + '</div>'
            + '<div class="ss-slides-wrap" id="ss-slides-wrap"></div>'
            + '<div class="ss-modal-footer">'
            +   '<div class="ss-dots" id="ss-dots"></div>'
            +   '<div class="ss-nav">'
            +     '<button class="ss-arrow" id="ss-prev" aria-label="Précédent">←</button>'
            +     '<button class="ss-arrow" id="ss-next" aria-label="Suivant">→</button>'
            +   '</div>'
            + '</div>'
            + '<a class="ss-access-link" id="ss-access-link" href="#">'
            +   'Accéder à SongSurf →'
            + '</a>'
            + '</div>';
        document.body.appendChild(overlay);

        // Slides + dots
        const wrap = document.getElementById('ss-slides-wrap');
        const dotsEl = document.getElementById('ss-dots');
        SS_SLIDES.forEach(function(s, i) {
            const slide = document.createElement('div');
            slide.className = 'ss-slide' + (i === 0 ? ' active' : '');
            const mediaHtml = s.image
                ? '<div class="ss-img-wrap"><img class="ss-img" src="' + s.image + '" alt="' + s.placeholder + '" loading="lazy" /></div>'
                : '<div class="ss-placeholder"><span class="ss-placeholder-label">' + s.placeholder + '</span></div>';
            slide.innerHTML =
                '<h3 class="ss-slide-title">' + s.title + '</h3>'
                + '<p class="ss-slide-desc">' + s.desc + '</p>'
                + mediaHtml
                + '<div class="ss-instruction">' + s.instruction + '</div>';
            wrap.appendChild(slide);

            const dot = document.createElement('button');
            dot.className = 'ss-dot' + (i === 0 ? ' active' : '');
            dot.setAttribute('aria-label', 'Slide ' + (i + 1));
            dot.addEventListener('click', function() { goSlide(i); });
            dotsEl.appendChild(dot);
        });

        // Events
        document.getElementById('ss-close').addEventListener('click', closeModal);
        overlay.addEventListener('click', function(e) { if (e.target === overlay) closeModal(); });
        document.getElementById('ss-prev').addEventListener('click', function() { goSlide(ssCurrentSlide - 1); });
        document.getElementById('ss-next').addEventListener('click', function() { goSlide(ssCurrentSlide + 1); });
        document.addEventListener('keydown', function(e) {
            const modal = document.getElementById('ss-modal');
            if (!modal || !modal.classList.contains('open')) return;
            if (e.key === 'ArrowLeft')  goSlide(ssCurrentSlide - 1);
            if (e.key === 'ArrowRight') goSlide(ssCurrentSlide + 1);
            if (e.key === 'Escape')     closeModal();
        });
    }

    function goSlide(idx) {
        const slides = document.querySelectorAll('.ss-slide');
        const dots   = document.querySelectorAll('.ss-dot');
        if (idx < 0 || idx >= slides.length) return;
        slides[ssCurrentSlide].classList.remove('active');
        dots[ssCurrentSlide].classList.remove('active');
        ssCurrentSlide = idx;
        slides[ssCurrentSlide].classList.add('active');
        dots[ssCurrentSlide].classList.add('active');
        document.getElementById('ss-counter').textContent = (ssCurrentSlide + 1) + ' / ' + SS_SLIDES.length;
        document.getElementById('ss-prev').style.opacity = ssCurrentSlide === 0 ? '0.25' : '1';
        document.getElementById('ss-next').style.opacity = ssCurrentSlide === slides.length - 1 ? '0.25' : '1';
    }

    function openModal() {
        injectModal();
        goSlide(0);
        const link = document.getElementById('ss-access-link');
        if (link) link.href = state.songsurfUrl || '%%SONGSURF_URL%%';
        document.getElementById('ss-modal').classList.add('open');
        document.body.style.overflow = 'hidden';
    }

    function closeModal() {
        const modal = document.getElementById('ss-modal');
        if (modal) modal.classList.remove('open');
        document.body.style.overflow = '';
    }

    // ---- card renders ----

    function pendingHtml() {
        return '<p class="svc-state svc-pending">⏳ Compte en attente de validation par l\'admin.</p>';
    }

    function renderSongsurf() {
        const body   = document.getElementById('songsurf-body');
        const card   = document.getElementById('songsurf-card');
        const status = document.getElementById('ss-svc-status');
        if (!body) return;
        if (!state.approved) { body.innerHTML = pendingHtml(); return; }

        if (state.songsurf) {
            if (status) {
                status.innerHTML = '<span class="ss-status-dot"></span><span class="ss-status-ring"></span>';
            }
            body.innerHTML =
                '<div class="ss-card-inner">'
                + '<div class="ss-card-logo">♪</div>'
                + '<p class="ss-card-label">Clique pour démarrer</p>'
                + '<span class="ss-card-arrow">→</span>'
                + '</div>';
            if (card) {
                card.classList.add('svc-card-clickable');
                card.onclick = openModal;
            }
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
        if (!state.approved) { body.innerHTML = pendingHtml(); return; }
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

    // ---- submissions ----

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
            state.songsurf       = !!data.access_songsurf;
            state.jellyfin       = !!data.access_jellyfin;
            state.reqSongsurf    = !!data.request_songsurf;
            state.reqJellyfin    = !!data.request_jellyfin;
            state.githubUsername = data.github_username || null;
            state.linkedinName   = data.linkedin_name   || null;
            state.songsurfUrl    = data.songsurf_url || null;
            state.approved       = !!data.approved;
            renderSongsurf();
            renderJellyfin();
        } catch (err) {
            setMsg(false, 'Impossible de charger l\'état des accès.');
        }
    }

    loadState();
    return { loadState };
}
"##;

pub const CSS_FRIEND_SERVICES_STYLES: &str = r##"
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
            transition: box-shadow 0.2s, border-color 0.2s, transform 0.15s;
        }
        .svc-card-clickable {
            cursor: pointer;
        }
        .svc-card-clickable:hover {
            border-color: rgba(232,183,196,0.6);
            box-shadow: 0 0 0 2px rgba(232,183,196,0.18), var(--shadow-soft);
            transform: translateY(-2px);
        }
        .svc-card-banner {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 12px 14px;
            font-weight: 700;
            font-size: 0.875rem;
        }
        .svc-banner-songsurf { background: #0f0f0f; color: #fafafa; }
        .svc-icon { font-size: 1rem; }
        .svc-card-body {
            padding: 12px 14px 14px;
            display: flex;
            flex-direction: column;
            gap: 8px;
            flex: 1;
        }

        /* ---- SongSurf card — MyCss-aligned header ---- */
        .ss-svc-card {
            border-radius: 18px;
        }
        .ss-svc-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 10px 14px;
            background: var(--muted);
            border-bottom: 1px solid var(--border);
        }
        .ss-svc-badge {
            font-size: 0.8125rem;
            font-weight: 700;
            color: #E8B7C4;
            background: rgba(232,183,196,0.12);
            border: 1px solid rgba(232,183,196,0.3);
            border-radius: 9999px;
            padding: 2px 10px;
            letter-spacing: 0.01em;
        }

        /* ---- Jellyfin card — same structure as SongSurf ---- */
        .jf-svc-card {
            border-radius: 18px;
        }
        .jf-svc-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 10px 14px;
            background: var(--muted);
            border-bottom: 1px solid var(--border);
        }
        .jf-svc-badge {
            font-size: 0.8125rem;
            font-weight: 700;
            color: #00a4dc;
            background: rgba(0,164,220,0.10);
            border: 1px solid rgba(0,164,220,0.28);
            border-radius: 9999px;
            padding: 2px 10px;
            letter-spacing: 0.01em;
        }
        .ss-svc-status {
            position: relative;
            width: 14px;
            height: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .ss-status-dot {
            width: 9px;
            height: 9px;
            border-radius: 50%;
            background: #6EDAD3;
            position: relative;
            z-index: 1;
        }
        .ss-status-ring {
            position: absolute;
            inset: -2px;
            border-radius: 50%;
            border: 2px solid #6EDAD3;
            opacity: 0;
            animation: ss-pulse 2s ease-out infinite;
        }
        @keyframes ss-pulse {
            0%   { opacity: 0.7; transform: scale(0.8); }
            100% { opacity: 0;   transform: scale(2.2); }
        }

        /* ---- Card activated state ---- */
        .ss-card-inner {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            gap: 8px;
            padding: 18px 0 10px;
            position: relative;
        }
        .ss-card-logo {
            font-size: 2rem;
            color: var(--primary);
            line-height: 1;
        }
        .ss-card-label {
            font-size: 0.8125rem;
            color: var(--muted-foreground);
            margin: 0;
        }
        .ss-card-arrow {
            font-size: 1rem;
            color: var(--muted-foreground);
            transition: transform 0.2s;
        }
        .svc-card-clickable:hover .ss-card-arrow { transform: translateX(4px); }

        /* ---- Modal overlay ---- */
        .ss-modal-overlay {
            display: none;
            position: fixed;
            inset: 0;
            background: rgba(0,0,0,0.72);
            backdrop-filter: blur(6px);
            z-index: 1000;
            align-items: center;
            justify-content: center;
            padding: 16px;
        }
        .ss-modal-overlay.open {
            display: flex;
        }
        .ss-modal-card {
            position: relative;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 16px;
            width: 100%;
            max-width: 560px;
            padding: 28px 28px 20px;
            display: flex;
            flex-direction: column;
            gap: 20px;
            box-shadow: 0 24px 64px rgba(0,0,0,0.5);
        }

        /* ---- Modal header ---- */
        .ss-modal-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .ss-logo-text {
            font-size: 1rem;
            font-weight: 700;
            color: var(--foreground);
            letter-spacing: -0.01em;
        }
        .ss-counter {
            font-size: 0.75rem;
            color: var(--muted-foreground);
            font-variant-numeric: tabular-nums;
        }
        .ss-close {
            position: absolute;
            top: 14px;
            right: 14px;
            background: none;
            border: none;
            color: var(--muted-foreground);
            font-size: 1rem;
            cursor: pointer;
            padding: 4px 6px;
            border-radius: 6px;
            line-height: 1;
            transition: color 0.15s, background 0.15s;
        }
        .ss-close:hover { color: var(--foreground); background: var(--muted); }

        /* ---- Slides ---- */
        .ss-slides-wrap {
            position: relative;
            overflow: hidden;
            min-height: 260px;
        }
        .ss-slide {
            display: none;
            flex-direction: column;
            gap: 12px;
            animation: ss-fadein 0.22s ease;
        }
        .ss-slide.active { display: flex; }
        @keyframes ss-fadein {
            from { opacity: 0; transform: translateX(12px); }
            to   { opacity: 1; transform: translateX(0); }
        }
        .ss-slide-title {
            font-size: 1.125rem;
            font-weight: 700;
            color: var(--foreground);
            margin: 0;
        }
        .ss-slide-desc {
            font-size: 0.875rem;
            color: var(--muted-foreground);
            margin: 0;
            line-height: 1.6;
        }
        .ss-img-wrap {
            border-radius: 10px;
            overflow: hidden;
            background: var(--muted);
            max-height: 180px;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .ss-img {
            width: 100%;
            max-height: 180px;
            object-fit: contain;
            display: block;
        }
        .ss-placeholder {
            border: 2px dashed var(--border);
            border-radius: 10px;
            background: var(--muted);
            height: 130px;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 12px;
        }
        .ss-placeholder-label {
            font-size: 0.75rem;
            color: var(--muted-foreground);
            text-align: center;
            font-style: italic;
            line-height: 1.5;
        }
        .ss-instruction {
            font-size: 0.8125rem;
            font-weight: 600;
            color: var(--foreground);
            background: var(--muted);
            border-radius: 8px;
            padding: 8px 12px;
        }

        /* ---- Modal footer ---- */
        .ss-modal-footer {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .ss-dots {
            display: flex;
            gap: 6px;
            align-items: center;
        }
        .ss-dot {
            width: 7px;
            height: 7px;
            border-radius: 50%;
            background: var(--border);
            border: none;
            cursor: pointer;
            padding: 0;
            transition: background 0.2s, transform 0.2s;
        }
        .ss-dot.active {
            background: var(--foreground);
            transform: scale(1.25);
        }
        .ss-nav {
            display: flex;
            gap: 6px;
        }
        .ss-arrow {
            background: var(--muted);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--foreground);
            font-size: 0.875rem;
            width: 34px;
            height: 34px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: background 0.15s, opacity 0.2s;
        }
        .ss-arrow:hover { background: var(--card); }

        /* ---- Access link (always visible) ---- */
        .ss-access-link {
            display: block;
            text-align: center;
            font-size: 0.75rem;
            color: var(--muted-foreground);
            text-decoration: none;
            padding: 4px;
            border-radius: 6px;
            transition: color 0.15s;
            margin-top: -8px;
        }
        .ss-access-link:hover { color: var(--foreground); }

        /* ---- Service card common styles ---- */
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
        .svc-state            { font-size: 0.8125rem; color: var(--muted-foreground); margin: 0; }
        .svc-state.svc-open   { color: var(--success); font-weight: 600; }
        .svc-state.svc-pending { color: #ca8a04; }
        .svc-submitted      { font-size: 0.8125rem; color: var(--muted-foreground); margin: 0; }
        .service-msg {
            margin-top: 8px;
            padding: 7px 11px;
            border-radius: var(--radius-md);
            font-size: 0.875rem;
            display: none;
        }
        .service-msg.ok    { display: block; background: var(--success-bg); color: var(--success); border: 1px solid var(--success-border); }
        .service-msg.error { display: block; background: var(--destructive-bg); color: var(--destructive); border: 1px solid var(--destructive-border); }
        @media (max-width: 480px) {
            .services-grid { grid-template-columns: 1fr; }
        }
"##;
