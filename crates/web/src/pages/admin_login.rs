use axum::response::Html;

pub async fn admin_login() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Admin Login</title>
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
            max-width: 520px;
            margin: 0 auto;
            padding: 28px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            min-height: 100vh;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 32px;
            box-shadow: 0 18px 45px rgba(19, 35, 49, 0.15);
        }
        h1 { margin: 0 0 8px; font-size: clamp(1.5rem, 5vw, 2rem); }
        .hint { margin: 10px 0 24px; opacity: .82; font-size: 0.95rem; }
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
            color: #fff;
            background: linear-gradient(120deg, #ff6b3b, #ef4e24);
        }
        .result {
            margin-top: 14px;
            border-radius: 10px;
            padding: 10px;
            font-size: .92rem;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; }
        .challenge-grid {
            margin-top: 10px;
            display: grid;
            grid-template-columns: repeat(3, minmax(0, 1fr));
            gap: 8px;
        }
        .challenge-btn {
            border: 1px solid rgba(19, 35, 49, 0.18);
            background: #fff;
            color: #132331;
            border-radius: 10px;
            padding: 8px;
            font-weight: 700;
            cursor: pointer;
        }
        .challenge-btn.selected {
            border-color: #ef4e24;
            background: #fff0ec;
        }
        .trap-zone {
            position: absolute;
            left: -10000px;
            top: -10000px;
            width: 1px;
            height: 1px;
            overflow: hidden;
        }
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

            <label style="margin-top:12px;">Challenge fun: clique uniquement sur 🔒 Lock</label>
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
"##,
    )
}