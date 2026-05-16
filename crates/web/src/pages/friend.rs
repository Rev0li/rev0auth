use axum::response::Html;

use super::friend_page_assembly;

pub async fn friend(songsurf_url: &str) -> Html<String> {
    Html(friend_page_assembly::assemble_friend_page(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Home - rev0auth</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRIEND_PAGE_STYLES%%
        %%FRIEND_ONBOARDING_CSS%%
        %%FRIEND_SERVICES_CSS%%
        %%FRIEND_CHAT_CSS%%
        %%FRIEND_STATUS_CSS%%
        %%FRIEND_AVATAR_CSS%%
        %%FRIEND_WALL_CSS%%
    </style>
</head>
<body>

    <!-- Onboarding modal — first login -->
    <div id="onboarding-modal" class="onboarding-modal">
        <div class="onboarding-card">
            <h2>Première connexion</h2>
            <p class="onboarding-intro">Choisis ton mot de passe et une photo de profil avant de continuer.</p>

            <label for="onboarding-new-password" class="onboarding-label">Nouveau mot de passe</label>
            <input id="onboarding-new-password" class="onboarding-field" type="password" placeholder="nouveau mot de passe" />

            <label for="onboarding-confirm-password" class="onboarding-label">Confirmer le mot de passe</label>
            <input id="onboarding-confirm-password" class="onboarding-field" type="password" placeholder="retape le nouveau mot de passe" />

            <span class="onboarding-label" style="margin-top:14px;display:block">Choisis un avatar (optionnel)</span>
            <div id="onboarding-avatar-grid" class="onboarding-avatar-grid"></div>

            <div class="actions actions-tight">
                <button id="onboarding-submit" class="btn-action">Valider et continuer</button>
            </div>
            <div id="onboarding-msg" class="onboarding-msg"></div>
        </div>
    </div>

    <!-- Navbar -->
    <nav class="navbar">
        <span class="navbar-brand">rev0auth</span>
        <div class="navbar-user">
            <img id="header-avatar" class="header-avatar" alt="Avatar" />
            <span id="pseudo-display" class="navbar-pseudo"></span>
            <a class="nav-btn" href="/members/profile">Profil</a>
            <button class="nav-btn nav-btn-logout" id="logout-btn">Déconnexion</button>
        </div>
    </nav>

    <!-- Hero -->
    <section class="hero">
        <p class="hero-sub">Cet espace est un réseau privé réservé à un cercle de confiance. Tes données restent chez nous — hébergées sur nos propres machines, sans publicité, sans tracking, sans tiers. Tu accèdes à des services sélectionnés, partages avec la communauté et gardes le contrôle.</p>
        <div class="hero-steps">
            <div class="hero-step"
                 data-popup-title="GitHub"
                 data-popup-desc="Consulte le profil GitHub de Rev0li."
                 data-popup-href="https://github.com/Rev0li"
                 data-popup-img="/static/hero/github.png"
                 data-popup-target="_blank">
                <span class="hero-step-icon">🐙</span>
                <span class="hero-step-label">GitHub</span>
                <span class="hero-step-hint">Consulte le profil</span>
            </div>
            <span class="hero-step-arrow">→</span>
            <div class="hero-step"
                 data-popup-title="Star le repo"
                 data-popup-desc="Star rev0auth sur GitHub pour soutenir le projet."
                 data-popup-href="https://github.com/Rev0li/rev0auth"
                 data-popup-img="/static/hero/repo.png"
                 data-popup-target="_blank">
                <span class="hero-step-icon">⭐</span>
                <span class="hero-step-label">Star le repo</span>
                <span class="hero-step-hint">rev0auth sur GitHub</span>
            </div>
            <span class="hero-step-arrow">→</span>
            <div class="hero-step"
                 data-popup-title="Inscription"
                 data-popup-desc="Demande ton accès au réseau privé rev0auth."
                 data-popup-href="/portal"
                 data-popup-img="/static/hero/portal.png"
                 data-popup-target="">
                <span class="hero-step-icon">✍️</span>
                <span class="hero-step-label">Inscription</span>
                <span class="hero-step-hint">Demande ton accès</span>
            </div>
        </div>
    </section>

    <!-- Hero step preview popup -->
    <div id="step-popup-overlay" class="step-popup-overlay">
        <div class="step-popup-panel">
            <button class="step-popup-close" id="step-popup-close">✕</button>
            <div class="step-popup-img-wrap">
                <img id="step-popup-img" class="step-popup-img" alt="" />
                <div class="step-popup-img-placeholder" id="step-popup-placeholder"></div>
            </div>
            <div class="step-popup-body">
                <div class="step-popup-icon" id="step-popup-icon"></div>
                <strong class="step-popup-title" id="step-popup-title"></strong>
                <p class="step-popup-desc" id="step-popup-desc"></p>
                <a class="btn-action step-popup-btn" id="step-popup-link" rel="noopener noreferrer">Ouvrir →</a>
            </div>
        </div>
    </div>

    <main class="page-content">

        <!-- Services -->
        <section class="section" id="section-services">
            <h2 class="section-heading">Services</h2>
            <p class="section-sub">Chaque service a ses conditions d'accès. Suis les étapes pour débloquer.</p>
            <div class="services-grid">

                <div class="svc-card" id="songsurf-card">
                    <div class="svc-card-banner svc-banner-songsurf">
                        <span class="svc-icon">🎵</span>
                        <span>Songsurf</span>
                    </div>
                    <div class="svc-card-body" id="songsurf-body"></div>
                </div>

                <div class="svc-card">
                    <div class="svc-card-banner svc-banner-jellyfin">
                        <span class="svc-icon">🎬</span>
                        <span>Jellyfin</span>
                    </div>
                    <div class="svc-card-body" id="jellyfin-body"></div>
                </div>

            </div>
            <div id="service-msg" class="service-msg"></div>
        </section>

        <!-- GitHub Support -->
        <section class="section" id="section-support">
            <h2 class="section-heading">Soutenir le projet</h2>
            <div class="support-card">
                <div class="support-text">
                    <p>rev0auth est un projet open-source self-hosted.</p>
                    <p>Chaque contribution aide à maintenir l'infra, les tests et la doc publique.</p>
                </div>
                <a class="btn-action" href="https://github.com/sponsors/Rev0li" target="_blank" rel="noopener noreferrer">
                    ❤️ Faire un don
                </a>
            </div>
        </section>

        <!-- Community Wall -->
        <section class="section" id="section-wall">
            <h2 class="section-heading">Mur communautaire</h2>
            <p class="section-sub">Messages courts visibles par tous les membres.</p>
            <div id="wall-list" class="wall-list">
                <p class="wall-empty">Chargement...</p>
            </div>
            <div class="wall-compose">
                <textarea id="wall-input" class="wall-input" maxlength="140" rows="2" placeholder="Laisse un message... (140 car. max)"></textarea>
                <div class="wall-compose-footer">
                    <span id="wall-char-count" class="wall-char-count">140</span>
                    <button id="wall-send-btn" class="btn-action">Poster</button>
                </div>
            </div>
        </section>

        <!-- Socials / Portfolio -->
        <section class="section" id="section-socials">
            <h2 class="section-heading">Liens</h2>
            <div class="socials-row">
                <a class="social-card" href="https://github.com/Rev0li" target="_blank" rel="noopener noreferrer">
                    <span class="social-icon">⭐</span>
                    <span>GitHub</span>
                </a>
                <a class="social-card" href="#PORTFOLIO_URL" target="_blank" rel="noopener noreferrer">
                    <span class="social-icon">🌐</span>
                    <span>Portfolio</span>
                </a>
            </div>
        </section>

    </main>

    <!-- Chat FAB + badge -->
    <div class="chat-fab-wrap">
        <button class="chat-fab" id="chat-open-btn" title="Messages">
            💬
            <span class="chat-fab-badge" id="chat-fab-badge"></span>
        </button>
    </div>

    <!-- Chat popup -->
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
            <div class="chat-emoji-wrap">
                <button id="chat-emoji-btn" class="chat-emoji-btn" type="button" title="Emojis">😊</button>
                <div id="chat-emoji-panel" class="chat-emoji-panel"></div>
            </div>
            <textarea id="chat-body" class="chat-overlay-input" rows="1" placeholder="Message..."></textarea>
            <button id="chat-send-btn" class="chat-overlay-send">➤</button>
        </div>
    </div>

    <script>
        const pseudo = localStorage.getItem('logged_pseudo');
        if (!pseudo) { window.location.href = '/'; }

        const needsOnboarding = localStorage.getItem('needs_onboarding') === '1';

        %%COMMON_JS_UTILS%%

        const avatarModule = createFriendAvatarModule({ pseudo });
        avatarModule.loadAvatar();

        const statusModule = createFriendStatusModule({ pseudo });
        const servicesModule = createFriendServicesModule({ pseudo });
        const chatModule = createFriendChatModule({ pseudo });
        const wallModule = createFriendWallModule({ pseudo });
        const onboardingModule = createFriendOnboardingModule({ pseudo, avatarModule });

        // Populate pseudo displays
        const pseudoDisplay = document.getElementById('pseudo-display');
        if (pseudoDisplay) pseudoDisplay.textContent = pseudo;
        const heroPseudo = document.getElementById('hero-pseudo');
        if (heroPseudo) heroPseudo.textContent = pseudo;

        async function logout() {
            if (!confirm('Confirmer la déconnexion ?')) return;
            try {
                await fetch('/status/set-inactive/' + pseudo, { method: 'POST', cache: 'no-store' });
            } catch (_) {}
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
        %%FRIEND_WALL_JS%%

        // Hero step popups
        (function() {
            const overlay = document.getElementById('step-popup-overlay');
            const closeBtn = document.getElementById('step-popup-close');
            const popupImg = document.getElementById('step-popup-img');
            const popupPlaceholder = document.getElementById('step-popup-placeholder');
            const popupIcon = document.getElementById('step-popup-icon');
            const popupTitle = document.getElementById('step-popup-title');
            const popupDesc = document.getElementById('step-popup-desc');
            const popupLink = document.getElementById('step-popup-link');

            document.querySelectorAll('.hero-step[data-popup-href]').forEach(el => {
                el.addEventListener('click', () => {
                    const imgSrc = el.getAttribute('data-popup-img') || '';
                    const title = el.getAttribute('data-popup-title') || '';
                    const desc = el.getAttribute('data-popup-desc') || '';
                    const href = el.getAttribute('data-popup-href') || '';
                    const target = el.getAttribute('data-popup-target') || '';
                    const iconEl = el.querySelector('.hero-step-icon');

                    popupIcon.textContent = iconEl ? iconEl.textContent : '';
                    popupTitle.textContent = title;
                    popupDesc.textContent = desc;
                    popupLink.href = href;
                    popupLink.target = target;
                    popupLink.textContent = target === '_blank' ? 'Ouvrir dans un nouvel onglet →' : 'Accéder →';

                    popupPlaceholder.textContent = iconEl ? iconEl.textContent : '';
                    popupImg.style.display = 'none';
                    popupPlaceholder.style.display = 'flex';

                    if (imgSrc) {
                        const probe = new Image();
                        probe.onload = () => {
                            popupImg.src = probe.src;
                            popupImg.style.display = 'block';
                            popupPlaceholder.style.display = 'none';
                        };
                        probe.src = imgSrc;
                    }

                    overlay.classList.add('open');
                });
            });

            function closePopup() { overlay.classList.remove('open'); }
            if (closeBtn) closeBtn.addEventListener('click', closePopup);
            overlay.addEventListener('click', (e) => { if (e.target === overlay) closePopup(); });
            document.addEventListener('keydown', (e) => { if (e.key === 'Escape') closePopup(); });
        })();
    </script>
</body>
</html>
"##,
        songsurf_url,
    ))
}
