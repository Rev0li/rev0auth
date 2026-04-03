use axum::response::Html;

pub async fn profile() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Profil membre - rev0auth</title>
    <style>
        body {
            margin: 0;
            font-family: "Space Grotesk", "Avenir Next", sans-serif;
            color: #132331;
            background:
                radial-gradient(circle at 10% 5%, #ffe7cd, transparent 35%),
                radial-gradient(circle at 90% 0%, #d9f0ff, transparent 40%),
                linear-gradient(145deg, #eef7ff, #e8f8ef);
            min-height: 100vh;
        }
        .page {
            max-width: 760px;
            margin: 0 auto;
            padding: 24px;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 18px;
            padding: 20px;
            box-shadow: 0 12px 34px rgba(19, 35, 49, 0.14);
            margin-bottom: 14px;
        }
        h1 { margin-top: 0; }
        label { display: block; font-weight: 700; margin: 10px 0 6px; }
        input, textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        textarea { min-height: 90px; resize: vertical; }
        .actions {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 12px;
        }
        button, a.btn {
            border: 0;
            border-radius: 10px;
            padding: 9px 13px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
        }
        .primary { background: linear-gradient(120deg, #ff6b3b, #ef4e24); color: #fff; }
        .secondary { background: #f2f9ff; color: #132331; border: 1px solid rgba(19, 35, 49, 0.15); }
        .msg {
            margin-top: 10px;
            font-size: 0.9rem;
            border-radius: 8px;
            padding: 8px;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; color: #0d9b73; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; color: #ef4e24; }
        .meta { font-size: 0.9rem; opacity: 0.82; }
        .admin-note {
            margin-top: 10px;
            padding: 9px 10px;
            border-radius: 10px;
            border: 1px solid #f6d08a;
            background: #fff9ea;
            color: #6d4b00;
            font-size: 0.88rem;
            display: none;
        }
        .admin-nav {
            margin-top: 10px;
            display: none;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Profil membre</h1>
            <p class="meta">Ici tu retrouves toutes tes infos. You can update or delete your account anytime.</p>
            <div id="admin-note" class="admin-note">Mode admin actif: tu modifies le profil complet de cet utilisateur.</div>
            <div id="admin-nav" class="admin-nav">
                <button id="prev-user" class="secondary">User precedent</button>
                <button id="next-user" class="secondary">User suivant</button>
                <span id="admin-nav-meta" class="meta"></span>
            </div>
            <div class="actions">
                <a class="btn secondary" id="back-link" href="/members/dashboard">Retour dashboard</a>
            </div>
        </article>

        <article class="card">
            <h2>Infos compte</h2>
            <p class="meta"><strong>Pseudo:</strong> <span id="info-pseudo">--</span></p>
            <p class="meta"><strong>Role:</strong> <span id="info-role">--</span></p>
            <p class="meta"><strong>Status:</strong> <span id="info-status">--</span></p>
            <p class="meta"><strong>Created at:</strong> <span id="info-created">--</span></p>
            <p class="meta"><strong>Avatar:</strong> <span id="info-avatar">--</span></p>
        </article>

        <article class="card">
            <h2>Profil editable</h2>
            <label for="bio">Presentation</label>
            <textarea id="bio" placeholder="Qui es-tu, ce que tu fais, ce que tu veux partager..."></textarea>

            <label for="commentary">Commentary</label>
            <textarea id="commentary" placeholder="Question, idee, demande d'amelioration..."></textarea>

            <div class="actions">
                <button id="save-profile" class="primary">Sauver profil</button>
            </div>
            <div id="profile-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Avatar</h2>
            <label for="avatar">Image</label>
            <input id="avatar" type="file" accept="image/*" />
            <div class="actions">
                <button id="upload-avatar" class="primary">Uploader avatar</button>
            </div>
            <div id="avatar-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Mot de passe</h2>
            <label for="current-password">Mot de passe actuel</label>
            <input id="current-password" type="password" placeholder="Actuel" />

            <label for="new-password">Nouveau mot de passe</label>
            <input id="new-password" type="password" placeholder="Nouveau" />

            <div class="actions">
                <button id="save-password" class="primary">Mettre a jour le mot de passe</button>
            </div>
            <div id="password-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Danger zone</h2>
            <p class="meta">Tu peux supprimer ton compte a tout moment. Cette action est irreversible.</p>
            <div class="actions">
                <button id="delete-account" class="secondary" style="border-color:#ef4e24;color:#ef4e24;">Supprimer mon compte</button>
            </div>
            <div id="delete-msg" class="msg"></div>
        </article>
    </main>

    <script>
        const params = new URLSearchParams(window.location.search);
        const adminMode = params.get('admin') === '1';
        const queryPseudo = (params.get('pseudo') || '').trim();
        const localPseudo = localStorage.getItem('logged_pseudo');
        const pseudo = adminMode ? (queryPseudo || localPseudo) : localPseudo;
        let currentPseudo = pseudo;
        let adminUsers = [];

        if (!pseudo) {
            window.location.href = '/';
        }

        if (adminMode) {
            const note = document.getElementById('admin-note');
            note.style.display = 'block';
            document.getElementById('admin-nav').style.display = 'flex';
            document.getElementById('back-link').setAttribute('href', '/dashboard');
        }

        async function ensureAdminSession() {
            if (!adminMode) return true;

            try {
                const res = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                if (res.ok) return true;
            } catch (_err) {
                // Ask password below as fallback.
            }

            const password = window.prompt('Session admin expiree. Entre le mot de passe admin:');
            if (!password) return false;

            try {
                const loginRes = await fetch('/japprends/login', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ password })
                });
                if (!loginRes.ok) return false;

                const checkRes = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                return checkRes.ok;
            } catch (_err) {
                return false;
            }
        }

        function setMsg(id, ok, text) {
            const el = document.getElementById(id);
            el.className = 'msg ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        async function loadProfile() {
            try {
                const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
                const data = await res.json();
                if (data.ok && typeof data.bio === 'string') {
                    document.getElementById('info-pseudo').textContent = data.pseudo || '--';
                    document.getElementById('info-role').textContent = data.role || '--';
                    document.getElementById('info-status').textContent = data.status || '--';
                    document.getElementById('info-created').textContent = data.created_at_epoch ? new Date(data.created_at_epoch * 1000).toLocaleString() : '--';
                    document.getElementById('info-avatar').textContent = data.avatar_filename || 'none';
                    document.getElementById('bio').value = data.bio;
                    document.getElementById('commentary').value = data.commentary || '';
                }
            } catch (_err) {
                setMsg('profile-msg', false, 'Impossible de charger le profil.');
            }
        }

        function updateAdminNavMeta() {
            if (!adminMode) return;
            const idx = adminUsers.findIndex((p) => p.toLowerCase() === currentPseudo.toLowerCase());
            const total = adminUsers.length;
            const meta = document.getElementById('admin-nav-meta');
            if (idx >= 0 && total > 0) {
                meta.textContent = 'User ' + (idx + 1) + ' / ' + total;
            } else {
                meta.textContent = 'User -- / --';
            }
        }

        async function loadAdminUsersNavigator() {
            if (!adminMode) return;
            try {
                const res = await fetch('/users', { cache: 'no-store' });
                const list = await res.json();
                adminUsers = Array.isArray(list) ? list.map((u) => u.pseudo) : [];
                if (adminUsers.length > 0 && !adminUsers.some((p) => p.toLowerCase() === currentPseudo.toLowerCase())) {
                    currentPseudo = adminUsers[0];
                }
                updateAdminNavMeta();
            } catch (_err) {
                document.getElementById('admin-nav-meta').textContent = 'Navigation users indisponible';
            }
        }

        function goToAdminUser(offset) {
            if (!adminMode || adminUsers.length === 0) return;
            const idx = adminUsers.findIndex((p) => p.toLowerCase() === currentPseudo.toLowerCase());
            if (idx < 0) return;

            const nextIdx = idx + offset;
            if (nextIdx < 0 || nextIdx >= adminUsers.length) return;

            const nextPseudo = adminUsers[nextIdx];
            const url = '/members/profile?pseudo=' + encodeURIComponent(nextPseudo) + '&admin=1';
            window.location.href = url;
        }

        document.getElementById('save-profile').addEventListener('click', async () => {
            const bio = document.getElementById('bio').value;
            const commentary = document.getElementById('commentary').value;
            try {
                const res = await fetch('/members/profile/data', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo: currentPseudo, bio, commentary })
                });
                const data = await res.json();
                setMsg('profile-msg', !!data.ok, data.message || 'Profil mis a jour.');
            } catch (err) {
                setMsg('profile-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('upload-avatar').addEventListener('click', async () => {
            const input = document.getElementById('avatar');
            if (!input.files || input.files.length === 0) {
                setMsg('avatar-msg', false, 'Choisis un fichier image.');
                return;
            }

            const form = new FormData();
            form.append('pseudo', currentPseudo);
            form.append('avatar', input.files[0]);

            try {
                const res = await fetch('/members/avatar', {
                    method: 'POST',
                    body: form
                });
                const data = await res.json();
                setMsg('avatar-msg', !!data.ok, data.message || 'Avatar mis a jour.');
            } catch (err) {
                setMsg('avatar-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('save-password').addEventListener('click', async () => {
            const currentPassword = document.getElementById('current-password').value;
            const newPassword = document.getElementById('new-password').value;
            if (!newPassword) {
                setMsg('password-msg', false, 'Entre un nouveau mot de passe.');
                return;
            }

            try {
                const res = adminMode
                    ? await fetch('/japprends/set-password/' + encodeURIComponent(currentPseudo), {
                        method: 'POST',
                        headers: { 'content-type': 'application/json' },
                        body: JSON.stringify({ password: newPassword })
                    })
                    : await fetch('/members/password', {
                        method: 'PUT',
                        headers: { 'content-type': 'application/json' },
                        body: JSON.stringify({
                            pseudo: currentPseudo,
                            current_password: currentPassword,
                            new_password: newPassword
                        })
                    });
                const data = await res.json();
                setMsg('password-msg', !!data.ok, data.message || 'Mot de passe mis a jour.');
                if (data.ok) {
                    document.getElementById('current-password').value = '';
                    document.getElementById('new-password').value = '';
                }
            } catch (err) {
                setMsg('password-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('prev-user').addEventListener('click', () => goToAdminUser(-1));
        document.getElementById('next-user').addEventListener('click', () => goToAdminUser(1));

        document.getElementById('delete-account').addEventListener('click', async () => {
            if (!confirm('Supprimer ton compte definitivement ?')) return;
            try {
                const res = await fetch('/members/account', {
                    method: 'DELETE',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo: currentPseudo })
                });
                const data = await res.json();
                setMsg('delete-msg', !!data.ok, data.message || 'Action terminee.');
                if (data.ok) {
                    if (!adminMode) {
                        localStorage.removeItem('logged_pseudo');
                        setTimeout(() => {
                            window.location.href = '/';
                        }, 500);
                    } else {
                        await loadAdminUsersNavigator();
                        if (adminUsers.length > 0) {
                            const url = '/members/profile?pseudo=' + encodeURIComponent(adminUsers[0]) + '&admin=1';
                            window.location.href = url;
                        } else {
                            window.location.href = '/dashboard';
                        }
                    }
                }
            } catch (err) {
                setMsg('delete-msg', false, 'Erreur: ' + err.message);
            }
        });

        (async () => {
            if (adminMode) {
                const ok = await ensureAdminSession();
                if (!ok) {
                    alert('Authentification admin requise pour ouvrir ce profil.');
                    window.location.href = '/japprends/login';
                    return;
                }
                await loadAdminUsersNavigator();
            }
            loadProfile();
            updateAdminNavMeta();
        })();
    </script>
</body>
</html>
"##,
    )
}
