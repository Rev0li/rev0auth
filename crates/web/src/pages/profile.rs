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
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Profil membre</h1>
            <p class="meta">Ici tu retrouves toutes tes infos. You can update or delete your account anytime.</p>
            <div id="admin-note" class="admin-note">Mode admin actif: tu modifies le profil complet de cet utilisateur.</div>
            <div id="admin-nav" class="admin-nav">
                <button id="prev-user" class="secondary">User precedent</button>
                <button id="next-user" class="secondary">User suivant</button>
                <span id="admin-nav-meta" class="meta"></span>
            </div>
            <div class="actions">
                <a class="btn secondary" id="back-link" href="/members/dashboard">Retour dashboard</a>
            </div>
        </article>

        <article class="card">
            <h2>Infos compte</h2>
            <p class="meta"><strong>Pseudo:</strong> <span id="info-pseudo">--</span></p>
            <p class="meta"><strong>Role:</strong> <span id="info-role">--</span></p>
            <p class="meta admin-only" id="status-row"><strong>Status:</strong> <span id="info-status">--</span></p>
            <p class="meta admin-only" id="status-emoji-row"><strong>Smiley:</strong> <span id="info-status-emoji">--</span></p>
            <p class="meta admin-only" id="commentary-row"><strong>Commentary:</strong> <span id="info-commentary">--</span></p>
            <p class="meta"><strong>Created at:</strong> <span id="info-created">--</span></p>
            <p class="meta"><strong>Avatar:</strong> <span id="info-avatar">--</span></p>
        </article>

        <article class="card">
            <h2>Profil editable</h2>
            <label for="bio">Presentation</label>
            <textarea id="bio" placeholder="Qui es-tu, ce que tu fais, ce que tu veux partager..."></textarea>

            <label for="commentary" class="admin-only">Commentary</label>
            <textarea id="commentary" class="admin-only" placeholder="Question, idee, demande d'amelioration..."></textarea>

            <div class="actions">
                <button id="save-profile" class="primary">Sauver profil</button>
            </div>
            <div id="profile-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Avatar</h2>
            <img id="avatar-preview" class="avatar-preview" alt="Avatar preview" />
            <div id="avatar-state" class="meta">Aucun avatar pour le moment.</div>
            <label for="avatar">Image</label>
            <input id="avatar" type="file" accept="image/*" />
            <div class="actions">
                <button id="upload-avatar" class="primary">Uploader avatar</button>
                <button id="delete-avatar" class="secondary danger-outline">Supprimer avatar</button>
            </div>
            <div id="avatar-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Mot de passe</h2>
            <label for="current-password">Mot de passe actuel</label>
            <input id="current-password" type="password" placeholder="Actuel" />

            <label for="new-password">Nouveau mot de passe</label>
            <input id="new-password" type="password" placeholder="Nouveau" />

            <div class="actions">
                <button id="save-password" class="primary">Mettre a jour le mot de passe</button>
            </div>
            <div id="password-msg" class="msg"></div>
        </article>

        <article class="card" id="member-messages-card">
            <h2>Messages membres</h2>
            <p class="meta meta-topless">Tous les messages sont envoyes automatiquement a l'admin.</p>

            <label for="msg-subject">Sujet</label>
            <input id="msg-subject" type="text" placeholder="Sujet du message" />

            <label for="msg-body">Message</label>
            <textarea id="msg-body" placeholder="Ton message..."></textarea>

            <div class="actions">
                <button id="send-message" class="primary">Envoyer message</button>
                <button id="refresh-messages" class="secondary">Rafraichir</button>
            </div>
            <div id="messages-msg" class="msg"></div>

            <div class="list-box">
                <strong>Boite de reception</strong>
                <div id="messages-inbox" class="meta list-meta-gap">Aucun message.</div>
            </div>
            <div class="list-box">
                <strong>Messages envoyes</strong>
                <div id="messages-sent" class="meta list-meta-gap">Aucun envoi.</div>
            </div>
        </article>

        <article class="card" id="donation-card">
            <h2>Donation (Crypto / Coupon PCS)</h2>
            <label for="donation-method">Methode</label>
            <select id="donation-method">
                <option value="crypto">Crypto</option>
                <option value="pcs">Coupon PCS</option>
            </select>

            <label for="donation-code">Code / Reference</label>
            <input id="donation-code" type="text" placeholder="Code coupon ou tx id" />

            <label for="donation-photo">Photo justificative</label>
            <input id="donation-photo" type="file" accept="image/*" />

            <div class="actions">
                <button id="upload-donation" class="primary">Envoyer preuve donation</button>
                <button id="refresh-donations" class="secondary">Rafraichir</button>
            </div>
            <div id="donation-msg" class="msg"></div>

            <div class="list-box">
                <strong>Mes preuves envoyees</strong>
                <div id="donations-list" class="meta list-meta-gap">Aucune preuve envoyee.</div>
            </div>
        </article>

        <article class="card">
            <h2>Danger zone</h2>
            <p class="meta">Tu peux supprimer ton compte a tout moment. Cette action est irreversible.</p>
            <div class="actions">
                <button id="delete-account" class="secondary danger-outline">Supprimer mon compte</button>
            </div>
            <div id="delete-msg" class="msg"></div>
        </article>
    </main>

    <script>
        %%COMMON_JS_UTILS%%
        %%PROFILE_INFO_MODULE%%
        %%PROFILE_EDIT_MODULE%%
        %%PROFILE_AVATAR_MODULE%%
        %%PROFILE_PASSWORD_MODULE%%
        %%PROFILE_MESSAGES_MODULE%%
        %%PROFILE_DONATIONS_MODULE%%
        %%PROFILE_ADMIN_NAVIGATOR_MODULE%%
        %%PROFILE_ACCOUNT_DELETION_MODULE%%

        const params = new URLSearchParams(window.location.search);
        const adminMode = params.get('admin') === '1';
        const queryPseudo = (params.get('pseudo') || '').trim();
        const localPseudo = localStorage.getItem('logged_pseudo');
        const pseudo = adminMode ? (queryPseudo || localPseudo || '') : (localPseudo || '');

        if (!pseudo) {
            window.location.href = '/';
        }

        if (adminMode) {
            const note = document.getElementById('admin-note');
            note.style.display = 'block';
            document.getElementById('admin-nav').style.display = 'flex';
            document.getElementById('back-link').setAttribute('href', '/dashboard');
            document.querySelectorAll('.admin-only').forEach((el) => {
                el.style.display = 'block';
            });
            document.getElementById('member-messages-card').style.display = 'none';
            document.getElementById('donation-card').style.display = 'none';
        }

        async function ensureAdminSession() {
            if (!adminMode) return true;

            try {
                const res = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                if (res.ok) return true;
            } catch (_err) {
                // Ask password below as fallback.
            }

            const password = window.prompt('Session admin expiree. Entre le mot de passe admin:');
            if (!password) return false;

            try {
                const loginRes = await fetch('/japprends/login', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ password })
                });
                if (!loginRes.ok) return false;

                const checkRes = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                return checkRes.ok;
            } catch (_err) {
                return false;
            }
        }

        const infoModule = createProfileInfoModule({ pseudo, adminMode });
        const editModule = createProfileEditModule({ pseudo });
        const avatarModule = createProfileAvatarModule({ pseudo });
        const passwordModule = createProfilePasswordModule({ pseudo, adminMode });
        const messagesModule = createProfileMessagesModule({ pseudo, adminMode });
        const donationsModule = createProfileDonationsModule({ pseudo, adminMode });
        const adminNavigatorModule = createProfileAdminNavigatorModule({ pseudo, adminMode });
        const accountDeletionModule = createProfileAccountDeletionModule({ pseudo, adminMode });

        accountDeletionModule.setAdminNavigatorModule(adminNavigatorModule);

        (async () => {
            if (adminMode) {
                const ok = await ensureAdminSession();
                if (!ok) {
                    alert('Authentification admin requise pour ouvrir ce profil.');
                    window.location.href = '/japprends/login';
                    return;
                }
                await adminNavigatorModule.loadAdminUsersNavigator();
            }
            await infoModule.loadProfile();
            await editModule.loadProfileData();
            await avatarModule.loadAvatarState();
            if (!adminMode) {
                await messagesModule.loadMessages();
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
