use axum::response::Html;

use super::{frontend_theme, portal_page_styles};

pub async fn portal() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Inscription</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%PORTAL_PAGE_STYLES%%
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
            <small class="hint-warn">⚠ Ton pseudo est définitif — il ne pourra jamais être modifié.</small>

            <label for="referral">Comment tu m'as connu ? *</label>
            <textarea id="referral" rows="2" placeholder="Ami, réseau social, portfolio..." required style="resize:vertical"></textarea>

            <button class="btn btn-primary" id="signup-btn">Envoyer ma demande</button>
            <div id="temp-password-box" class="result"></div>
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
                const pwd = data.temp_password || tempPassword;
                tempPasswordBox.style.display = 'block';
                tempPasswordBox.innerHTML = '✅ Demande envoyée ! Mot de passe temporaire :<br><code id="shown-pwd" style="font-size:1.1em;letter-spacing:0.05em;user-select:all">' + pwd + '</code>'
                    + '<button type="button" id="copy-pwd-btn" style="margin-left:10px;padding:3px 10px;border-radius:6px;border:1px solid currentColor;background:transparent;cursor:pointer;font-size:0.8em" onclick="(function(){navigator.clipboard.writeText(document.getElementById(\'shown-pwd\').textContent).then(()=>{var b=document.getElementById(\'copy-pwd-btn\');b.textContent=\'✓ Copié\';setTimeout(()=>{b.textContent=\'Copier\'},1800)})})()">Copier</button>'
                    + '<br><small style="opacity:0.7;margin-top:4px;display:block">Note-le bien — il ne sera plus affiché.</small>';
            } else {
                setResult(output, false, data.message || 'Inscription refusee.');
                tempPasswordBox.style.display = 'none';
                tempPasswordBox.textContent = '';
            }
        });
    </script>
</body>
</html>
"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
    .replace("%%PORTAL_PAGE_STYLES%%", portal_page_styles::PORTAL_PAGE_STYLES)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}
