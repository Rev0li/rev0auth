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
        %%FRIEND_PAGE_STYLES%%
    </style>
</head>
<body>
    <div id="onboarding-modal" class="onboarding-modal">
        <div class="onboarding-card">
            <h2>Premiere connexion</h2>
            <p class="onboarding-intro">Change ton mot de passe temporaire avant de continuer.</p>

            <label for="onboarding-new-password" class="onboarding-label">Nouveau mot de passe</label>
            <input id="onboarding-new-password" class="onboarding-field" type="password" placeholder="nouveau mot de passe" />

            <label for="onboarding-confirm-password" class="onboarding-label">Confirmer le nouveau mot de passe</label>
            <input id="onboarding-confirm-password" class="onboarding-field" type="password" placeholder="retape le nouveau mot de passe" />

            <label for="onboarding-message" class="onboarding-label">Message de presentation</label>
            <textarea id="onboarding-message" class="onboarding-textarea" placeholder="Presente-toi rapidement..."></textarea>

            <div class="actions actions-tight">
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
                    <span class="mood-label">Humeur rapide:</span>
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
            <p class="services-intro">Par defaut tu n'as acces a rien. Demande l'acces service par service.</p>
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

    </main>

    <!-- Chat FAB + badge -->
    <div class="chat-fab-wrap">
        <button class="chat-fab" id="chat-open-btn" title="Messages">
            💬
            <span class="chat-fab-badge" id="chat-fab-badge"></span>
        </button>
    </div>

    <!-- Chat popup (50% screen height) -->
    <div class="chat-popup" id="chat-popup">
        <div class="chat-popup-header">
            <div class="chat-popup-avatar">A</div>
            <div class="chat-popup-title">Admin</div>
            <button class="chat-popup-close" id="chat-close-btn">✕</button>
        </div>
        <div id="chat-history" class="chat-history">
            <p class="chat-empty">Chargement...</p>
        </div>
        <div id="chat-msg" class="chat-popup-msg"></div>
        <div class="chat-popup-footer">
            <textarea id="chat-body" class="chat-overlay-input" rows="1" placeholder="Message..."></textarea>
            <button id="chat-send-btn" class="chat-overlay-send">➤</button>
        </div>
    </div>

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
