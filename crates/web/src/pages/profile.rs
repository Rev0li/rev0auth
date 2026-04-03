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
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Profil membre</h1>
            <p class="meta">Edite ta bio et mets a jour ton avatar.</p>
            <div class="actions">
                <a class="btn secondary" href="/members/dashboard">Retour dashboard</a>
            </div>
        </article>

        <article class="card">
            <h2>Bio</h2>
            <label for="bio">Presentation</label>
            <textarea id="bio" placeholder="Qui es-tu, ce que tu fais, ce que tu veux partager..."></textarea>
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
    </main>

    <script>
        const pseudo = localStorage.getItem('logged_pseudo');
        if (!pseudo) {
            window.location.href = '/';
        }

        function setMsg(id, ok, text) {
            const el = document.getElementById(id);
            el.className = 'msg ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        async function loadProfile() {
            try {
                const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
                const data = await res.json();
                if (data.ok && typeof data.bio === 'string') {
                    document.getElementById('bio').value = data.bio;
                }
            } catch (_err) {
                setMsg('profile-msg', false, 'Impossible de charger le profil.');
            }
        }

        document.getElementById('save-profile').addEventListener('click', async () => {
            const bio = document.getElementById('bio').value;
            try {
                const res = await fetch('/members/profile/data', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo, bio })
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
            form.append('pseudo', pseudo);
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

        loadProfile();
    </script>
</body>
</html>
"##,
    )
}
