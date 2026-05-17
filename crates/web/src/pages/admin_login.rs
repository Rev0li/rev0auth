use axum::response::Html;

use super::{admin_login_page_styles, frontend_theme};

pub async fn admin_login(has_key: bool) -> Html<String> {
    let body = if has_key {
        yubikey_mode()
    } else {
        bootstrap_mode()
    };

    Html(
        format!(
            r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Admin Login</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%ADMIN_LOGIN_PAGE_STYLES%%
        .mono-hint {{
            font-family: monospace;
            background: var(--muted);
            color: var(--muted-foreground);
            padding: 8px 12px;
            border-radius: var(--radius-sm);
            font-size: 0.85rem;
            margin: 12px 0;
        }}
        .step-list {{
            padding: 0 0 0 1.2em;
            margin: 12px 0;
            color: var(--muted-foreground);
            font-size: 0.9rem;
            line-height: 1.7;
        }}
    </style>
</head>
<body>
    <main class="page">
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>
        {}
    </main>
    %%SCRIPTS%%
</body>
</html>
"##,
            body
        )
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace(
            "%%ADMIN_LOGIN_PAGE_STYLES%%",
            admin_login_page_styles::ADMIN_LOGIN_PAGE_STYLES,
        )
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS)
        .replace("%%SCRIPTS%%", if has_key { YUBIKEY_SCRIPT } else { BOOTSTRAP_SCRIPT }),
    )
}

fn yubikey_mode() -> &'static str {
    r#"<article class="card">
    <h1>Admin Access</h1>
    <p id="yubikey-waiting" style="text-align:center;color:var(--muted-foreground);font-size:0.9rem;margin:0 0 12px">Attente de connexion...</p>
    <div id="yubikey-result" class="result"></div>
    <button class="btn" id="yubikey-retry-btn" style="display:none;margin-top:8px;background:var(--background);color:var(--muted-foreground);border:1px solid var(--border)">
        Réessayer
    </button>
</article>"#
}

fn bootstrap_mode() -> &'static str {
    r#"<article class="card">
    <h1>Admin Access</h1>
    <p id="yubikey-waiting" style="text-align:center;color:var(--muted-foreground);font-size:0.9rem;margin:0 0 20px">En attente de connexion...</p>
    <button class="btn" id="register-btn" style="display:none">Enregistrer</button>
    <div id="register-result" class="result" style="margin-top:10px"></div>
</article>"#
}

const YUBIKEY_SCRIPT: &str = r#"<script>
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

async function startAuth() {
    const output = document.getElementById('yubikey-result');
    const waiting = document.getElementById('yubikey-waiting');
    const retryBtn = document.getElementById('yubikey-retry-btn');

    output.style.display = 'none';
    retryBtn.style.display = 'none';
    if (waiting) waiting.style.display = 'block';

    let data;
    try {
        const res = await fetch('/japprends/webauthn/auth/start');
        data = await res.json();
    } catch (e) {
        if (waiting) waiting.style.display = 'none';
        setResult(output, false, 'Erreur réseau. Recharge la page.');
        retryBtn.style.display = 'block';
        return;
    }

    if (data.locked) {
        if (waiting) waiting.style.display = 'none';
        setResult(output, false, data.message || 'Trop de tentatives. Réessaie plus tard.');
        return;
    }

    if (!data.webauthn_required) {
        if (waiting) waiting.style.display = 'none';
        setResult(output, false, 'Authentification non disponible. Recharge la page.');
        return;
    }

    try {
        const options = data.webauthn_challenge.publicKey;
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
            body: JSON.stringify({ token: data.webauthn_token, credential: credJSON })
        });
        const resp = await res.json();
        setResult(output, resp.ok, resp.message);
        if (resp.ok) {
            setTimeout(() => { window.location.href = '/japprends/tdd'; }, 350);
        } else {
            retryBtn.style.display = 'block';
        }
    } catch (err) {
        if (waiting) waiting.style.display = 'none';
        retryBtn.style.display = 'block';
    }
}

document.getElementById('yubikey-retry-btn').addEventListener('click', startAuth);
startAuth();
</script>"#;

const BOOTSTRAP_SCRIPT: &str = r#"<script>
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

document.getElementById('register-btn').addEventListener('click', async () => {
    const output = document.getElementById('register-result');
    const btn = document.getElementById('register-btn');
    btn.disabled = true;
    btn.textContent = 'En cours...';
    output.style.display = 'none';

    try {
        const startRes = await fetch('/japprends/webauthn/register/start');
        if (!startRes.ok) {
            const err = await startRes.json().catch(() => ({}));
            setResult(output, false, err.error || 'Erreur serveur au démarrage.');
            btn.disabled = false;
            btn.textContent = 'Enregistrer la clé YubiKey';
            return;
        }
        const ccr = await startRes.json();

        const options = ccr.publicKey;
        options.challenge = base64urlToBuffer(options.challenge);
        options.user.id = base64urlToBuffer(options.user.id);
        if (options.excludeCredentials) {
            options.excludeCredentials = options.excludeCredentials.map(c => ({ ...c, id: base64urlToBuffer(c.id) }));
        }

        const cred = await navigator.credentials.create({ publicKey: options });

        const credJSON = {
            id: cred.id,
            rawId: bufferToBase64url(cred.rawId),
            type: cred.type,
            response: {
                attestationObject: bufferToBase64url(cred.response.attestationObject),
                clientDataJSON: bufferToBase64url(cred.response.clientDataJSON),
            }
        };

        const finishRes = await fetch('/japprends/webauthn/register/finish', {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ credential: credJSON })
        });
        const data = await finishRes.json();
        if (data.ok) {
            setResult(output, true, 'Clé enregistrée ! Rechargement en cours...');
            setTimeout(() => { window.location.reload(); }, 1500);
        } else {
            setResult(output, false, data.message || 'Erreur inconnue.');
            btn.disabled = false;
            btn.textContent = 'Enregistrer la clé YubiKey';
        }
    } catch (err) {
        setResult(output, false, 'Erreur: ' + err.message);
        btn.disabled = false;
        btn.textContent = 'Enregistrer la clé YubiKey';
    }
});
</script>"#;
