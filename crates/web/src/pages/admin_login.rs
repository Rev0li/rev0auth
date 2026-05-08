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
        #yubikey-step { display: none; }
        #password-step { display: none; }
        .yubikey-icon {
            font-size: 2.5rem;
            display: block;
            text-align: center;
            margin: 12px 0;
            animation: pulse 1.8s ease-in-out infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; transform: scale(1); }
            50% { opacity: 0.6; transform: scale(0.96); }
        }
    </style>
</head>
<body>
    <main class="page">
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>

        <!-- YubiKey-only mode (shown when key is registered) -->
        <article class="card" id="yubikey-step">
            <h1>Admin Access</h1>
            <p class="hint">Insere ta cle et touche-la pour te connecter.</p>
            <span class="yubikey-icon">🔑</span>
            <p id="yubikey-waiting" style="text-align:center;color:var(--color-muted);font-size:0.9rem;margin:0 0 12px">En attente de la cle...</p>
            <div id="yubikey-result" class="result"></div>
            <button class="btn" id="yubikey-retry-btn" style="display:none;margin-top:8px;background:var(--bg-page);color:var(--color-muted);border:1px solid var(--color-panel-border)">
                Reessayer
            </button>
        </article>

        <!-- Password mode (shown when no key is registered) -->
        <article class="card" id="password-step">
            <h1>Admin Access</h1>
            <p class="hint">Pseudo + seed + mot de passe + OTP 2FA (si actif) + challenge.</p>

            <div class="field">
                <label for="pseudo">Pseudo admin</label>
                <input id="pseudo" type="text" placeholder="admin pseudo" autocomplete="username" />
            </div>
            <div class="field">
                <label for="seed">Seed admin</label>
                <input id="seed" type="password" placeholder="admin seed" />
            </div>
            <div class="field">
                <label for="password">Mot de passe admin</label>
                <input id="password" type="password" placeholder="admin password" autocomplete="current-password" />
            </div>
            <div class="field">
                <label for="otp">Code OTP 2FA <span style="font-weight:400;color:var(--color-muted)">(optionnel)</span></label>
                <input id="otp" type="text" inputmode="numeric" autocomplete="one-time-code" placeholder="123456" />
            </div>

            <div class="trap-zone" aria-hidden="true">
                <label for="website">website</label>
                <input id="website" type="text" autocomplete="off" />
                <button id="fake-invisible-btn" type="button">fake</button>
            </div>

            <p class="challenge-label">Challenge: clique uniquement sur 🔒 Lock</p>
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
        // ---- WebAuthn helpers ----
        function base64urlToBuffer(b64) {
            const b = (b64 + '===').slice(0, b64.length + (4 - b64.length % 4) % 4)
                .replace(/-/g, '+').replace(/_/g, '/');
            const bin = atob(b);
            const buf = new Uint8Array(bin.length);
            for (let i = 0; i < bin.length; i++) buf[i] = bin.charCodeAt(i);
            return buf.buffer;
        }
        function bufferToBase64url(buf) {
            const bytes = new Uint8Array(buf);
            let str = '';
            for (const b of bytes) str += String.fromCharCode(b);
            return btoa(str).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
        }

        function setResult(el, ok, text) {
            el.className = 'result ' + (ok ? 'ok' : 'down');
            el.textContent = text;
            el.style.display = 'block';
        }

        // ---- Probe on load: choose mode based on key registration ----
        async function probeAndInit() {
            let data;
            try {
                const res = await fetch('/japprends/webauthn/auth/start');
                data = await res.json();
            } catch (e) {
                // Network error — fall back to password mode
                showPasswordMode();
                return;
            }

            if (data.locked) {
                document.getElementById('yubikey-step').style.display = 'block';
                const output = document.getElementById('yubikey-result');
                setResult(output, false, data.message || 'Trop de tentatives. Reessaie plus tard.');
                document.getElementById('yubikey-waiting').style.display = 'none';
                return;
            }

            if (data.webauthn_required) {
                showYubiKeyMode(data.webauthn_token, data.webauthn_challenge);
            } else {
                showPasswordMode();
            }
        }

        function showPasswordMode() {
            document.getElementById('password-step').style.display = 'block';
            document.getElementById('yubikey-step').style.display = 'none';
        }

        function showYubiKeyMode(token, challenge) {
            document.getElementById('yubikey-step').style.display = 'block';
            document.getElementById('password-step').style.display = 'none';
            startYubiKeyAuth(token, challenge);
        }

        // ---- YubiKey auth ----
        async function startYubiKeyAuth(token, challenge) {
            const output = document.getElementById('yubikey-result');
            const waiting = document.getElementById('yubikey-waiting');
            const retryBtn = document.getElementById('yubikey-retry-btn');

            if (!token || !challenge) {
                setResult(output, false, 'Session expiree. Recharge la page.');
                retryBtn.style.display = 'block';
                return;
            }

            output.style.display = 'none';
            retryBtn.style.display = 'none';
            if (waiting) waiting.style.display = 'block';

            try {
                const options = challenge.publicKey;
                options.challenge = base64urlToBuffer(options.challenge);
                if (options.allowCredentials) {
                    options.allowCredentials = options.allowCredentials.map(c => ({ ...c, id: base64urlToBuffer(c.id) }));
                }

                const cred = await navigator.credentials.get({ publicKey: options });
                if (waiting) waiting.style.display = 'none';

                const credJSON = {
                    id: cred.id,
                    rawId: bufferToBase64url(cred.rawId),
                    type: cred.type,
                    response: {
                        authenticatorData: bufferToBase64url(cred.response.authenticatorData),
                        clientDataJSON: bufferToBase64url(cred.response.clientDataJSON),
                        signature: bufferToBase64url(cred.response.signature),
                        userHandle: cred.response.userHandle ? bufferToBase64url(cred.response.userHandle) : null,
                    }
                };

                const res = await fetch('/japprends/webauthn/auth/finish', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ token, credential: credJSON })
                });
                const data = await res.json();
                setResult(output, data.ok, data.message);
                if (data.ok) {
                    setTimeout(() => { window.location.href = '/dashboard'; }, 350);
                } else {
                    retryBtn.style.display = 'block';
                }
            } catch (err) {
                if (waiting) waiting.style.display = 'none';
                setResult(output, false, 'Erreur YubiKey: ' + err.message);
                retryBtn.style.display = 'block';
            }
        }

        document.getElementById('yubikey-retry-btn').addEventListener('click', () => {
            probeAndInit();
        });

        // ---- Password mode setup ----
        let challengeChoice = '';
        let trapTouched = false;

        document.querySelectorAll('.challenge-btn').forEach((btn) => {
            btn.addEventListener('click', () => {
                challengeChoice = btn.getAttribute('data-choice') || '';
                document.querySelectorAll('.challenge-btn').forEach((n) => n.classList.remove('selected'));
                btn.classList.add('selected');
            });
        });

        document.getElementById('fake-invisible-btn').addEventListener('click', () => { trapTouched = true; });

        function bindEnter(inputId, buttonId) {
            const input = document.getElementById(inputId);
            const btn = document.getElementById(buttonId);
            if (!input || !btn) return;
            input.addEventListener('keydown', (e) => { if (e.key === 'Enter') { e.preventDefault(); btn.click(); } });
        }
        bindEnter('pseudo', 'login-btn');
        bindEnter('seed', 'login-btn');
        bindEnter('password', 'login-btn');
        bindEnter('otp', 'login-btn');

        document.getElementById('login-btn').addEventListener('click', async () => {
            const pseudo = document.getElementById('pseudo').value.trim();
            const seed = document.getElementById('seed').value.trim();
            const password = document.getElementById('password').value.trim();
            const otp = document.getElementById('otp').value.trim();
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
                body: JSON.stringify({ pseudo, seed, password, otp, challenge_choice: challengeChoice, trap_value: trapTouched ? 'clicked' : website })
            });
            const data = await res.json();
            setResult(output, data.ok, data.message);
            if (data.ok) setTimeout(() => { window.location.href = '/dashboard'; }, 350);
        });

        // ---- Start ----
        probeAndInit();
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
