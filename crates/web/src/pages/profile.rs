use axum::response::Html;

use super::profile_page_assembly;

pub async fn profile(songsurf_url: &str) -> Html<String> {
    let html = profile_page_assembly::assemble_profile_page(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Profil membre - rev0auth</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRONTEND_SHARED_CSS%%
        %%PROFILE_PAGE_STYLES%%
        %%FRIEND_CHAT_STYLES%%
    </style>
</head>
<body>
    <nav class="profile-navbar">
        <div class="profile-navbar-left">
            <a class="btn-back" id="back-link" href="/home/friend">← Retour</a>
            <div id="admin-note" class="admin-note" style="display:none">Mode admin</div>
        </div>
        <div class="profile-navbar-right">
            <img id="profile-nav-avatar" class="profile-nav-avatar" alt="" />
            <span id="profile-nav-pseudo" class="profile-nav-pseudo"></span>
        </div>
    </nav>

    <main class="page">
        <div id="admin-nav" class="admin-nav" style="display:none">
            <button id="prev-user" class="btn-profile-action secondary">← Précédent</button>
            <button id="next-user" class="btn-profile-action secondary">Suivant →</button>
            <span id="admin-nav-meta" class="meta"></span>
        </div>

        <article class="card">
            <h2>Infos compte</h2>
            <div class="info-grid">
                <div class="info-item"><span class="info-label">Pseudo</span><span id="info-pseudo" class="info-val">--</span></div>
                <div class="info-item"><span class="info-label">Rôle</span><span id="info-role" class="info-val">--</span></div>
                <div class="info-item admin-only" id="status-row"><span class="info-label">Status</span><span id="info-status" class="info-val">--</span></div>
                <div class="info-item admin-only" id="status-emoji-row"><span class="info-label">Smiley</span><span id="info-status-emoji" class="info-val">--</span></div>
                <div class="info-item admin-only" id="commentary-row"><span class="info-label">Note admin</span><span id="info-commentary" class="info-val">--</span></div>
                <div class="info-item"><span class="info-label">Membre depuis</span><span id="info-created" class="info-val">--</span></div>
            </div>
        </article>

        <article class="card admin-only" id="services-admin-card">
            <h2>Accès services</h2>
            <div id="services-admin-grid" class="services-admin-grid"></div>
            <div id="services-admin-msg" class="msg"></div>
        </article>

        <article class="card admin-only" id="admin-actions-card">
            <h2>Actions admin</h2>
            <div class="admin-actions-row">
                <button id="approve-toggle-btn" class="btn-profile-action" style="display:none">...</button>
                <div id="approve-status-label" class="meta"></div>
            </div>
            <div id="approve-msg" class="msg" style="margin-top:6px"></div>
            <hr class="admin-actions-sep" />
            <h3 class="admin-actions-sub">Écrire un message</h3>
            <textarea id="admin-msg-body" class="field-input" rows="3" placeholder="Ton message pour ce membre..."></textarea>
            <button id="admin-send-msg-btn" class="btn-profile-action" style="margin-top:8px">Envoyer</button>
            <div id="admin-msg-result" class="msg" style="margin-top:6px"></div>
        </article>

        <article class="card admin-only" id="songsurf-logs-card">
            <h2>Logs SongSurf</h2>
            <div id="songsurf-logs-list" class="dl-log-list">Chargement…</div>
        </article>

        <article class="card">
            <h2>Avatar</h2>
            <div class="avatar-section">
                <div class="avatar-current">
                    <img id="avatar-preview" class="avatar-preview" alt="Avatar" />
                    <div id="avatar-state" class="info-label">Aucun avatar.</div>
                </div>
                <div class="avatar-actions-col">
                    <input id="avatar" type="file" accept="image/*" style="display:none" />
                    <button id="upload-avatar" class="btn-profile-action">Importer une image</button>
                    <button id="delete-avatar" class="btn-profile-action danger">Supprimer</button>
                </div>
            </div>
            <div class="default-avatar-label">Avatars par défaut</div>
            <div id="default-avatar-grid" class="default-avatar-grid"></div>
            <div id="avatar-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Mot de passe</h2>
            <label for="current-password" class="field-label">Mot de passe actuel</label>
            <input id="current-password" type="password" placeholder="Actuel" class="field-input" />
            <label for="new-password" class="field-label">Nouveau mot de passe</label>
            <input id="new-password" type="password" placeholder="Nouveau" class="field-input" />
            <label for="confirm-password" class="field-label">Confirmer le nouveau</label>
            <input id="confirm-password" type="password" placeholder="Confirmer" class="field-input" />
            <div class="actions">
                <button id="save-password" class="btn-profile-action">Mettre à jour</button>
            </div>
            <div id="password-msg" class="msg"></div>
        </article>

        <article class="card" id="donation-card">
            <h2>Don</h2>
            <a class="donation-gh-link" href="https://github.com/sponsors/Rev0li" target="_blank" rel="noopener noreferrer">❤️ GitHub Sponsors →</a>
            <label for="donation-method" class="field-label">Méthode</label>
            <select id="donation-method" class="field-input">
                <option value="pcs" selected>Coupon PCS</option>
                <option value="crypto">Crypto</option>
            </select>

            <div id="pcs-info-section" style="display:none" class="donation-hint">
                <span class="donation-hint-icon">ℹ</span>
                <span>Les coupons PCS Mastercard s'achètent en tabac avec un <strong>montant libre</strong> (5 €, 10 €, 20 €, 50 €…). Gratte le code au dos et colle-le dans le champ ci-dessous.</span>
            </div>

            <div id="crypto-addresses-section" style="display:none">
                <p class="field-label" style="margin-top:10px">Adresses :</p>
                <div id="crypto-addresses-list" class="list-box" style="margin-top:4px"></div>
            </div>

            <label for="donation-code" class="field-label">Code / Référence</label>
            <input id="donation-code" type="text" placeholder="TX ID ou code coupon" class="field-input" />
            <div class="actions">
                <button id="upload-donation" class="btn-profile-action">Don</button>
                <button id="refresh-donations" class="btn-profile-action secondary">↺</button>
            </div>
            <div id="donation-msg" class="msg"></div>
            <div id="donations-list" class="don-list"></div>
        </article>

        <article class="card card-danger">
            <h2>Danger zone</h2>
            <p class="meta">Cette action est irréversible.</p>
            <button id="delete-account" class="btn-profile-action danger">Supprimer mon compte</button>
            <div id="delete-msg" class="msg"></div>
        </article>
    </main>

    <!-- Chat FAB -->
    <div class="chat-fab-wrap" id="chat-fab-wrap">
        <button class="chat-fab" id="chat-open-btn" title="Messages">
            💬
            <span class="chat-fab-badge" id="chat-fab-badge"></span>
        </button>
    </div>

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
        const SONGSURF_URL = '%%SONGSURF_URL%%';

        %%COMMON_JS_UTILS%%
        %%PROFILE_INFO_MODULE%%
        %%PROFILE_AVATAR_MODULE%%
        %%PROFILE_PASSWORD_MODULE%%
        %%PROFILE_DONATIONS_MODULE%%
        %%PROFILE_ADMIN_NAVIGATOR_MODULE%%
        %%PROFILE_ACCOUNT_DELETION_MODULE%%
        %%FRIEND_CHAT_MODULE%%

        const params = new URLSearchParams(window.location.search);
        const adminMode = params.get('admin') === '1';
        const queryPseudo = (params.get('pseudo') || '').trim();
        const localPseudo = localStorage.getItem('logged_pseudo');
        const pseudo = adminMode ? (queryPseudo || localPseudo || '') : (localPseudo || '');

        if (!pseudo) { window.location.href = '/'; }

        // Populate navbar
        document.getElementById('profile-nav-pseudo').textContent = pseudo;
        (function() {
            const navAvatar = document.getElementById('profile-nav-avatar');
            const probe = new Image();
            probe.onload = () => { navAvatar.src = probe.src; };
            probe.onerror = () => { navAvatar.style.display = 'none'; };
            probe.src = '/members/avatar/' + encodeURIComponent(pseudo) + '?t=' + Date.now();
        })();

        if (adminMode) {
            document.getElementById('admin-note').style.display = 'block';
            document.getElementById('admin-nav').style.display = 'flex';
            document.getElementById('back-link').setAttribute('href', '/japprends/tdd#members');
            document.querySelectorAll('.admin-only').forEach((el) => {
                el.style.display = el.classList.contains('info-item') ? 'flex' : 'block';
            });
            document.getElementById('donation-card').style.display = 'none';
            document.getElementById('chat-fab-wrap').style.display = 'none';
        }

        async function ensureAdminSession() {
            if (!adminMode) return true;
            try {
                const res = await fetch('/japprends/auth-check', {
                    method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({})
                });
                if (res.ok) return true;
            } catch (_) {}
            const password = window.prompt('Session admin expirée. Entre le mot de passe admin:');
            if (!password) return false;
            try {
                const loginRes = await fetch('/japprends/login', {
                    method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ password })
                });
                if (!loginRes.ok) return false;
                const checkRes = await fetch('/japprends/auth-check', {
                    method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({})
                });
                return checkRes.ok;
            } catch (_) { return false; }
        }

        const infoModule = createProfileInfoModule({ pseudo, adminMode });
        const avatarModule = createProfileAvatarModule({ pseudo });
        const passwordModule = createProfilePasswordModule({ pseudo, adminMode });
        const donationsModule = createProfileDonationsModule({ pseudo, adminMode });
        const adminNavigatorModule = createProfileAdminNavigatorModule({ pseudo, adminMode });
        const accountDeletionModule = createProfileAccountDeletionModule({ pseudo, adminMode });
        accountDeletionModule.setAdminNavigatorModule(adminNavigatorModule);

        if (!adminMode) {
            createFriendChatModule({ pseudo });
        }

        document.getElementById('upload-avatar').addEventListener('click', () => {
            document.getElementById('avatar').click();
        });
        document.getElementById('avatar').addEventListener('change', () => {
            avatarModule.uploadAvatar();
        });

        function renderServicesAdmin(data) {
            const grid = document.getElementById('services-admin-grid');
            if (!grid) return;
            const services = [
                { key: 'songsurf', label: 'Songsurf', req: data.request_songsurf, access: data.access_songsurf,
                  meta: data.github_username ? '⭐ GitHub : @' + data.github_username : null },
                { key: 'jellyfin', label: 'Jellyfin', req: data.request_jellyfin, access: data.access_jellyfin,
                  meta: data.linkedin_name ? '🤝 LinkedIn : ' + data.linkedin_name : null },
            ];
            grid.innerHTML = services.map(s =>
                '<div class="svc-admin-row">'
                + '<div>'
                + '<span class="svc-admin-label">' + escapeHtml(s.label) + '</span>'
                + (s.req ? ' <span class="svc-req-badge">⏳ demande en cours</span>' : '')
                + (s.meta ? '<div class="svc-admin-meta">' + escapeHtml(s.meta) + '</div>' : '')
                + '</div>'
                + '<button class="btn-profile-action' + (s.access ? ' danger' : '') + '" onclick="toggleServiceAdmin(\'' + s.key + '\',' + !s.access + ')">'
                + (s.access ? 'Révoquer' : 'Accorder')
                + '</button>'
                + '</div>'
            ).join('');
        }

        async function toggleServiceAdmin(service, value) {
            const payload = {};
            payload['access_' + service] = !!value;
            try {
                const res = await fetch('/japprends/users/' + pseudo, {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify(payload)
                });
                const data = await res.json();
                const msgEl = document.getElementById('services-admin-msg');
                if (data.ok) {
                    const profileData = await infoModule.loadProfile();
                    if (profileData) renderServicesAdmin(profileData);
                    msgEl.className = 'msg ok';
                    msgEl.textContent = 'Accès mis à jour.';
                    setTimeout(() => { msgEl.className = 'msg'; msgEl.textContent = ''; }, 2000);
                } else {
                    msgEl.className = 'msg down';
                    msgEl.textContent = data.message || 'Erreur.';
                }
            } catch (err) {
                document.getElementById('services-admin-msg').className = 'msg down';
                document.getElementById('services-admin-msg').textContent = 'Erreur: ' + err.message;
            }
        }

        async function loadSongsurfLogs() {
            if (!adminMode || !SONGSURF_URL) return;
            const panel = document.getElementById('songsurf-logs-list');
            if (!panel) return;
            try {
                const res = await fetch(
                    SONGSURF_URL.replace(/\/$/, '') + '/api/admin/dl-logs?pseudo=' + encodeURIComponent(pseudo),
                    { credentials: 'include' }
                );
                if (!res.ok) { panel.textContent = 'SongSurf indisponible.'; return; }
                const data = await res.json();
                if (!data.success || !data.entries || data.entries.length === 0) {
                    panel.textContent = 'Aucun téléchargement.';
                    return;
                }
                panel.innerHTML = data.entries.map(e =>
                    '<div class="dl-log-row">'
                    + '<span class="dl-log-ts">' + escapeHtml(e.timestamp) + '</span>'
                    + '<span class="dl-log-track">' + escapeHtml(e.artist) + ' — ' + escapeHtml(e.title) + '</span>'
                    + '<span class="dl-log-album">' + escapeHtml(e.album) + '</span>'
                    + '</div>'
                ).join('') + '<div class="dl-log-total">Total : ' + data.total + ' DL</div>';
            } catch (e) {
                panel.textContent = 'Erreur : ' + e.message;
            }
        }

        (async () => {
            if (adminMode) {
                const ok = await ensureAdminSession();
                if (!ok) {
                    alert('Authentification admin requise.');
                    window.location.href = '/japprends/login';
                    return;
                }
                await adminNavigatorModule.loadAdminUsersNavigator();
            }
            const profileData = await infoModule.loadProfile();
            if (adminMode && profileData) renderServicesAdmin(profileData);
            if (adminMode) await adminNavigatorModule.loadApprovalStatus(pseudo);
            if (adminMode) loadSongsurfLogs();
            await avatarModule.loadAvatarState();
            if (!adminMode) {
                await donationsModule.loadDonations();
            }
            adminNavigatorModule.updateAdminNavMeta();
        })();
    </script>
</body>
</html>
"##
    );
    Html(html.replace("%%SONGSURF_URL%%", songsurf_url))
}
