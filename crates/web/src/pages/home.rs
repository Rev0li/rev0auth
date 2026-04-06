use axum::response::Html;

use super::frontend_theme;

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
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: #132331;
            background:
                radial-gradient(circle at 10% 5%, #ffe7cd, transparent 35%),
                radial-gradient(circle at 90% 0%, #d9f0ff, transparent 40%),
                linear-gradient(145deg, #eef7ff, #e8f8ef);
            min-height: 100vh;
        }
        .page {
            max-width: 500px;
            margin: 0 auto;
            padding: 28px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            min-height: 100vh;
        }
        .card {
            background: rgba(255, 255, 255, 0.9);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 32px;
            box-shadow: 0 18px 45px rgba(19, 35, 49, 0.15);
        }
        h1 { margin: 0 0 8px; font-size: clamp(1.5rem, 5vw, 2rem); }
        .hint { margin: 10px 0 24px; opacity: .8; font-size: 0.95rem; }
        label { display: block; margin: 14px 0 6px; font-weight: 700; }
        input {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 10px;
            padding: 10px;
            font: inherit;
            background: #fff;
            box-sizing: border-box;
        }
        .btn {
            margin-top: 16px;
            border: 0;
            border-radius: 10px;
            padding: 11px 16px;
            font-weight: 700;
            cursor: pointer;
            width: 100%;
        }
        .btn-primary { color: #fff; background: linear-gradient(120deg, #ff6b3b, #ef4e24); }
        .result {
            margin-top: 14px;
            border-radius: 10px;
            padding: 10px;
            font-size: .92rem;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; }
        .link {
            display: block;
            margin-top: 18px;
            text-align: center;
            text-decoration: none;
            color: #132331;
            font-weight: 700;
        }
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
                <button class="btn btn-primary" id="pseudo-btn">Verifier</button>
                <div id="pseudo-result" class="result"></div>
            </div>

            <!-- STEP 2: Password Input (hidden until pseudo is approved) -->
            <div id="step-2" style="display: none;">
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
                    window.location.href = '/members/dashboard';
                }, 600);
            }
        });
    </script>
</body>
</html>
"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}
