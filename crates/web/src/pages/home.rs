use axum::response::Html;

use super::{frontend_theme, home_page_styles};

pub async fn home() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Connexion</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%HOME_PAGE_STYLES%%
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Connexion</h1>
            <p class="hint">Accede a ton compte.</p>

            <label for="pseudo">Pseudo</label>
            <input id="pseudo" placeholder="ton_pseudo" autocomplete="username" />

            <label for="password">Mot de passe</label>
            <input id="password" type="password" placeholder="ton_mot_de_passe" autocomplete="current-password" />

            <button class="btn btn-primary" id="login-btn">Se connecter</button>
            <div id="login-result" class="result"></div>

            <a class="link" href="/portal">→ S'inscrire</a>
        </article>
    </main>

    <script>
        // Auto-fill pseudo from previous session
        (function () {
            const stored = localStorage.getItem('logged_pseudo');
            if (stored) {
                const input = document.getElementById('pseudo');
                if (input) input.value = stored;
            }
        })();

        function setResult(el, ok, text) {
            el.className = 'result ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        async function tryLogin() {
            const pseudo = document.getElementById('pseudo').value.trim();
            const password = document.getElementById('password').value.trim();
            const output = document.getElementById('login-result');
            const btn = document.getElementById('login-btn');

            if (!pseudo || !password) {
                setResult(output, false, 'Pseudo et mot de passe requis.');
                return;
            }

            btn.disabled = true;
            btn.textContent = 'Connexion...';

            const res = await fetch('/auth/password-check', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, password })
            });
            const data = await res.json();
            setResult(output, data.ok, data.message);

            if (data.ok) {
                localStorage.setItem('logged_pseudo', pseudo);
                if (String(data.state || '').toLowerCase() === 'onboarding') {
                    localStorage.setItem('needs_onboarding', '1');
                } else {
                    localStorage.removeItem('needs_onboarding');
                }
                if (data.songsurf_url) {
                    sessionStorage.setItem('songsurf_launch_url', data.songsurf_url);
                } else {
                    sessionStorage.removeItem('songsurf_launch_url');
                }
                setTimeout(() => { window.location.href = '/home/friend'; }, 600);
            } else {
                btn.disabled = false;
                btn.textContent = 'Se connecter';
            }
        }

        document.getElementById('login-btn').addEventListener('click', tryLogin);
        ['pseudo', 'password'].forEach(id => {
            document.getElementById(id).addEventListener('keydown', e => {
                if (e.key === 'Enter') { e.preventDefault(); tryLogin(); }
            });
        });
    </script>
</body>
</html>
"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
    .replace("%%HOME_PAGE_STYLES%%", home_page_styles::HOME_PAGE_STYLES)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}
