use axum::response::Html;

use super::profile_page_assembly;

pub async fn profile() -> Html<String> {
    Html(profile_page_assembly::assemble_profile_page(
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
    <div class="profile-topbar">
        <a class="btn-back" id="back-link" href="/home/friend">← Retour</a>
        <div id="admin-note" class="admin-note" style="display:none">Mode admin — modification du profil utilisateur</div>
    </div>

    <main class="page">
        <div id="admin-nav" class="admin-nav" style="display:none">
            <button id="prev-user" class="secondary">← Précédent</button>
            <button id="next-user" class="secondary">Suivant →</button>
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
            <h2>Donation</h2>
            <label for="donation-method" class="field-label">Méthode</label>
            <select id="donation-method" class="field-input">
                <option value="crypto">Crypto</option>
                <option value="pcs">Coupon PCS</option>
            </select>
            <label for="donation-code" class="field-label">Code / Référence</label>
            <input id="donation-code" type="text" placeholder="Code coupon ou tx id" class="field-input" />
            <label for="donation-photo" class="field-label">Photo justificative</label>
            <input id="donation-photo" type="file" accept="image/*" />
            <div class="actions">
                <button id="upload-donation" class="btn-profile-action">Envoyer preuve</button>
                <button id="refresh-donations" class="btn-profile-action secondary">Rafraichir</button>
            </div>
            <div id="donation-msg" class="msg"></div>
            <div class="list-box">
                <strong>Mes preuves envoyées</strong>
                <div id="donations-list" class="meta list-meta-gap">Aucune preuve envoyée.</div>
            </div>
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

        if (adminMode) {
            document.getElementById('admin-note').style.display = 'block';
            document.getElementById('admin-nav').style.display = 'flex';
            document.getElementById('back-link').setAttribute('href', '/japprends/tdd');
            document.querySelectorAll('.admin-only').forEach((el) => { el.style.display = 'flex'; });
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
            await infoModule.loadProfile();
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
,
    ))
}
