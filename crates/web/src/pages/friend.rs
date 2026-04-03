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
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 14px;
            margin-top: 20px;
        }
        .stat-box {
            background: linear-gradient(135deg, #f2f9ff 0%, #e8f7f5 100%);
            border: 1px solid rgba(13, 155, 115, 0.2);
            border-radius: 12px;
            padding: 16px;
            text-align: center;
        }
        .stat-number {
            font-size: 1.8rem;
            font-weight: 800;
            color: #0d9b73;
        }
        .stat-label {
            font-size: 0.85rem;
            color: #132331;
            margin-top: 6px;
            opacity: 0.8;
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
            <ul class="feature-list">
                <li>Consulter ton profil et tes données</li>
                <li>Accéder à l'espace membres</li>
                <li>Participer aux discussions</li>
                <li>Gérer tes préférences</li>
            </ul>
        </article>

        <article class="card">
            <h2>Mon Statut</h2>
            <p style="margin-top: 0; opacity: 0.8;">Mets à jour ton statut selon ton activité sur les apps</p>
            <div class="status-buttons">
                <button class="status-btn" id="busy-btn">🟡 Occupé (Jellyfin/Songsurf)</button>
                <button class="status-btn" id="active-btn">🟢 Actif</button>
            </div>
            <div id="status-msg" class="status-msg"></div>
        </article>

        <article class="card">
            <h2>Premium - Services intégrés</h2>
            <p style="margin-top: 0; opacity: 0.8;">Acces rapide vers tes services externes.</p>
            <div class="services">
                <div class="service-card">
                    <h3>Songsurf</h3>
                    <p>Accès au service musique.</p>
                    <button class="service-btn" data-url="https://revoli-songsurf.duckdns.org">Ouvrir Songsurf</button>
                </div>
                <div class="service-card">
                    <h3>Jellyfin</h3>
                    <p>Accès streaming media.</p>
                    <button class="service-btn" data-url="https://revoli-jellyfin.duckdns.org">Ouvrir Jellyfin</button>
                </div>
                <div class="service-card">
                    <h3>GitHub</h3>
                    <p>Accès au repo/profil GitHub.</p>
                    <button class="service-btn" data-url="https://github.com/Rev0li">Ouvrir GitHub</button>
                </div>
            </div>
        </article>

        <article class="card">
            <h2>Statistiques</h2>
            <div class="stats">
                <div class="stat-box">
                    <div class="stat-number" id="member-count">--</div>
                    <div class="stat-label">Membres actifs</div>
                </div>
                <div class="stat-box">
                    <div class="stat-number" id="signup-pending">--</div>
                    <div class="stat-label">En attente</div>
                </div>
                <div class="stat-box">
                    <div class="stat-number">24h</div>
                    <div class="stat-label">Accès disponible</div>
                </div>
            </div>
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

        // Load stats
        async function loadStats() {
            try {
                const res = await fetch('/status/all', { cache: 'no-store' });
                const data = await res.json();
                
                const usersRes = await fetch('/users', { cache: 'no-store' });
                const usersList = await usersRes.json();
                
                document.getElementById('member-count').textContent = usersList.length;
                document.getElementById('signup-pending').textContent = data.signup_requests_pending;
            } catch (_err) {
                console.log('Stats loading error');
            }
        }

        loadStats();
        setInterval(loadStats, 10000);

        // Status management
        const statusMsg = document.getElementById('status-msg');

        async function setStatus(status) {
            try {
                const endpoint = status === 'busy' ? '/status/set-busy/' + pseudo : '/status/set-active/' + pseudo;
                const res = await fetch(endpoint, { method: 'POST', cache: 'no-store' });
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

        document.querySelectorAll('.service-btn').forEach((button) => {
            button.addEventListener('click', () => {
                const baseUrl = button.getAttribute('data-url');
                if (!baseUrl) return;
                window.location.href = baseUrl;
            });
        });

        document.getElementById('logout-btn').addEventListener('click', logout);
        document.getElementById('busy-btn').addEventListener('click', () => setStatus('busy'));
        document.getElementById('active-btn').addEventListener('click', () => setStatus('active'));
    </script>
</body>
</html>
"##,
    )
}
