use axum::response::Html;

pub async fn friend() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Home - rev0auth</title>
    <style>
        body {
            margin: 0;
            font-family: "Space Grotesk", "Avenir Next", sans-serif;
            color: #132331;
            background:
                radial-gradient(circle at 15% 10%, #d9f0ff, transparent 40%),
                radial-gradient(circle at 85% 80%, #ffe7ca, transparent 40%),
                linear-gradient(135deg, #eef8ff 0%, #e6f7ee 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            padding: 28px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 32px;
            gap: 16px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.8rem, 5vw, 2.5rem); }
        .header-meta {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .header-status {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .header-actions {
            display: flex;
            flex-direction: column;
            align-items: flex-end;
            gap: 10px;
        }
        .header-avatar {
            width: 62px;
            height: 62px;
            border-radius: 50%;
            border: 2px solid rgba(19, 35, 49, 0.16);
            background: #f3f7fa;
            object-fit: cover;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.16);
        }
        .header-action-row {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            justify-content: flex-end;
        }
        .logout-btn {
            padding: 8px 14px;
            background: rgba(255, 107, 59, 0.9);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .logout-btn:hover { background: rgba(239, 78, 36, 1); }
        .profile-btn {
            padding: 8px 14px;
            background: rgba(13, 155, 115, 0.92);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .profile-btn:hover { background: rgba(10, 132, 98, 1); }
        .chat-card {
            margin-bottom: 20px;
            border: 1px solid rgba(19, 35, 49, 0.14);
            border-radius: 14px;
            background: rgba(255, 255, 255, 0.9);
            padding: 16px;
            box-shadow: 0 12px 24px rgba(19, 35, 49, 0.09);
        }
        .chat-card h2 {
            margin: 0 0 10px;
            font-size: 1.1rem;
        }
        .chat-card label {
            display: block;
            margin: 10px 0 6px;
            font-weight: 700;
        }
        .chat-card input,
        .chat-card textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        .chat-card textarea {
            min-height: 110px;
            resize: vertical;
        }
        .chat-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .chat-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
            display: block;
        }
        .chat-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
            display: block;
        }
        .chat-history {
            margin-top: 12px;
            max-height: 300px;
            overflow: auto;
            display: grid;
            gap: 8px;
            padding-right: 4px;
        }
        .chat-bubble {
            max-width: 86%;
            padding: 9px 11px;
            border-radius: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            white-space: pre-wrap;
            line-height: 1.4;
            font-size: 0.9rem;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.07);
        }
        .chat-bubble.mine {
            justify-self: end;
            background: #e8fff5;
            border-color: #b3ecd1;
        }
        .chat-bubble.theirs {
            justify-self: start;
            background: #fff;
        }
        .chat-meta {
            display: block;
            margin-top: 4px;
            font-size: 0.78rem;
            opacity: 0.72;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 28px;
            box-shadow: 0 16px 45px rgba(19, 35, 49, 0.12);
            margin-bottom: 18px;
        }
        h2 { margin: 0 0 14px; font-size: 1.4rem; }
        .greeting { font-size: 1.1rem; line-height: 1.6; margin-bottom: 20px; }
        .greeting strong { color: #0d9b73; }
        .feature-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        .feature-list li {
            padding: 10px 0;
            border-bottom: 1px solid rgba(19, 35, 49, 0.08);
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .feature-list li:last-child { border-bottom: 0; }
        .feature-list li:before {
            content: "→";
            color: #ff6f3f;
            font-weight: 700;
        }
        .status-buttons {
            display: flex;
            gap: 10px;
            margin-top: 16px;
            flex-wrap: wrap;
        }
        .status-btn {
            padding: 8px 14px;
            border: 1px solid rgba(13, 155, 115, 0.3);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.05);
            color: #0d9b73;
            font-weight: 600;
            cursor: pointer;
            font-size: 0.9rem;
        }
        .status-btn:hover {
            background: rgba(13, 155, 115, 0.15);
        }
        .status-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .status-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .status-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
        .remark-panel {
            display: none;
            margin-top: 14px;
            padding: 12px;
            border: 1px dashed rgba(19, 35, 49, 0.2);
            border-radius: 12px;
            background: rgba(243, 247, 250, 0.8);
        }
        .services {
            display: flex;
            flex-direction: column;
            gap: 14px;
            margin-top: 14px;
        }
        .service-card {
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 12px;
            background: #fff;
            padding: 14px;
            width: 100%;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .service-media {
            width: 100%;
            aspect-ratio: 16 / 9;
            height: auto;
            object-fit: cover;
            border-radius: 10px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            background: #f3f7fa;
        }
        .service-card h3 {
            margin: 0 0 6px;
            font-size: 1rem;
        }
        .service-card p {
            margin: 0 0 10px;
            font-size: 0.9rem;
            opacity: 0.8;
        }
        .service-btn {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.08);
            color: #0d9b73;
            font-weight: 700;
            padding: 9px 10px;
            cursor: pointer;
        }
        .service-btn:hover {
            background: rgba(13, 155, 115, 0.18);
        }
        .service-btn.locked {
            background: #f3f7fa;
            color: #4b5f71;
            border-color: rgba(19, 35, 49, 0.16);
        }
        .service-state {
            font-size: 0.84rem;
            margin: 8px 0;
            opacity: 0.84;
        }
        .service-input {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 8px;
            box-sizing: border-box;
            margin-bottom: 8px;
        }
        .service-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .service-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .service-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
        @media (max-width: 900px) {
            .container {
                padding: 18px;
            }
            .header {
                flex-direction: column;
                align-items: stretch;
            }
            .header-actions {
                align-items: flex-start;
            }
            .header-action-row {
                justify-content: flex-start;
            }
            .service-card {
                aspect-ratio: 1 / 1;
                overflow: hidden;
            }
            .service-media {
                aspect-ratio: 1 / 1;
            }
        }
        .onboarding-modal {
            position: fixed;
            inset: 0;
            background: rgba(10, 20, 30, 0.6);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 90;
            padding: 16px;
        }
        .onboarding-card {
            width: 100%;
            max-width: 520px;
            background: #fff;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            padding: 16px;
        }
        .onboarding-msg {
            margin-top: 10px;
            border-radius: 8px;
            padding: 8px;
            font-size: 0.9rem;
            display: none;
        }
        .onboarding-msg.ok {
            display: block;
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .onboarding-msg.error {
            display: block;
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
    </style>
</head>
<body>
    <div id="onboarding-modal" class="onboarding-modal">
        <div class="onboarding-card">
            <h2>Premiere connexion</h2>
            <p style="margin-top:0;opacity:.85;">Change ton mot de passe temporaire avant de continuer.</p>

            <label for="onboarding-new-password" style="display:block;font-weight:700;margin-top:10px;">Nouveau mot de passe</label>
            <input id="onboarding-new-password" type="password" placeholder="nouveau mot de passe" style="width:100%;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;" />

            <label for="onboarding-confirm-password" style="display:block;font-weight:700;margin-top:10px;">Confirmer le nouveau mot de passe</label>
            <input id="onboarding-confirm-password" type="password" placeholder="retape le nouveau mot de passe" style="width:100%;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;" />

            <label for="onboarding-message" style="display:block;font-weight:700;margin-top:10px;">Message de presentation</label>
            <textarea id="onboarding-message" placeholder="Presente-toi rapidement..." style="width:100%;min-height:110px;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;"></textarea>

            <div class="actions" style="margin-top:10px;">
                <button id="onboarding-submit" class="profile-btn">Valider</button>
            </div>
            <div id="onboarding-msg" class="onboarding-msg"></div>
        </div>
    </div>

    <main class="container">
        <div class="header">
            <div class="header-meta">
                <h1>Bienvenue <span id="welcome-pseudo">!</span></h1>
                <div class="header-status">
                    <span style="font-weight:700;opacity:.8;">Humeur rapide:</span>
                    <button class="status-btn" id="happy-btn">😀 Content</button>
                    <button class="status-btn" id="meh-btn">😐 Bof</button>
                    <button class="status-btn" id="question-btn">❓ Besoin d'aide</button>
                </div>
                <div id="status-msg" class="status-msg"></div>
            </div>
            <div class="header-actions">
                <img id="header-avatar" class="header-avatar" alt="Photo de profil" />
                <div class="header-action-row">
                    <a class="profile-btn" href="/members/profile">Mon profil</a>
                    <button class="logout-btn" id="logout-btn">Déconnexion</button>
                </div>
            </div>
        </div>

        <article class="card">
            <h2>Presentation du projet</h2>
            <p class="greeting">
                Bienvenue <strong id="pseudo-display">ami</strong>.
                <br>rev0auth est un projet self-host Rust pour apprendre, construire et partager un systeme auth clair.
            </p>
            <p class="greeting">
                Si le projet t'aide, tu peux soutenir son evolution via un don.
                <br>Chaque contribution aide a maintenir l'infra, les tests et la documentation publique.
            </p>
            <div class="actions">
                <a class="profile-btn" href="https://github.com/sponsors/Rev0li" target="_blank" rel="noopener noreferrer">Faire un don</a>
            </div>
        </article>

        <article class="card">
            <h2>Services intégrés</h2>
            <p style="margin-top: 0; opacity: 0.8;">Par defaut tu n'as acces a rien. Demande l'acces service par service.</p>
            <div class="services">
                <div class="service-card">
                    <h3>Songsurf</h3>
                    <img class="service-media" src="https://images.unsplash.com/photo-1516280440614-37939bbacd81?auto=format&fit=crop&w=900&q=80" alt="Songsurf - musique" />
                    <p>Acces au service musique.</p>
                    <div class="service-state" id="songsurf-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="songsurf-btn">Demander acces Songsurf</button>
                </div>
                <div class="service-card">
                    <h3>Jellyfin</h3>
                    <img class="service-media" src="https://images.unsplash.com/photo-1489599849927-2ee91cede3ba?auto=format&fit=crop&w=900&q=80" alt="Jellyfin - streaming video" />
                    <p>Acces streaming media.</p>
                    <div class="service-state" id="jellyfin-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="jellyfin-btn">Demander acces Jellyfin</button>
                </div>
                <div class="service-card">
                    <h3>GitHub</h3>
                    <img class="service-media" src="data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 900 400'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0' x2='1' y1='0' y2='1'%3E%3Cstop stop-color='%23142331'/%3E%3Cstop offset='1' stop-color='%230d9b73'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='900' height='400' rx='26' fill='url(%23g)'/%3E%3Ccircle cx='165' cy='160' r='88' fill='%23ffffff' fill-opacity='.12'/%3E%3Ccircle cx='165' cy='145' r='48' fill='%23ffffff'/%3E%3Cpath d='M95 275c24-42 57-61 70-61s46 19 70 61' fill='%23ffffff'/%3E%3Ctext x='305' y='175' fill='%23ffffff' font-family='Space Grotesk, Arial, sans-serif' font-size='62' font-weight='700'%3EGitHub Profile%3C/text%3E%3Ctext x='305' y='238' fill='%23d9f0ff' font-family='Space Grotesk, Arial, sans-serif' font-size='28'%3EAcces conditionne par star + demande%3C/text%3E%3C/svg%3E" alt="GitHub profile" />
                    <p>Acces conditionne: compte cree + star sur ce projet.</p>
                    <div class="service-state" id="github-state">Etat: verrouille</div>
                    <input id="github-username" class="service-input" placeholder="Ton username GitHub" />
                    <button class="service-btn locked" id="github-btn">J'ai mis une star, demander acces GitHub</button>
                </div>
            </div>
            <div id="service-msg" class="service-msg"></div>

        </article>

        <article class="chat-card">
            <h2>Start message with me</h2>
            <p style="margin:0 0 8px;opacity:.82;">Conversation directe avec l'admin.</p>

            <div id="chat-history" class="chat-history">Chargement de l'historique...</div>

            <label for="chat-subject">Sujet</label>
            <input id="chat-subject" placeholder="Sujet" />

            <label for="chat-body">Message</label>
            <textarea id="chat-body" placeholder="Ton message..."></textarea>

            <div class="actions" style="margin-top:10px;">
                <button id="chat-send-btn" class="profile-btn">Envoyer message</button>
            </div>
            <div id="chat-msg" class="chat-msg"></div>
        </article>

    </main>

    <script>
        // Get logged-in pseudo from localStorage
        const pseudo = localStorage.getItem('logged_pseudo');

        if (!pseudo) {
            window.location.href = '/';
        } else {
            document.getElementById('welcome-pseudo').textContent = pseudo;
            document.getElementById('pseudo-display').textContent = pseudo;
        }

        const accessState = {
            github: false,
            jellyfin: false,
            songsurf: false,
            requestGithub: false,
            requestJellyfin: false,
            requestSongsurf: false
        };
        const needsOnboarding = localStorage.getItem('needs_onboarding') === '1';

        function bindEnterToClick(inputId, buttonId) {
            const input = document.getElementById(inputId);
            const button = document.getElementById(buttonId);
            if (!input || !button) return;
            input.addEventListener('keydown', (event) => {
                if (event.key === 'Enter') {
                    event.preventDefault();
                    button.click();
                }
            });
        }

        bindEnterToClick('onboarding-new-password', 'onboarding-submit');
        bindEnterToClick('onboarding-confirm-password', 'onboarding-submit');

        function setOnboardingMsg(ok, message) {
            const el = document.getElementById('onboarding-msg');
            el.className = 'onboarding-msg ' + (ok ? 'ok' : 'error');
            el.textContent = message;
        }

        function openOnboardingModal() {
            const modal = document.getElementById('onboarding-modal');
            modal.style.display = 'flex';
        }

        function closeOnboardingModal() {
            const modal = document.getElementById('onboarding-modal');
            modal.style.display = 'none';
        }

        async function submitOnboarding() {
            const newPassword = document.getElementById('onboarding-new-password').value.trim();
            const confirmPassword = document.getElementById('onboarding-confirm-password').value.trim();
            const message = document.getElementById('onboarding-message').value.trim();

            if (!newPassword || !confirmPassword) {
                setOnboardingMsg(false, 'Entre le nouveau mot de passe et sa confirmation.');
                return;
            }

            if (newPassword !== confirmPassword) {
                setOnboardingMsg(false, 'Les deux mots de passe ne correspondent pas.');
                return;
            }

            try {
                const profileDataRes = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
                const profileData = await profileDataRes.json();

                const passwordRes = await fetch('/members/password', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo,
                        new_password: newPassword
                    })
                });
                const passwordData = await passwordRes.json();
                if (!passwordData.ok) {
                    setOnboardingMsg(false, passwordData.message || 'Impossible de changer le mot de passe.');
                    return;
                }

                const profileRes = await fetch('/members/profile/data', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo,
                        bio: profileData && typeof profileData.bio === 'string' ? profileData.bio : '',
                        commentary: message || null
                    })
                });
                const profileUpdate = await profileRes.json();
                if (!profileUpdate.ok) {
                    setOnboardingMsg(false, profileUpdate.message || 'Mot de passe change mais message non enregistre.');
                    return;
                }

                localStorage.removeItem('needs_onboarding');
                setOnboardingMsg(true, 'Onboarding valide. Bienvenue.');
                setTimeout(closeOnboardingModal, 450);
            } catch (err) {
                setOnboardingMsg(false, 'Erreur: ' + err.message);
            }
        }

        // Logout function
        async function logout() {
            if (!confirm('Confirmer la déconnexion ?')) return;
            
            try {
                // Mark user as inactive
                await fetch('/status/set-inactive/' + pseudo, { method: 'POST', cache: 'no-store' });
            } catch (err) {
                console.log('Logout error:', err);
            }
            
            // Clear session and redirect
            localStorage.removeItem('logged_pseudo');
            window.location.href = '/';
        }

        // Status management
        const statusMsg = document.getElementById('status-msg');
        const serviceMsg = document.getElementById('service-msg');
        const headerAvatar = document.getElementById('header-avatar');

        const chatSubjectInput = document.getElementById('chat-subject');
        const chatBodyInput = document.getElementById('chat-body');
        const chatMsg = document.getElementById('chat-msg');
        const chatHistory = document.getElementById('chat-history');

        function fallbackAvatar(pseudoValue) {
            const first = (pseudoValue || '?').charAt(0).toUpperCase() || '?';
            const svg = `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
                <defs>
                    <linearGradient id='g' x1='0' y1='0' x2='1' y2='1'>
                        <stop offset='0%' stop-color='#0d9b73'/>
                        <stop offset='100%' stop-color='#132331'/>
                    </linearGradient>
                </defs>
                <rect width='100' height='100' rx='50' fill='url(#g)'/>
                <text x='50' y='61' text-anchor='middle' font-size='44' font-family='Space Grotesk, Arial, sans-serif' fill='#ffffff'>${first}</text>
            </svg>`;
            return 'data:image/svg+xml;utf8,' + encodeURIComponent(svg);
        }

        function setHeaderAvatarSrc(hasAvatar) {
            if (hasAvatar) {
                headerAvatar.src = '/members/avatar/' + encodeURIComponent(pseudo) + '?t=' + Date.now();
                headerAvatar.onerror = () => {
                    headerAvatar.onerror = null;
                    headerAvatar.src = fallbackAvatar(pseudo);
                };
                return;
            }
            headerAvatar.src = fallbackAvatar(pseudo);
        }

        function setServiceMsg(ok, message) {
            serviceMsg.className = 'service-msg ' + (ok ? 'ok' : 'error');
            serviceMsg.textContent = message;
            serviceMsg.style.display = 'block';
        }

        function renderServiceButtons() {
            const songsurfBtn = document.getElementById('songsurf-btn');
            const jellyfinBtn = document.getElementById('jellyfin-btn');
            const githubBtn = document.getElementById('github-btn');

            const songsurfState = document.getElementById('songsurf-state');
            const jellyfinState = document.getElementById('jellyfin-state');
            const githubState = document.getElementById('github-state');

            if (accessState.songsurf) {
                songsurfState.textContent = 'Etat: ACCES OUVERT';
                songsurfBtn.textContent = 'Ouvrir Songsurf';
                songsurfBtn.classList.remove('locked');
            } else if (accessState.requestSongsurf) {
                songsurfState.textContent = 'Etat: demande envoyee, en attente admin';
                songsurfBtn.textContent = 'Demande Songsurf envoyee';
                songsurfBtn.classList.add('locked');
            } else {
                songsurfState.textContent = 'Etat: verrouille';
                songsurfBtn.textContent = 'Demander acces Songsurf';
                songsurfBtn.classList.add('locked');
            }

            if (accessState.jellyfin) {
                jellyfinState.textContent = 'Etat: ACCES OUVERT';
                jellyfinBtn.textContent = 'Ouvrir Jellyfin';
                jellyfinBtn.classList.remove('locked');
            } else if (accessState.requestJellyfin) {
                jellyfinState.textContent = 'Etat: demande envoyee, en attente admin';
                jellyfinBtn.textContent = 'Demande Jellyfin envoyee';
                jellyfinBtn.classList.add('locked');
            } else {
                jellyfinState.textContent = 'Etat: verrouille';
                jellyfinBtn.textContent = 'Demander acces Jellyfin';
                jellyfinBtn.classList.add('locked');
            }

            if (accessState.github) {
                githubState.textContent = 'Etat: ACCES OUVERT';
                githubBtn.textContent = 'Ouvrir GitHub';
                githubBtn.classList.remove('locked');
            } else if (accessState.requestGithub) {
                githubState.textContent = 'Etat: demande envoyee, en attente admin';
                githubBtn.textContent = 'Demande GitHub envoyee';
                githubBtn.classList.add('locked');
            } else {
                githubState.textContent = 'Etat: verrouille';
                githubBtn.textContent = 'J\'ai mis une star, demander acces GitHub';
                githubBtn.classList.add('locked');
            }
        }

        async function loadMemberAccessState() {
            try {
                const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
                const data = await res.json();
                accessState.github = !!data.access_github;
                accessState.jellyfin = !!data.access_jellyfin;
                accessState.songsurf = !!data.access_songsurf;
                accessState.requestGithub = !!data.request_github;
                accessState.requestJellyfin = !!data.request_jellyfin;
                accessState.requestSongsurf = !!data.request_songsurf;
                setHeaderAvatarSrc(!!data.avatar_present);

                if (data.github_username) {
                    document.getElementById('github-username').value = data.github_username;
                }
                renderServiceButtons();
            } catch (err) {
                setHeaderAvatarSrc(false);
                setServiceMsg(false, 'Impossible de charger l\'etat des acces: ' + err.message);
            }
        }

        async function requestService(service, extraPayload) {
            try {
                const res = await fetch('/members/access/request', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify(Object.assign({ pseudo, service }, extraPayload || {}))
                });
                const data = await res.json();
                setServiceMsg(!!data.ok, data.message || 'Reponse recue.');
                if (data.ok) {
                    await loadMemberAccessState();
                }
            } catch (err) {
                setServiceMsg(false, 'Erreur: ' + err.message);
            }
        }

        async function setStatus(status) {
            try {
                const res = await fetch('/members/status', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo,
                        status
                    })
                });
                const data = await res.json();

                statusMsg.className = 'status-msg ' + (data.ok ? 'ok' : 'error');
                statusMsg.textContent = data.message;
                statusMsg.style.display = 'block';

                setTimeout(() => {
                    statusMsg.style.display = 'none';
                }, 3000);
                return data.ok;
            } catch (err) {
                statusMsg.className = 'status-msg error';
                statusMsg.textContent = 'Erreur: ' + err.message;
                statusMsg.style.display = 'block';
                return false;
            }
        }

        function setChatMsg(ok, message) {
            chatMsg.className = 'chat-msg ' + (ok ? 'ok' : 'error');
            chatMsg.textContent = message;
        }

        function escapeHtml(value) {
            return String(value || '')
                .replace(/&/g, '&amp;')
                .replace(/</g, '&lt;')
                .replace(/>/g, '&gt;')
                .replace(/"/g, '&quot;')
                .replace(/'/g, '&#39;');
        }

        async function loadChatHistory() {
            if (!chatHistory) return;
            try {
                const [inboxRes, sentRes] = await Promise.all([
                    fetch('/members/messages/inbox?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' }),
                    fetch('/members/messages/sent?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' })
                ]);
                const inbox = await inboxRes.json();
                const sent = await sentRes.json();
                const merged = [];
                if (Array.isArray(inbox)) merged.push(...inbox.map((m) => Object.assign({}, m, { mine: false })));
                if (Array.isArray(sent)) merged.push(...sent.map((m) => Object.assign({}, m, { mine: true })));

                merged.sort((a, b) => {
                    if (a.created_at_epoch === b.created_at_epoch) return a.id - b.id;
                    return a.created_at_epoch - b.created_at_epoch;
                });

                const unreadInbound = merged.filter((m) => !m.mine && !m.is_read);
                if (unreadInbound.length > 0) {
                    await Promise.all(unreadInbound.map((m) =>
                        fetch('/members/messages/' + m.id + '/read', {
                            method: 'POST',
                            headers: { 'content-type': 'application/json' },
                            body: JSON.stringify({ pseudo })
                        })
                    ));
                }

                if (merged.length === 0) {
                    chatHistory.textContent = 'Aucun message pour le moment.';
                    return;
                }

                chatHistory.innerHTML = merged.map((m) => {
                    const dt = new Date(m.created_at_epoch * 1000).toLocaleString();
                    const who = m.mine ? 'To admin' : 'Admin';
                    return '<div class="chat-bubble ' + (m.mine ? 'mine' : 'theirs') + '">'
                        + '<strong>' + escapeHtml(m.subject || 'Sans sujet') + '</strong><br>'
                        + escapeHtml(m.body || '')
                        + '<span class="chat-meta">' + who + ' • ' + dt + '</span>'
                        + '</div>';
                }).join('');
                chatHistory.scrollTop = chatHistory.scrollHeight;
            } catch (err) {
                chatHistory.textContent = 'Historique indisponible: ' + err.message;
            }
        }

        async function sendQuickChat() {
            const subject = chatSubjectInput.value.trim();
            const body = chatBodyInput.value.trim();

            if (!subject || !body) {
                setChatMsg(false, 'Remplis sujet et message.');
                return;
            }

            try {
                const res = await fetch('/members/messages/send', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        from_pseudo: pseudo,
                        subject,
                        body
                    })
                });
                const data = await res.json();
                setChatMsg(!!data.ok, data.message || 'Message envoye.');
                if (data.ok) {
                    chatSubjectInput.value = '';
                    chatBodyInput.value = '';
                    await loadChatHistory();
                }
            } catch (err) {
                setChatMsg(false, 'Erreur: ' + err.message);
            }
        }

        document.getElementById('songsurf-btn').addEventListener('click', async () => {
            if (accessState.songsurf) {
                window.location.href = 'https://revoli-songsurf.duckdns.org';
                return;
            }
            await requestService('songsurf');
        });

        document.getElementById('jellyfin-btn').addEventListener('click', async () => {
            if (accessState.jellyfin) {
                window.location.href = 'https://revoli-jellyfin.duckdns.org';
                return;
            }
            await requestService('jellyfin');
        });

        document.getElementById('github-btn').addEventListener('click', async () => {
            if (accessState.github) {
                window.location.href = 'https://github.com/Rev0li';
                return;
            }

            const githubUsername = document.getElementById('github-username').value.trim();
            if (!githubUsername) {
                setServiceMsg(false, 'Renseigne ton username GitHub avant la demande.');
                return;
            }

            await requestService('github', {
                github_username: githubUsername,
                starred: true
            });
        });

        document.getElementById('logout-btn').addEventListener('click', logout);
        document.getElementById('chat-send-btn').addEventListener('click', sendQuickChat);
        document.querySelector('a[href="/members/profile"]').addEventListener('click', (event) => {
            if (localStorage.getItem('needs_onboarding') === '1') {
                event.preventDefault();
                openOnboardingModal();
                setOnboardingMsg(false, 'Termine d\'abord le changement de mot de passe initial.');
            }
        });
        document.getElementById('happy-btn').addEventListener('click', async () => {
            await setStatus('content');
        });
        document.getElementById('meh-btn').addEventListener('click', async () => {
            await setStatus('bof');
        });
        document.getElementById('question-btn').addEventListener('click', async () => {
            await setStatus('question');
        });
        document.getElementById('onboarding-submit').addEventListener('click', submitOnboarding);
        setHeaderAvatarSrc(false);
        if (needsOnboarding) {
            openOnboardingModal();
        }
        loadMemberAccessState();
        loadChatHistory();
        setInterval(loadChatHistory, 8000);
    </script>
</body>
</html>
"##,
    )
}
