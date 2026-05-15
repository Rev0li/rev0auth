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
            <p class="hint">Accede a ton compte avec ton pseudo.</p>

            <!-- STEP 1: Pseudo Input -->
            <div id="step-1">
                <label for="pseudo">Pseudo</label>
                <input id="pseudo" placeholder="ton_pseudo" />
                <button class="btn btn-primary" id="pseudo-btn">Vérifier</button>
                <div id="pseudo-result" class="result"></div>
            </div>

            <!-- STEP 2: Password Input (hidden until pseudo is approved) -->
            <div id="step-2" class="hidden">
                <p class="hint">Pseudo approuve ! Entre ton mot de passe.</p>
                <label for="password">Mot de passe</label>
                <input id="password" type="password" placeholder="ton_mot_de_passe" />
                <button class="btn btn-primary" id="password-btn">Se connecter</button>
                <div id="password-result" class="result"></div>
            </div>

            <a class="link" href="/portal">→ S'inscrire</a>
        </article>
    </main>

    <script>
        let currentPseudo = '';

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

        bindEnterToClick('pseudo', 'pseudo-btn');
        bindEnterToClick('password', 'password-btn');

        // STEP 1: Verify pseudo exists and is approved
        document.getElementById('pseudo-btn').addEventListener('click', async () => {
            const pseudo = document.getElementById('pseudo').value.trim();
            const output = document.getElementById('pseudo-result');

            if (!pseudo) {
                setResult(output, false, 'Entre ton pseudo pour te connecter.');
                return;
            }

            const res = await fetch('/portal/login', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo })
            });
            const data = await res.json();
            setResult(output, data.ok, data.message);

            if (data.ok) {
                currentPseudo = pseudo;
                // Show password step
                document.getElementById('step-1').style.display = 'none';
                document.getElementById('step-2').style.display = 'block';
                document.getElementById('password').focus();
            }
        });

        // STEP 2: Verify password
        document.getElementById('password-btn').addEventListener('click', async () => {
            const password = document.getElementById('password').value.trim();
            const output = document.getElementById('password-result');

            if (!password) {
                setResult(output, false, 'Entre ton mot de passe.');
                return;
            }

            const res = await fetch('/auth/password-check', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: currentPseudo, password })
            });
            const data = await res.json();
            setResult(output, data.ok, data.message);

            if (data.ok) {
                // Store pseudo in localStorage and redirect
                localStorage.setItem('logged_pseudo', currentPseudo);
                if (String(data.state || '').toLowerCase() === 'onboarding') {
                    localStorage.setItem('needs_onboarding', '1');
                } else {
                    localStorage.removeItem('needs_onboarding');
                }
                setTimeout(() => {
                    window.location.href = '/home/friend';
                }, 600);
            }
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
