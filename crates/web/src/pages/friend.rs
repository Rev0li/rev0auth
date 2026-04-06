use axum::response::Html;

use super::friend_page_assembly;

pub async fn friend() -> Html<String> {
    Html(friend_page_assembly::assemble_friend_page(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Home - rev0auth</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRONTEND_SHARED_CSS%%
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: #132331;
            background:
                radial-gradient(circle at 15% 10%, #d9f0ff, transparent 40%),
                radial-gradient(circle at 85% 80%, #ffe7ca, transparent 40%),
                linear-gradient(135deg, #eef8ff 0%, #e6f7ee 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            padding: 28px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 32px;
            gap: 16px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.8rem, 5vw, 2.5rem); }
        .header-meta {
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .header-status {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            align-items: center;
        }
        .header-actions {
            display: flex;
            flex-direction: column;
            align-items: flex-end;
            gap: 10px;
        }
        .header-action-row {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            justify-content: flex-end;
        }
        .logout-btn {
            padding: 8px 14px;
            background: rgba(255, 107, 59, 0.9);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .logout-btn:hover { background: rgba(239, 78, 36, 1); }
        .profile-btn {
            padding: 8px 14px;
            background: rgba(13, 155, 115, 0.92);
            color: white;
            border: 0;
            border-radius: 10px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
        }
        .profile-btn:hover { background: rgba(10, 132, 98, 1); }
        .chat-card {
            margin-bottom: 20px;
            border: 1px solid rgba(19, 35, 49, 0.14);
            border-radius: 14px;
            background: rgba(255, 255, 255, 0.9);
            padding: 16px;
            box-shadow: 0 12px 24px rgba(19, 35, 49, 0.09);
        }
        .chat-card h2 {
            margin: 0 0 10px;
            font-size: 1.1rem;
        }
        .chat-card label {
            display: block;
            margin: 10px 0 6px;
            font-weight: 700;
        }
        .chat-card input,
        .chat-card textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        .chat-card textarea {
            min-height: 110px;
            resize: vertical;
        }
        .chat-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .chat-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
            display: block;
        }
        .chat-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
            display: block;
        }
        .chat-history {
            margin-top: 12px;
            max-height: 300px;
            overflow: auto;
            display: grid;
            gap: 8px;
            padding-right: 4px;
        }
        .chat-bubble {
            max-width: 86%;
            padding: 9px 11px;
            border-radius: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            white-space: pre-wrap;
            line-height: 1.4;
            font-size: 0.9rem;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.07);
        }
        .chat-bubble.mine {
            justify-self: end;
            background: #e8fff5;
            border-color: #b3ecd1;
        }
        .chat-bubble.theirs {
            justify-self: start;
            background: #fff;
        }
        .chat-meta {
            display: block;
            margin-top: 4px;
            font-size: 0.78rem;
            opacity: 0.72;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 20px;
            padding: 28px;
            box-shadow: 0 16px 45px rgba(19, 35, 49, 0.12);
            margin-bottom: 18px;
        }
        h2 { margin: 0 0 14px; font-size: 1.4rem; }
        .greeting { font-size: 1.1rem; line-height: 1.6; margin-bottom: 20px; }
        .greeting strong { color: #0d9b73; }
        .feature-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        .feature-list li {
            padding: 10px 0;
            border-bottom: 1px solid rgba(19, 35, 49, 0.08);
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .feature-list li:last-child { border-bottom: 0; }
        .feature-list li:before {
            content: "→";
            color: #ff6f3f;
            font-weight: 700;
        }
        .status-buttons {
            display: flex;
            gap: 10px;
            margin-top: 16px;
            flex-wrap: wrap;
        }
        .status-btn {
            padding: 8px 14px;
            border: 1px solid rgba(13, 155, 115, 0.3);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.05);
            color: #0d9b73;
            font-weight: 600;
            cursor: pointer;
            font-size: 0.9rem;
        }
        .status-btn:hover {
            background: rgba(13, 155, 115, 0.15);
        }
        .status-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .status-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .status-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
        .remark-panel {
            display: none;
            margin-top: 14px;
            padding: 12px;
            border: 1px dashed rgba(19, 35, 49, 0.2);
            border-radius: 12px;
            background: rgba(243, 247, 250, 0.8);
        }
        .services {
            display: flex;
            flex-direction: column;
            gap: 14px;
            margin-top: 14px;
        }
        .service-card {
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 12px;
            background: #fff;
            padding: 14px;
            width: 100%;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .service-media {
            width: 100%;
            aspect-ratio: 16 / 9;
            height: auto;
            object-fit: cover;
            border-radius: 10px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            background: #f3f7fa;
        }
        .service-card h3 {
            margin: 0 0 6px;
            font-size: 1rem;
        }
        .service-card p {
            margin: 0 0 10px;
            font-size: 0.9rem;
            opacity: 0.8;
        }
        .service-btn {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            background: rgba(13, 155, 115, 0.08);
            color: #0d9b73;
            font-weight: 700;
            padding: 9px 10px;
            cursor: pointer;
        }
        .service-btn:hover {
            background: rgba(13, 155, 115, 0.18);
        }
        .service-btn.locked {
            background: #f3f7fa;
            color: #4b5f71;
            border-color: rgba(19, 35, 49, 0.16);
        }
        .service-state {
            font-size: 0.84rem;
            margin: 8px 0;
            opacity: 0.84;
        }
        .service-input {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 8px;
            box-sizing: border-box;
            margin-bottom: 8px;
        }
        .service-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .service-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .service-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
        %%FRIEND_ONBOARDING_CSS%%
        %%FRIEND_SERVICES_CSS%%
        %%FRIEND_CHAT_CSS%%
        %%FRIEND_STATUS_CSS%%
        %%FRIEND_AVATAR_CSS%%
        @media (max-width: 900px) {
            .container {
                padding: 18px;
            }
            .header {
                flex-direction: column;
                align-items: stretch;
            }
            .header-actions {
                align-items: flex-start;
            }
            .header-action-row {
                justify-content: flex-start;
            }
            .service-card {
                aspect-ratio: 1 / 1;
                overflow: hidden;
            }
            .service-media {
                aspect-ratio: 1 / 1;
            }
        }
        .onboarding-modal {
            position: fixed;
            inset: 0;
            background: rgba(10, 20, 30, 0.6);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 90;
            padding: 16px;
        }
        .onboarding-card {
            width: 100%;
            max-width: 520px;
            background: #fff;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            padding: 16px;
        }
        .onboarding-msg {
            margin-top: 10px;
            border-radius: 8px;
            padding: 8px;
            font-size: 0.9rem;
            display: none;
        }
        .onboarding-msg.ok {
            display: block;
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
        }
        .onboarding-msg.error {
            display: block;
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
        }
    </style>
</head>
<body>
    <div id="onboarding-modal" class="onboarding-modal">
        <div class="onboarding-card">
            <h2>Premiere connexion</h2>
            <p style="margin-top:0;opacity:.85;">Change ton mot de passe temporaire avant de continuer.</p>

            <label for="onboarding-new-password" style="display:block;font-weight:700;margin-top:10px;">Nouveau mot de passe</label>
            <input id="onboarding-new-password" type="password" placeholder="nouveau mot de passe" style="width:100%;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;" />

            <label for="onboarding-confirm-password" style="display:block;font-weight:700;margin-top:10px;">Confirmer le nouveau mot de passe</label>
            <input id="onboarding-confirm-password" type="password" placeholder="retape le nouveau mot de passe" style="width:100%;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;" />

            <label for="onboarding-message" style="display:block;font-weight:700;margin-top:10px;">Message de presentation</label>
            <textarea id="onboarding-message" placeholder="Presente-toi rapidement..." style="width:100%;min-height:110px;border:1px solid rgba(19,35,49,.2);border-radius:8px;padding:9px;box-sizing:border-box;font:inherit;background:#fff;"></textarea>

            <div class="actions" style="margin-top:10px;">
                <button id="onboarding-submit" class="profile-btn">Valider</button>
            </div>
            <div id="onboarding-msg" class="onboarding-msg"></div>
        </div>
    </div>

    <main class="container">
        <div class="header">
            <div class="header-meta">
                <h1>Bienvenue <span id="welcome-pseudo">!</span></h1>
                <div class="header-status">
                    <span style="font-weight:700;opacity:.8;">Humeur rapide:</span>
                    <button class="status-btn" id="happy-btn">😀 Content</button>
                    <button class="status-btn" id="meh-btn">😐 Bof</button>
                    <button class="status-btn" id="question-btn">❓ Besoin d'aide</button>
                </div>
                <div id="status-msg" class="status-msg"></div>
            </div>
            <div class="header-actions">
                <img id="header-avatar" class="header-avatar" alt="Photo de profil" />
                <div class="header-action-row">
                    <a class="profile-btn" href="/members/profile">Mon profil</a>
                    <button class="logout-btn" id="logout-btn">Déconnexion</button>
                </div>
            </div>
        </div>

        <article class="card">
            <h2>Presentation du projet</h2>
            <p class="greeting">
                Bienvenue <strong id="pseudo-display">ami</strong>.
                <br>rev0auth est un projet self-host Rust pour apprendre, construire et partager un systeme auth clair.
            </p>
            <p class="greeting">
                Si le projet t'aide, tu peux soutenir son evolution via un don.
                <br>Chaque contribution aide a maintenir l'infra, les tests et la documentation publique.
            </p>
            <div class="actions">
                <a class="profile-btn" href="https://github.com/sponsors/Rev0li" target="_blank" rel="noopener noreferrer">Faire un don</a>
            </div>
        </article>

        <article class="card">
            <h2>Services intégrés</h2>
            <p style="margin-top: 0; opacity: 0.8;">Par defaut tu n'as acces a rien. Demande l'acces service par service.</p>
            <div class="services">
                <div class="service-card">
                    <h3>Songsurf</h3>
                    <img class="service-media" src="https://images.unsplash.com/photo-1516280440614-37939bbacd81?auto=format&fit=crop&w=900&q=80" alt="Songsurf - musique" />
                    <p>Acces au service musique.</p>
                    <div class="service-state" id="songsurf-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="songsurf-btn">Demander acces Songsurf</button>
                </div>
                <div class="service-card">
                    <h3>Jellyfin</h3>
                    <img class="service-media" src="https://images.unsplash.com/photo-1489599849927-2ee91cede3ba?auto=format&fit=crop&w=900&q=80" alt="Jellyfin - streaming video" />
                    <p>Acces streaming media.</p>
                    <div class="service-state" id="jellyfin-state">Etat: verrouille</div>
                    <button class="service-btn locked" id="jellyfin-btn">Demander acces Jellyfin</button>
                </div>
                <div class="service-card">
                    <h3>GitHub</h3>
                    <img class="service-media" src="data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 900 400'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0' x2='1' y1='0' y2='1'%3E%3Cstop stop-color='%23142331'/%3E%3Cstop offset='1' stop-color='%230d9b73'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='900' height='400' rx='26' fill='url(%23g)'/%3E%3Ccircle cx='165' cy='160' r='88' fill='%23ffffff' fill-opacity='.12'/%3E%3Ccircle cx='165' cy='145' r='48' fill='%23ffffff'/%3E%3Cpath d='M95 275c24-42 57-61 70-61s46 19 70 61' fill='%23ffffff'/%3E%3Ctext x='305' y='175' fill='%23ffffff' font-family='Space Grotesk, Arial, sans-serif' font-size='62' font-weight='700'%3EGitHub Profile%3C/text%3E%3Ctext x='305' y='238' fill='%23d9f0ff' font-family='Space Grotesk, Arial, sans-serif' font-size='28'%3EAcces conditionne par star + demande%3C/text%3E%3C/svg%3E" alt="GitHub profile" />
                    <p>Acces conditionne: compte cree + star sur ce projet.</p>
                    <div class="service-state" id="github-state">Etat: verrouille</div>
                    <input id="github-username" class="service-input" placeholder="Ton username GitHub" />
                    <button class="service-btn locked" id="github-btn">J'ai mis une star, demander acces GitHub</button>
                </div>
            </div>
            <div id="service-msg" class="service-msg"></div>

        </article>

        <article class="chat-card">
            <h2>Start message with me</h2>
            <p style="margin:0 0 8px;opacity:.82;">Conversation directe avec l'admin.</p>

            <div id="chat-history" class="chat-history">Chargement de l'historique...</div>

            <label for="chat-subject">Sujet</label>
            <input id="chat-subject" placeholder="Sujet" />

            <label for="chat-body">Message</label>
            <textarea id="chat-body" placeholder="Ton message..."></textarea>

            <div class="actions" style="margin-top:10px;">
                <button id="chat-send-btn" class="profile-btn">Envoyer message</button>
            </div>
            <div id="chat-msg" class="chat-msg"></div>
        </article>

    </main>

    <script>
        // Get logged-in pseudo from localStorage
        const pseudo = localStorage.getItem('logged_pseudo');

        if (!pseudo) {
            window.location.href = '/';
        } else {
            document.getElementById('welcome-pseudo').textContent = pseudo;
            document.getElementById('pseudo-display').textContent = pseudo;
        }

        const needsOnboarding = localStorage.getItem('needs_onboarding') === '1';

        %%COMMON_JS_UTILS%%

        // Initialize avatar module
        const avatarModule = createFriendAvatarModule({ pseudo });
        avatarModule.setHeaderAvatarSrc(false);

        // Initialize status module
        const statusModule = createFriendStatusModule({ pseudo });

        // Initialize services module
        const servicesModule = createFriendServicesModule({ pseudo });

        // Initialize chat module
        const chatModule = createFriendChatModule({ pseudo });

        // Initialize onboarding module
        const onboardingModule = createFriendOnboardingModule({ pseudo });

        // Logout function
        async function logout() {
            if (!confirm('Confirmer la déconnexion ?')) return;
            
            try {
                // Mark user as inactive
                await fetch('/status/set-inactive/' + pseudo, { method: 'POST', cache: 'no-store' });
            } catch (err) {
                console.log('Logout error:', err);
            }
            
            // Clear session and redirect
            localStorage.removeItem('logged_pseudo');
            window.location.href = '/';
        }

        document.getElementById('logout-btn').addEventListener('click', logout);
        
        if (needsOnboarding) {
            onboardingModule.openOnboardingModal();
        }

        %%FRIEND_ONBOARDING_JS%%
        %%FRIEND_SERVICES_JS%%
        %%FRIEND_CHAT_JS%%
        %%FRIEND_STATUS_JS%%
        %%FRIEND_AVATAR_JS%%
    </script>
</body>
</html>
"##,
    ))
}
