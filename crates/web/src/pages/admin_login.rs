use axum::response::Html;

use super::{admin_login_page_styles, frontend_theme};

pub async fn admin_login() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Admin Login</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%ADMIN_LOGIN_PAGE_STYLES%%
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Admin Access</h1>
            <p class="hint">Connexion admin renforcee: pseudo + seed + mot de passe + bouton challenge.</p>

            <label for="pseudo">Pseudo admin</label>
            <input id="pseudo" type="text" placeholder="admin pseudo" />

            <label for="seed">Seed admin</label>
            <input id="seed" type="password" placeholder="admin seed" />

            <label for="password">Mot de passe admin</label>
            <input id="password" type="password" placeholder="admin password" />

            <div class="trap-zone" aria-hidden="true">
                <label for="website">website</label>
                <input id="website" type="text" autocomplete="off" />
                <button id="fake-invisible-btn" type="button">fake</button>
            </div>

            <label class="challenge-label">Challenge fun: clique uniquement sur 🔒 Lock</label>
            <div class="challenge-grid">
                <button class="challenge-btn" data-choice="spark" type="button">✨ Spark</button>
                <button class="challenge-btn" data-choice="rocket" type="button">🚀 Rocket</button>
                <button class="challenge-btn" data-choice="secure-lock" type="button">🔒 Lock</button>
                <button class="challenge-btn" data-choice="sun" type="button">☀ Sun</button>
                <button class="challenge-btn" data-choice="moon" type="button">🌙 Moon</button>
                <button class="challenge-btn" data-choice="star" type="button">⭐ Star</button>
            </div>

            <button class="btn" id="login-btn">Se connecter</button>
            <div id="login-result" class="result"></div>
        </article>
    </main>

    <script>
        function setResult(el, ok, text) {
            el.className = 'result ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        let challengeChoice = '';
        let trapTouched = false;

        document.querySelectorAll('.challenge-btn').forEach((btn) => {
            btn.addEventListener('click', () => {
                challengeChoice = btn.getAttribute('data-choice') || '';
                document.querySelectorAll('.challenge-btn').forEach((node) => node.classList.remove('selected'));
                btn.classList.add('selected');
            });
        });

        document.getElementById('fake-invisible-btn').addEventListener('click', () => {
            trapTouched = true;
        });

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

        bindEnterToClick('pseudo', 'login-btn');
        bindEnterToClick('seed', 'login-btn');
        bindEnterToClick('password', 'login-btn');

        document.getElementById('login-btn').addEventListener('click', async () => {
            const pseudo = document.getElementById('pseudo').value.trim();
            const seed = document.getElementById('seed').value.trim();
            const password = document.getElementById('password').value.trim();
            const website = document.getElementById('website').value.trim();
            const output = document.getElementById('login-result');

            if (!pseudo || !seed || !password) {
                setResult(output, false, 'Entre pseudo, seed et mot de passe admin.');
                return;
            }

            if (!challengeChoice) {
                setResult(output, false, 'Clique un bouton challenge.');
                return;
            }

            const res = await fetch('/japprends/login', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo,
                    seed,
                    password,
                    challenge_choice: challengeChoice,
                    trap_value: trapTouched ? 'clicked' : website
                })
            });

            const data = await res.json();
            setResult(output, data.ok, data.message);

            if (data.ok) {
                setTimeout(() => {
                    window.location.href = '/dashboard';
                }, 350);
            }
        });
    </script>
</body>
</html>
"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace(
            "%%ADMIN_LOGIN_PAGE_STYLES%%",
            admin_login_page_styles::ADMIN_LOGIN_PAGE_STYLES,
        )
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}