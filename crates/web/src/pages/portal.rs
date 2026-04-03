use axum::response::Html;

pub async fn portal() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Inscription</title>
    <style>
        body {
            margin: 0;
            font-family: "Space Grotesk", "Avenir Next", sans-serif;
            color: #132331;
            background:
                radial-gradient(circle at 12% 0%, #ffe7ca 0%, transparent 32%),
                radial-gradient(circle at 88% 12%, #d5ecff 0%, transparent 35%),
                linear-gradient(150deg, #eef8ff 0%, #e6f7ee 100%);
            min-height: 100vh;
        }
        .page {
            max-width: 600px;
            margin: 0 auto;
            padding: 28px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 16px;
            gap: 10px;
        }
        .header h1 { margin: 0; font-size: clamp(1.5rem, 5vw, 2rem); }
        .card {
            background: rgba(255, 255, 255, 0.9);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 28px;
            box-shadow: 0 16px 36px rgba(19, 35, 49, 0.14);
        }
        .hint { margin: 12px 0 24px; opacity: .82; font-size: 0.95rem; }
        label { display: block; margin: 14px 0 6px; font-weight: 700; }
        .label-optional { font-size: 0.85rem; font-weight: 400; opacity: 0.75; }
        input, textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 10px;
            padding: 10px;
            font: inherit;
            background: #fff;
            box-sizing: border-box;
        }
        textarea { min-height: 80px; resize: vertical; }
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
        <div class="header">
            <h1>S'inscrire</h1>
            <a class="link" href="/">← Connexion</a>
        </div>

        <article class="card">
            <p class="hint">Inscription avec validation manuelle. Tu recevras un mot de passe temporaire a copier apres envoi.</p>

            <label for="pseudo">Pseudo *</label>
            <input id="pseudo" placeholder="ton_pseudo" required />

            <label for="referral">Comment tu m'as connu ? *</label>
            <input id="referral" placeholder="Ami, réseau social, portfolio..." required />

            <button class="btn btn-primary" id="signup-btn">Envoyer ma demande</button>
            <div id="temp-password-box" class="result" style="margin-top:14px; display:none;"></div>
            <div id="signup-error" class="result"></div>
        </article>
    </main>

    <script>
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

        function generateTempPassword() {
            const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789';
            const bytes = new Uint8Array(12);
            crypto.getRandomValues(bytes);
            return Array.from(bytes, (value) => chars[value % chars.length]).join('');
        }

        bindEnterToClick('pseudo', 'signup-btn');
        bindEnterToClick('referral', 'signup-btn');

        document.getElementById('signup-btn').addEventListener('click', async () => {
            const pseudo = document.getElementById('pseudo').value.trim();
            const referral = document.getElementById('referral').value.trim();
            const output = document.getElementById('signup-error');
            const tempPasswordBox = document.getElementById('temp-password-box');

            if (!pseudo || !referral) {
                setResult(output, false, 'Remplis les champs obligatoires (pseudo, comment tu m\'as connu).');
                return;
            }

            const tempPassword = generateTempPassword();

            const res = await fetch('/portal/signup-request', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, referral, temp_password: tempPassword })
            });
            const data = await res.json();

            if (data.ok) {
                output.style.display = 'none';
                output.textContent = '';
                tempPasswordBox.style.display = 'block';
                tempPasswordBox.innerHTML = 'Mot de passe temporaire: <strong>' + (data.temp_password || tempPassword) + '</strong>';
            } else {
                setResult(output, false, data.message || 'Inscription refusee.');
                tempPasswordBox.style.display = 'none';
                tempPasswordBox.textContent = '';
            }
        });
    </script>
</body>
</html>
"##,
    )
}
