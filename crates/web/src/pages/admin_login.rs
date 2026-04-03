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
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Admin Access</h1>
            <p class="hint">Connexion requise pour acceder au dashboard admin.</p>

            <label for="password">Mot de passe admin</label>
            <input id="password" type="password" placeholder="admin password" />
            <button class="btn" id="login-btn">Se connecter</button>
            <div id="login-result" class="result"></div>
        </article>
    </main>

    <script>
        function setResult(el, ok, text) {
            el.className = 'result ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        document.getElementById('login-btn').addEventListener('click', async () => {
            const password = document.getElementById('password').value.trim();
            const output = document.getElementById('login-result');

            if (!password) {
                setResult(output, false, 'Entre le mot de passe admin.');
                return;
            }

            const res = await fetch('/japprends/login', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ password })
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