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
            align-items: center;
            margin-bottom: 32px;
            gap: 16px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.8rem, 5vw, 2.5rem); }
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
        .services {
            display: grid;
            grid-template-columns: repeat(3, minmax(180px, 220px));
            justify-content: center;
            gap: 12px;
            margin-top: 14px;
        }
        .service-card {
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 12px;
            background: #fff;
            padding: 14px;
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
            .services {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    <main class="container">
        <div class="header">
            <h1>Bienvenue <span id="welcome-pseudo">!</span></h1>
            <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                <a class="profile-btn" href="/members/profile">Mon profil</a>
                <button class="logout-btn" id="logout-btn">Déconnexion</button>
            </div>
        </div>

        <article class="card">
            <h2>Espace personnel</h2>
            <p class="greeting">
                Salut <strong id="pseudo-display">ami</strong> ! 
                <br>Tu es maintenant connecté à la plateforme. Bienvenue dans la communauté rev0auth !
            </p>
            <p class="greeting">
                This page is your member home.
                <br>Tu peux acceder a tes services, mettre a jour ton profil et partager ton feedback.
            </p>
            <ul class="feature-list">
                <li>Consulter ton profil et tes données</li>
                <li>Accéder à l'espace membres</li>
                <li>Participer aux discussions</li>
                <li>Gérer tes préférences</li>
            </ul>
        </article>

        <article class="card">
            <h2>Mon humeur / My status</h2>
            <p style="margin-top: 0; opacity: 0.8;">Choisis un smiley puis laisse un commentaire si tu veux une amelioration.</p>
            <div class="status-buttons">
                <button class="status-btn" id="happy-btn">😀 Content</button>
                <button class="status-btn" id="meh-btn">😐 Bof</button>
                <button class="status-btn" id="question-btn">❓ Question / Amelioration</button>
            </div>
            <label for="commentary" style="display:block;margin-top:12px;font-weight:700;">Commentary</label>
            <textarea id="commentary" placeholder="Une idee, une question, une amelioration..." style="width:100%;min-height:86px;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;"></textarea>
            <div id="status-msg" class="status-msg"></div>
        </article>

        <article class="card">
            <h2>Premium - Services intégrés</h2>
            <p style="margin-top: 0; opacity: 0.8;">Par defaut tu n'as acces a rien. Demande l'acces service par service.</p>
            <div class="services">
                <div class="service-card">
                    <h3>Songsurf</h3>
                    <p>Acces au service musique.</p>
                    <div class="service-state" id="songsurf-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="songsurf-btn">Demander acces Songsurf</button>
                </div>
                <div class="service-card">
                    <h3>Jellyfin</h3>
                    <p>Acces streaming media.</p>
                    <div class="service-state" id="jellyfin-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="jellyfin-btn">Demander acces Jellyfin</button>
                </div>
                <div class="service-card">
                    <h3>GitHub</h3>
                    <p>Acces conditionne: compte cree + star sur ce projet.</p>
                    <div class="service-state" id="github-state">Etat: verrouille</div>
                    <input id="github-username" class="service-input" placeholder="Ton username GitHub" />
                    <button class="service-btn locked" id="github-btn">J'ai mis une star, demander acces GitHub</button>
                </div>
            </div>
            <div id="service-msg" class="service-msg"></div>
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
        const commentaryInput = document.getElementById('commentary');
        const serviceMsg = document.getElementById('service-msg');

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

                if (data.github_username) {
                    document.getElementById('github-username').value = data.github_username;
                }
                renderServiceButtons();
            } catch (err) {
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
                        status,
                        commentary: commentaryInput ? commentaryInput.value : ''
                    })
                });
                const data = await res.json();
                
                statusMsg.className = 'status-msg ' + (data.ok ? 'ok' : 'error');
                statusMsg.textContent = data.message;
                statusMsg.style.display = 'block';
                
                setTimeout(() => {
                    statusMsg.style.display = 'none';
                }, 3000);
            } catch (err) {
                statusMsg.className = 'status-msg error';
                statusMsg.textContent = 'Erreur: ' + err.message;
                statusMsg.style.display = 'block';
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
        document.getElementById('happy-btn').addEventListener('click', () => setStatus('content'));
        document.getElementById('meh-btn').addEventListener('click', () => setStatus('bof'));
        document.getElementById('question-btn').addEventListener('click', () => setStatus('question'));
        loadMemberAccessState();
    </script>
</body>
</html>
"##,
    )
}
