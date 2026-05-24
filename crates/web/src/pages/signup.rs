use axum::response::Html;

use super::{frontend_theme, signup_page_styles};

pub const AVATARS: &[(&str, &str, &str)] = &[
    ("fox", "Renard", r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#d4500a'/><polygon points='20,55 30,20 42,55' fill='#d4500a'/><polygon points='58,55 70,20 80,55' fill='#d4500a'/><polygon points='23,52 30,27 39,52' fill='#f9b084'/><polygon points='61,52 70,27 77,52' fill='#f9b084'/><circle cx='50' cy='60' r='22' fill='#f9b084'/><ellipse cx='43' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><ellipse cx='57' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><circle cx='44' cy='53' r='1.2' fill='white'/><circle cx='58' cy='53' r='1.2' fill='white'/><ellipse cx='50' cy='64' rx='3' ry='2' fill='#1a1a1a'/><ellipse cx='50' cy='68' rx='9' ry='5' fill='#fde4cc' opacity='0.7'/></svg>"#),
    ("wolf", "Loup", r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#4a5568'/><polygon points='18,52 28,15 40,52' fill='#4a5568'/><polygon points='60,52 72,15 82,52' fill='#4a5568'/><polygon points='21,50 28,22 37,50' fill='#9aa5b4'/><polygon points='63,50 72,22 79,50' fill='#9aa5b4'/><ellipse cx='50' cy='62' rx='24' ry='20' fill='#9aa5b4'/><ellipse cx='50' cy='71' rx='13' ry='9' fill='#bec5cf'/><ellipse cx='42' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><circle cx='43' cy='53' r='1.3' fill='#e8f0fe'/><circle cx='59' cy='53' r='1.3' fill='#e8f0fe'/><ellipse cx='50' cy='65' rx='4' ry='2.5' fill='#2d3748'/></svg>"#),
    ("cat", "Chat", r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#6b46c1'/><polygon points='22,52 32,18 44,52' fill='#6b46c1'/><polygon points='56,52 68,18 78,52' fill='#6b46c1'/><polygon points='25,50 32,25 41,50' fill='#f9a8d4'/><polygon points='59,50 68,25 75,50' fill='#f9a8d4'/><circle cx='50' cy='60' r='22' fill='#9f7aea'/><ellipse cx='42' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='42' cy='54' rx='2' ry='3.5' fill='#52b788'/><ellipse cx='58' cy='54' rx='2' ry='3.5' fill='#52b788'/><circle cx='43' cy='53' r='1' fill='white'/><polygon points='50,62 47,65 53,65' fill='#f9a8d4'/><line x1='28' y1='64' x2='43' y2='67' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='28' y1='68' x2='43' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='67' x2='72' y2='64' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='68' x2='72' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/></svg>"#),
    ("eagle", "Aigle", r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#1a202c'/><circle cx='50' cy='56' r='24' fill='#744210'/><circle cx='50' cy='48' r='17' fill='#f7fafc'/><circle cx='44' cy='46' r='5' fill='#f6ad55'/><circle cx='44' cy='46' r='3' fill='#1a1a1a'/><circle cx='45' cy='45' r='1' fill='white'/><polygon points='35,52 50,48 37,60' fill='#f6ad55'/><ellipse cx='63' cy='62' rx='12' ry='8' fill='#2d3748'/><ellipse cx='37' cy='63' rx='10' ry='7' fill='#744210'/></svg>"#),
    ("dragon", "Dragon", r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#065f46'/><polygon points='38,30 34,10 42,28' fill='#34d399'/><polygon points='62,30 66,10 58,28' fill='#34d399'/><circle cx='50' cy='58' r='24' fill='#059669'/><ellipse cx='50' cy='70' rx='12' ry='9' fill='#34d399'/><circle cx='46' cy='69' r='2' fill='#065f46'/><circle cx='54' cy='69' r='2' fill='#065f46'/><ellipse cx='41' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='59' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='41' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><ellipse cx='59' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><path d='M38,64 Q50,58 62,64' fill='none' stroke='#34d399' stroke-width='1.5' opacity='0.6'/></svg>"#),
];

fn avatar_grid_html() -> String {
    AVATARS.iter().map(|(id, name, svg)| {
        let encoded = svg.replace('#', "%23");
        let src = format!("data:image/svg+xml;charset=UTF-8,{encoded}");
        format!(
            r#"<button class="avatar-btn" data-id="{id}" type="button" title="{name}"><img src="{src}" alt="{name}" /><span>{name}</span></button>"#
        )
    }).collect::<Vec<_>>().join("\n")
}

pub fn signup_invalid() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Lien invalide</title>
    %%FRONTEND_THEME_BOOT%%
    <style>%%FRONTEND_SHARED_CSS%%
    %%SIGNUP_PAGE_STYLES%%</style>
</head>
<body>
    <main class="page">
        <div class="error-page">
            <h2>Lien invalide ou expiré</h2>
            <p>Ce lien d'inscription n'est plus valide.<br>Contacte un admin pour en obtenir un nouveau.</p>
        </div>
    </main>
</body>
</html>"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS)
        .replace("%%SIGNUP_PAGE_STYLES%%", signup_page_styles::SIGNUP_PAGE_STYLES),
    )
}

pub fn signup_form(invite_code: &str) -> Html<String> {
    let grid = avatar_grid_html();
    let template = r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Inscription — rev0auth</title>
    %%FRONTEND_THEME_BOOT%%
    <style>%%FRONTEND_SHARED_CSS%%
    %%SIGNUP_PAGE_STYLES%%</style>
</head>
<body>
    <main class="page">
        <div class="header">
            <h1>Créer un compte</h1>
            <p class="subtitle">Tu as été invité(e) à rejoindre la plateforme.</p>
            <a class="link" href="/">← Déjà un compte ? Connexion</a>
        </div>

        <article class="card">
            <div class="field">
                <label for="pseudo">Pseudo *</label>
                <input id="pseudo" type="text" placeholder="ton_pseudo" autocomplete="username" />
                <span class="hint-warn">⚠ Ton pseudo est définitif — il ne pourra jamais être modifié.</span>
            </div>

            <div class="field">
                <label for="password">Mot de passe *</label>
                <input id="password" type="password" placeholder="8 caractères minimum" autocomplete="new-password" />
            </div>

            <div class="field">
                <label for="confirm">Confirmer le mot de passe *</label>
                <input id="confirm" type="password" placeholder="répète le mot de passe" autocomplete="new-password" />
            </div>

            <div class="avatar-section">
                <label>Photo de profil</label>
                <div class="avatar-grid" id="avatar-grid">
                    %%AVATAR_GRID%%
                </div>
            </div>

            <button class="btn btn-primary" id="signup-btn">Créer mon compte</button>
            <div id="result" class="result"></div>
        </article>
    </main>

    <script>
        const INVITE_CODE = '%%INVITE_CODE%%';
        const PSEUDO_RE = /^[a-zA-Z0-9_-]{3,20}$/;
        let selectedAvatarId = null;

        document.querySelectorAll('.avatar-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.avatar-btn').forEach(b => b.classList.remove('selected'));
                btn.classList.add('selected');
                selectedAvatarId = btn.getAttribute('data-id');
            });
        });

        function showResult(ok, msg) {
            const el = document.getElementById('result');
            el.className = 'result ' + (ok ? 'ok' : 'err');
            el.textContent = msg;
        }

        document.getElementById('signup-btn').addEventListener('click', async () => {
            const pseudo = document.getElementById('pseudo').value.trim();
            const password = document.getElementById('password').value;
            const confirm = document.getElementById('confirm').value;

            if (!PSEUDO_RE.test(pseudo)) {
                showResult(false, 'Pseudo invalide : 3-20 caractères, lettres/chiffres/tiret/underscore, pas d\'espaces.');
                return;
            }
            if (password.length < 8) {
                showResult(false, 'Mot de passe trop court (8 caractères minimum).');
                return;
            }
            if (password !== confirm) {
                showResult(false, 'Les mots de passe ne correspondent pas.');
                return;
            }

            const btn = document.getElementById('signup-btn');
            btn.disabled = true;

            try {
                const res = await fetch('/signup', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo, password, invite_code: INVITE_CODE, avatar_id: selectedAvatarId })
                });
                const data = await res.json();
                if (data.ok) {
                    showResult(true, 'Compte créé ! Redirection vers la connexion...');
                    setTimeout(() => { window.location.href = '/'; }, 1500);
                } else {
                    showResult(false, data.message || 'Erreur lors de la création du compte.');
                    btn.disabled = false;
                }
            } catch {
                showResult(false, 'Erreur réseau. Réessaie.');
                btn.disabled = false;
            }
        });
    </script>
</body>
</html>"##;

    Html(
        template
            .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
            .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS)
            .replace("%%SIGNUP_PAGE_STYLES%%", signup_page_styles::SIGNUP_PAGE_STYLES)
            .replace("%%AVATAR_GRID%%", &grid)
            .replace("%%INVITE_CODE%%", invite_code),
    )
}
