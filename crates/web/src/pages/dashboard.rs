use axum::response::Html;

use super::dashboard_page_assembly;

use crate::admin_pseudo_from_env;

pub async fn dashboard() -> Html<String> {
    let admin_pseudo = admin_pseudo_from_env();
    let template = r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Dashboard ALL Include</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRONTEND_SHARED_CSS%%
        %%DASHBOARD_PAGE_STYLES%%
    </style>
</head>
<body>
    <main class="wrap">
        <header class="header">
            <h1>Dashboard ALL Include</h1>
            <div class="chip" id="last-check">Derniere verification: --</div>
        </header>

        <section class="row onboarding-panel" id="onboarding-panel">
            <strong>Onboarding initial</strong>
            <div class="mini">Change ton mot de passe temporaire, puis laisse un message de présentation pour l'admin.</div>
            <div class="form-group form-group-top">
                <label for="onboarding-current-password">Mot de passe temporaire</label>
                <input type="password" id="onboarding-current-password" placeholder="mot de passe temporaire" />
            </div>
            <div class="form-group">
                <label for="onboarding-new-password">Nouveau mot de passe</label>
                <input type="password" id="onboarding-new-password" placeholder="nouveau mot de passe" />
            </div>
            <div class="form-group">
                <label for="onboarding-message">Message de présentation</label>
                <textarea id="onboarding-message" placeholder="Présente-toi, explique pourquoi tu es là..."></textarea>
            </div>
            <div class="actions">
                <button class="btn-small grant" id="onboarding-submit">Valider l'onboarding</button>
            </div>
            <div id="onboarding-msg" class="onboarding-msg"></div>
        </section>

        <nav class="tabs">
            <button class="tab-btn active" data-tab-btn="overview">Overview</button>
            <button class="tab-btn" data-tab-btn="admin">Admin</button>
            <button class="tab-btn" data-tab-btn="user">User</button>
            <button class="tab-btn" data-tab-btn="theme">Theme</button>
            <button class="tab-btn" data-tab-btn="system">System</button>
            <button class="tab-btn" data-tab-btn="security">Security</button>
        </nav>

        <section class="tab-page active" id="tab-overview">
            <div class="grid">
                <article class="card">
                    <div class="label">Cote Admin</div>
                    <div class="state" id="admin-state">Chargement...</div>
                    <div class="meta">Sante du panneau admin et endpoints de supervision.</div>
                </article>

                <article class="card">
                    <div class="label">Cote User</div>
                    <div class="state" id="user-state">Chargement...</div>
                    <div class="meta">Disponibilite du flux utilisateur et pages membres.</div>
                </article>

                <article class="card">
                    <div class="label">API Auth</div>
                    <div class="state" id="api-state">Chargement...</div>
                    <div class="meta">Connectivite web -> API Rust (upstream configurable).</div>
                </article>
            </div>

            <section class="row">
                <strong>Monitoring live:</strong> refresh automatique toutes les 4 secondes.
                <div class="mini">
                    Sprint courant: <code id="sprint">--</code>
                    | Tests API: <code id="tests-total">--</code>
                    | Demandes en attente: <code id="pending-signups">--</code>
                </div>
                <div class="actions">
                    <button class="tab-btn" id="launch-tests-now">Launch test now</button>
                </div>
                <div id="tests-history" class="tests-history"></div>
            </section>

            <section class="row">
                <strong>Fun stats (overview)</strong>
                <div class="stats-strip">
                    <div class="stat-box">
                        <div class="stat-k">Couverture services</div>
                        <div class="stat-v" id="fun-service-coverage">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Demandes acces</div>
                        <div class="stat-v" id="fun-access-requests">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Activation users</div>
                        <div class="stat-v" id="fun-active-ratio">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Health chain</div>
                        <div class="stat-v" id="fun-chain-ok">--</div>
                    </div>
                </div>
            </section>
        </section>

        <section class="tab-page" id="tab-admin">
            <div class="grid">
                <article class="card">
                    <div class="label">Admin Core</div>
                    <div class="state" id="admin-state-2">Chargement...</div>
                    <div class="mini">Endpoint controle: <code>/japprends/ping</code></div>
                </article>
                <article class="card">
                    <div class="label">SLO</div>
                    <div class="mini">Objectif: uptime panel admin >= 99.9%</div>
                </article>
                <article class="card">
                    <div class="label">Pilotage</div>
                    <div class="mini">Vue orientee moderation, acces et operations quotidiennes.</div>
                </article>
            </div>
            <div class="row">
                <strong>Admin utils (stats live)</strong>
                <div class="stats-strip">
                    <div class="stat-box">
                        <div class="stat-k">Users total</div>
                        <div class="stat-v" id="stat-users-total">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Users actifs</div>
                        <div class="stat-v" id="stat-users-active">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Demandes pending</div>
                        <div class="stat-v" id="stat-signups-pending">--</div>
                    </div>
                    <div class="stat-box">
                        <div class="stat-k">Dernier run tests</div>
                        <div class="stat-v" id="stat-tests-last">--</div>
                    </div>
                </div>
                <div class="mini">Idees utiles: ajouter uptime 24h, latence moyenne API et taux d'erreur login.</div>
            </div>

            <div class="row">
                <strong>Messages membres (pseudo chat)</strong>
                <div class="mini">Vue par conversation avec chaque membre, réponse persistante dans l'historique.</div>
                <div class="chat-admin-wrap">
                    <div class="chat-admin-layout">
                        <aside id="admin-thread-list" class="chat-admin-threads">Chargement...</aside>
                        <div class="chat-admin-panel">
                            <div id="admin-messages" class="chat-admin-history">Sélectionne une conversation...</div>
                            <div class="chat-admin-compose">
                                <label for="admin-reply-to">Destinataire</label>
                                <input id="admin-reply-to" placeholder="pseudo membre" />
                                <label for="admin-reply-subject">Sujet</label>
                                <input id="admin-reply-subject" placeholder="Re: ..." />
                                <label for="admin-reply-body">Message</label>
                                <textarea id="admin-reply-body" placeholder="Ta reponse..."></textarea>
                                <div class="actions actions-no-top">
                                    <button class="btn-small grant" id="admin-reply-send">Envoyer reponse</button>
                                </div>
                                <div id="admin-reply-msg" class="chat-admin-msg"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="row">
                <strong>Preuves donations (crypto / PCS)</strong>
                <div class="mini">Validation manuelle des preuves avec photo + code.</div>
                <div id="admin-donations" class="mini">Chargement...</div>
            </div>
        </section>

        <section class="tab-page" id="tab-user">
            <div class="row">
                <strong>Creation utilisateur</strong>
                <div class="mini">Desactivee: comptes auto-crees via inscription publique pour eviter les collisions.</div>
            </div>

            <div class="row">
                <strong>VALIDATION INSCRIPTIONS</strong>
                <div class="mini">Validation manuelle des demandes user.</div>
                <div id="admin-signup-queue" class="mini">Chargement...</div>
            </div>

            <div class="row">
                <strong>Utilisateurs (Comptes actifs)</strong>
                <div id="users-list" class="mini">Chargement...</div>
            </div>

            <div class="grid">
                <article class="card">
                    <div class="label">User Core</div>
                    <div class="state" id="user-state-2">Chargement...</div>
                    <div class="mini">Endpoint controle: <code>/user/ping</code></div>
                </article>
                <article class="card">
                    <div class="label">EXPERIENCE USER</div>
                    <ul>
                        <li>Login stable</li>
                        <li>Refresh token fiable</li>
                        <li>Acces membre selon role</li>
                    </ul>
                </article>
                <article class="card">
                    <div class="label">UX STATUS</div>
                    <div class="mini">Etat des services critiques visibles en permanence.</div>
                </article>
            </div>
        </section>

        <section class="tab-page" id="tab-system">
            <div class="row">
                <strong>Timeline checks (dernieres 10):</strong>
                <ul id="timeline" class="timeline"></ul>
            </div>
            <div class="row">
                <strong>Verification de chaine (leger, maillons critiques)</strong>
                <div class="mini">Web -> API, Admin ping, User ping, registry endpoints.</div>
                <div id="chain-checks" class="chain-grid">Chargement...</div>
            </div>
            <div class="row">
                <strong>Tous les endpoints (visuel sante, non cliquable)</strong>
                <div class="mini">Cette vue ne navigue pas vers les pages. Elle indique juste si chaque scope est considere OK.</div>
                <div id="endpoints-system-list" class="endpoint-grid">Chargement...</div>
            </div>
        </section>

        <section class="tab-page" id="tab-theme">
            <div class="row">
                <strong>Theme editor (global frontend)</strong>
                <div class="mini">Ces tokens sont sauvegardes dans le navigateur admin et appliques sur tout le frontend via <code>rev0auth_theme</code>.</div>
                <div class="theme-editor-grid">
                    <div>
                        <label for="theme-preset-name" class="field-label">Nom du preset</label>
                        <input id="theme-preset-name" class="field-input" placeholder="ex: ocean-soft" />
                    </div>
                    <div>
                        <label for="theme-preset-select" class="field-label">Presets enregistres</label>
                        <select id="theme-preset-select" class="field-select"></select>
                    </div>
                </div>
                <div class="actions">
                    <button class="btn-small grant" id="theme-preset-save">Sauver preset</button>
                    <button class="btn-small" id="theme-preset-update">Mettre a jour preset selectionne</button>
                    <button class="btn-small" id="theme-preset-apply">Appliquer preset</button>
                    <button class="btn-small danger" id="theme-preset-delete">Supprimer preset</button>
                </div>
                <div id="theme-editor-list"></div>
                <div class="actions">
                    <button class="btn-small" id="theme-preview-apply">Preview page courante</button>
                    <button class="btn-small" id="theme-preview-reset">Reset preview</button>
                    <button class="btn-small grant" id="theme-editor-save">Sauvegarder theme</button>
                    <button class="btn-small" id="theme-editor-export-btn">Exporter JSON</button>
                    <button class="btn-small" id="theme-editor-import-btn">Importer JSON</button>
                    <button class="btn-small danger" id="theme-editor-reset">Reset theme</button>
                </div>
                <textarea id="theme-editor-export" class="field-textarea" placeholder="JSON export theme..."></textarea>
                <div id="theme-editor-msg" class="mini mini-top"></div>
            </div>
            <div class="row">
                <strong>Preview composants (live)</strong>
                <div class="mini">Ajuste les tokens ci-dessus: cette vitrine se met a jour instantanement et represente les elements principaux du site.</div>
                <div class="grid preview-grid">
                    <article class="card preview-card">
                        <div class="label">Card / Typography</div>
                        <h3 class="preview-title">Titre exemple</h3>
                        <div class="meta">Texte secondaire pour verifier le contraste et la lisibilite globale.</div>
                    </article>
                    <article class="card preview-card">
                        <div class="label">Buttons</div>
                        <div class="actions actions-tight">
                            <button class="btn-small primary" type="button">Primary</button>
                            <button class="btn-small secondary" type="button">Secondary</button>
                            <button class="btn-small danger" type="button">Danger</button>
                        </div>
                    </article>
                    <article class="card preview-card">
                        <div class="label">Feedback</div>
                        <div class="mini ok preview-feedback">Message succes</div>
                        <div class="mini down preview-feedback">Message erreur</div>
                    </article>
                    <article class="card preview-card">
                        <div class="label">Form controls</div>
                        <label for="theme-preview-input" class="preview-input-label">Input</label>
                        <input id="theme-preview-input" class="field-input" value="Preview value" />
                    </article>
                </div>
            </div>
        </section>

        <section class="tab-page" id="tab-security">
            <div class="row">
                <strong>YubiKey / WebAuthn FIDO2</strong>
                <div class="mini">Authentification forte par cle materielle. Une fois enregistree, la cle est exigee a chaque connexion admin.</div>

                <div id="webauthn-status-block" class="mini" style="margin-top:12px;">
                    Chargement...
                </div>

                <div class="actions" style="margin-top:14px;">
                    <button class="btn-small grant" id="webauthn-register-btn">Enregistrer une cle YubiKey</button>
                    <button class="btn-small danger" id="webauthn-remove-btn" style="display:none">Retirer la cle</button>
                </div>

                <div id="webauthn-register-msg" class="mini" style="display:none;margin-top:10px;"></div>

                <div id="webauthn-credential-export" style="display:none;margin-top:14px;">
                    <div class="mini">
                        Copie cette valeur dans ton <code>.env</code> comme <code>ADMIN_WEBAUTHN_CREDENTIAL</code>
                        pour que la cle persiste apres redemarrage:
                    </div>
                    <textarea id="webauthn-credential-json"
                        style="width:100%;min-height:90px;margin-top:8px;font-family:ui-monospace,monospace;font-size:0.75rem;resize:vertical;"
                        readonly></textarea>
                    <div class="actions actions-tight">
                        <button class="btn-small" id="webauthn-copy-btn">Copier</button>
                    </div>
                </div>
            </div>

            <div class="row">
                <strong>Config WebAuthn</strong>
                <div class="mini">
                    RP ID: <code id="webauthn-rp-id">--</code> &nbsp;|&nbsp;
                    Origin: <code id="webauthn-rp-origin">--</code>
                </div>
                <div class="mini" style="margin-top:6px;">
                    Dev: <code>WEBAUTHN_RP_ID=localhost</code> + <code>WEBAUTHN_RP_ORIGIN=http://localhost:3000</code> (defaut).<br/>
                    Prod: remplace par ton domaine + <code>https://</code>.
                    WebAuthn necessite HTTPS en production.
                </div>
            </div>
        </section>

    </main>

    <script>
        const adminPseudo = "%%ADMIN_PSEUDO%%";
        const currentPseudo = localStorage.getItem('logged_pseudo') || '';
        const monitorState = { adminOk: false, userOk: false, apiOk: false };
        const dashboardStats = { users: [], pendingSignups: 0, lastRun: null, endpoints: [] };
        const onboardingState = { bio: '' };
        const adminChatState = {
            messages: [],
            selectedThread: localStorage.getItem('dashboard_chat_thread') || ''
        };
        const chainState = {
            webToApi: false,
            adminPing: false,
            userPing: false,
            endpointRegistry: false
        };

        %%COMMON_JS_UTILS%%
        %%DASHBOARD_CHAT_MODULE%%
        %%DASHBOARD_USERS_MODULE%%
        %%DASHBOARD_DONATIONS_MODULE%%
        %%DASHBOARD_TESTING_MODULE%%
        %%DASHBOARD_QUEUE_MODULE%%
        %%DASHBOARD_STATUS_MODULE%%
        %%DASHBOARD_THEME_EDITOR_MODULE%%

        const adminChatModule = createDashboardChatModule({
            adminPseudo,
            adminChatState,
        });

        const {
            setAdminReplyMsg,
            startAdminReply,
            sendAdminReply,
            loadAdminMessages,
        } = adminChatModule;

        const themeEditorModule = createDashboardThemeEditorModule();

        function copyTempPassword(password) {
            if (!password) return;
            if (navigator.clipboard && navigator.clipboard.writeText) {
                navigator.clipboard.writeText(password).catch(() => {});
            }
        }

        function setOnboardingVisible(visible) {
            const panel = document.getElementById('onboarding-panel');
            if (panel) panel.style.display = visible ? 'block' : 'none';
        }

        async function loadOnboardingProfile() {
            if (!currentPseudo) return;
            try {
                const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
                const data = await res.json();
                onboardingState.bio = data && typeof data.bio === 'string' ? data.bio : '';
                const messageInput = document.getElementById('onboarding-message');
                if (messageInput && !messageInput.value) {
                    messageInput.value = data && typeof data.commentary === 'string' ? data.commentary : '';
                }
            } catch (_err) {
                onboardingState.bio = '';
            }
        }

        async function submitOnboarding() {
            const currentPassword = document.getElementById('onboarding-current-password').value.trim();
            const newPassword = document.getElementById('onboarding-new-password').value.trim();
            const message = document.getElementById('onboarding-message').value.trim();
            const output = document.getElementById('onboarding-msg');

            if (!currentPseudo) {
                output.style.color = '#dc4f2f';
                output.textContent = 'Pseudo manquant en session.';
                output.style.display = 'block';
                return;
            }
            if (!currentPassword || !newPassword) {
                output.style.color = '#dc4f2f';
                output.textContent = 'Remplis le mot de passe temporaire et le nouveau mot de passe.';
                output.style.display = 'block';
                return;
            }

            try {
                const passwordRes = await fetch('/members/password', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo: currentPseudo,
                        current_password: currentPassword,
                        new_password: newPassword
                    })
                });
                const passwordData = await passwordRes.json();
                if (!passwordData.ok) {
                    output.style.color = '#dc4f2f';
                    output.textContent = passwordData.message || 'Impossible de changer le mot de passe.';
                    output.style.display = 'block';
                    return;
                }

                const profileRes = await fetch('/members/profile/data', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo: currentPseudo,
                        bio: onboardingState.bio,
                        commentary: message || null
                    })
                });
                const profileData = await profileRes.json();
                if (!profileData.ok) {
                    output.style.color = '#dc4f2f';
                    output.textContent = profileData.message || 'Mot de passe change, mais le message n\'a pas pu etre enregistre.';
                    output.style.display = 'block';
                    return;
                }

                output.style.color = '#0d9b73';
                output.textContent = 'Onboarding termine. Ton compte est pret.';
                output.style.display = 'block';
                localStorage.removeItem('needs_onboarding');
                setOnboardingVisible(false);
            } catch (err) {
                output.style.color = '#dc4f2f';
                output.textContent = 'Erreur: ' + err.message;
                output.style.display = 'block';
            }
        }

        function paint(el, ok, label) {
            el.textContent = ok ? label + ' OK' : label + ' DOWN';
            el.className = 'state ' + (ok ? 'ok' : 'down');
        }

        function endpointScopeOk(ep) {
            if (ep.scope === 'admin') return monitorState.adminOk && monitorState.apiOk;
            if (ep.scope === 'member') return monitorState.userOk && monitorState.apiOk;
            return true;
        }

        function renderEndpointsSystem() {
            const panel = document.getElementById('endpoints-system-list');
            if (!panel) return;
            const data = dashboardStats.endpoints;
            if (!Array.isArray(data) || data.length === 0) {
                panel.textContent = 'Aucun endpoint trouve.';
                return;
            }

            panel.innerHTML = data.map((ep) => {
                const ok = endpointScopeOk(ep);
                const badge = ok
                    ? '<span class="badge-ok">OK</span>'
                    : '<span class="badge-ko">KO</span>';

                return '<div class="endpoint-item">'
                    + '<div><strong>' + ep.method + '</strong> ' + ep.path
                    + '<div class="endpoint-meta">scope: ' + ep.scope + '</div></div>'
                    + badge
                    + '</div>';
            }).join('');
        }

        function renderChainChecks() {
            const panel = document.getElementById('chain-checks');
            if (!panel) return;

            const items = [
                { key: 'webToApi', label: 'Web -> API health' },
                { key: 'adminPing', label: 'Admin ping endpoint' },
                { key: 'userPing', label: 'User ping endpoint' },
                { key: 'endpointRegistry', label: 'Endpoint registry' }
            ];

            panel.innerHTML = items.map((item) => {
                const ok = !!chainState[item.key];
                const badge = ok
                    ? '<span class="badge-ok">OK</span>'
                    : '<span class="badge-ko">KO</span>';
                return '<div class="endpoint-item"><div><strong>' + item.label + '</strong></div>' + badge + '</div>';
            }).join('');
        }

        function renderAdminStats() {
            const users = Array.isArray(dashboardStats.users) ? dashboardStats.users : [];
            const pending = Number(dashboardStats.pendingSignups || 0);
            const lastRun = dashboardStats.lastRun;

            const activeCount = users.filter((u) => String(u.status || '').toLowerCase() === 'actif').length;
            const lastRunLabel = lastRun ? (lastRun.passed + '/' + lastRun.total) : '--';
            const totalServices = users.length * 3;
            const grantedServices = users.reduce((sum, u) => {
                return sum
                    + (u.access_github ? 1 : 0)
                    + (u.access_jellyfin ? 1 : 0)
                    + (u.access_songsurf ? 1 : 0);
            }, 0);
            const requestedServices = users.reduce((sum, u) => {
                return sum
                    + (u.request_github ? 1 : 0)
                    + (u.request_jellyfin ? 1 : 0)
                    + (u.request_songsurf ? 1 : 0);
            }, 0);
            const activeRatio = users.length > 0
                ? Math.round((activeCount * 100) / users.length) + '%'
                : '--';
            const chainOk = chainState.webToApi && chainState.adminPing && chainState.userPing && chainState.endpointRegistry;

            document.getElementById('stat-users-total').textContent = String(users.length);
            document.getElementById('stat-users-active').textContent = String(activeCount);
            document.getElementById('stat-signups-pending').textContent = String(pending);
            document.getElementById('stat-tests-last').textContent = lastRunLabel;

            document.getElementById('fun-service-coverage').textContent = totalServices > 0
                ? (grantedServices + '/' + totalServices)
                : '--';
            document.getElementById('fun-access-requests').textContent = String(requestedServices);
            document.getElementById('fun-active-ratio').textContent = activeRatio;
            document.getElementById('fun-chain-ok').textContent = chainOk ? 'FULL OK' : 'CHECK';
        }

        function pushTimeline(message) {
            const list = document.getElementById('timeline');
            if (!list) return;
            const item = document.createElement('li');
            item.textContent = message;
            list.prepend(item);
            while (list.children.length > 10) {
                list.removeChild(list.lastElementChild);
            }
        }

        function requestRow(req) {
            const dt = new Date(req.created_at_epoch * 1000).toLocaleTimeString();
            const canAct = req.status === 'pending';
            const actions = canAct
                ? '<button data-act="approve" data-id="' + req.id + '">Approuver</button> '
                    + '<button data-act="reject" data-id="' + req.id + '">Rejeter</button>'
                : '';

            return '<div class="request-row">'
                + '<strong>#' + req.id + ' - ' + req.pseudo + '</strong> [' + req.status + ']'
                + '<br>referral: ' + req.referral
                + '<br>cree a: ' + dt
                + (actions ? '<div class="request-actions">' + actions + '</div>' : '')
                + '</div>';
        }

        async function loadAdminSignupQueue() {
            const panel = document.getElementById('admin-signup-queue');
            if (!panel) return;

            const res = await fetch('/japprends/signup-requests', { cache: 'no-store' });
            const list = await res.json();
            dashboardStats.pendingSignups = Array.isArray(list)
                ? list.filter((r) => r.status === 'pending').length
                : 0;
            renderAdminStats();
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucune demande pour le moment.';
                return;
            }

            panel.innerHTML = list.slice().reverse().map(requestRow).join('');
            panel.querySelectorAll('button[data-act]').forEach((btn) => {
                btn.addEventListener('click', async () => {
                    const id = btn.getAttribute('data-id');
                    const act = btn.getAttribute('data-act');
                    const res = await fetch('/japprends/signup-requests/' + id + '/' + act, { method: 'POST' });
                    const data = await res.json();
                    if (data && data.ok && data.temp_password) {
                        copyTempPassword(data.temp_password);
                        alert('Mot de passe temporaire: ' + data.temp_password);
                    }
                    await loadAdminSignupQueue();
                    await refreshStatus();
                });
            });
        }

        async function refreshStatus() {
            try {
                const [basicRes, allRes, adminRes, userRes] = await Promise.all([
                    fetch('/status', { cache: 'no-store' }),
                    fetch('/status/all', { cache: 'no-store' }),
                    fetch('/japprends/ping', { cache: 'no-store' }),
                    fetch('/user/ping', { cache: 'no-store' })
                ]);

                const data = await basicRes.json();
                const all = await allRes.json();
                const admin = await adminRes.json();
                const user = await userRes.json();

                const adminOk = data.admin_ok && admin.status === 'ok';
                const userOk = data.user_ok && user.status === 'ok';

                monitorState.adminOk = adminOk;
                monitorState.userOk = userOk;
                monitorState.apiOk = !!data.api_ok;

                chainState.webToApi = !!data.api_ok;
                chainState.adminPing = admin.status === 'ok';
                chainState.userPing = user.status === 'ok';

                paint(document.getElementById('admin-state'), adminOk, 'ADMIN');
                paint(document.getElementById('admin-state-2'), adminOk, 'ADMIN');
                paint(document.getElementById('user-state'), userOk, 'USER');
                paint(document.getElementById('user-state-2'), userOk, 'USER');
                paint(document.getElementById('api-state'), data.api_ok, 'API');
                renderChainChecks();
                renderEndpointsSystem();
                renderAdminStats();

                document.getElementById('sprint').textContent = all.sprint;
                document.getElementById('tests-total').textContent = String(all.tests_api_total);
                document.getElementById('pending-signups').textContent = String(all.signup_requests_pending);

                const dt = new Date(data.checked_at_epoch * 1000);
                document.getElementById('last-check').textContent =
                    'Derniere verification: ' + dt.toLocaleTimeString();

                pushTimeline(
                    dt.toLocaleTimeString() +
                    ' | ADMIN=' + (adminOk ? 'OK' : 'DOWN') +
                    ' | USER=' + (userOk ? 'OK' : 'DOWN') +
                    ' | API=' + (data.api_ok ? 'OK' : 'DOWN')
                );
            } catch (_err) {
                paint(document.getElementById('admin-state'), false, 'ADMIN');
                paint(document.getElementById('admin-state-2'), false, 'ADMIN');
                paint(document.getElementById('user-state'), false, 'USER');
                paint(document.getElementById('user-state-2'), false, 'USER');
                paint(document.getElementById('api-state'), false, 'API');
                chainState.webToApi = false;
                chainState.adminPing = false;
                chainState.userPing = false;
                renderChainChecks();
                renderAdminStats();
                pushTimeline(new Date().toLocaleTimeString() + ' | erreur de monitoring');
            }
        }

        async function loadAdminDonations() {
            const panel = document.getElementById('admin-donations');
            if (!panel) return;

            try {
                const res = await fetch('/japprends/donations', { cache: 'no-store' });
                const list = await res.json();
                if (!Array.isArray(list) || list.length === 0) {
                    panel.textContent = 'Aucune preuve donation pour le moment.';
                    return;
                }

                panel.innerHTML = list.slice().reverse().map((row) => {
                    const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                    const state = !row.reviewed ? 'pending' : (row.approved ? 'approuvee' : 'refusee');
                    const actions = row.reviewed
                        ? ''
                        : '<button data-donation-review="1" data-id="' + row.id + '" data-approved="true">Valider</button> '
                            + '<button data-donation-review="1" data-id="' + row.id + '" data-approved="false">Refuser</button>';
                    return '<div class="donation-row">'
                        + '<strong>#' + row.id + '</strong> • ' + row.pseudo + ' • ' + row.method
                        + '<br>code: ' + row.code
                        + '<br>etat: ' + state
                        + '<br>date: ' + dt
                        + '<div class="donation-actions">'
                        + '<a class="btn-small warn" target="_blank" rel="noopener noreferrer" href="/members/donations/proof/' + row.id + '/photo">Voir photo</a>'
                        + actions
                        + '</div>'
                        + '</div>';
                }).join('');

                panel.querySelectorAll('button[data-donation-review="1"]').forEach((btn) => {
                    btn.addEventListener('click', async () => {
                        const id = btn.getAttribute('data-id');
                        const approved = btn.getAttribute('data-approved') === 'true';
                        await fetch('/japprends/donations/' + id + '/review', {
                            method: 'POST',
                            headers: { 'content-type': 'application/json' },
                            body: JSON.stringify({ approved })
                        });
                        await loadAdminDonations();
                    });
                });
            } catch (_err) {
                panel.textContent = 'Impossible de charger les donations.';
            }
        }

        function renderTestsHistory(runs) {
            const panel = document.getElementById('tests-history');
            if (!panel) return;
            if (!Array.isArray(runs) || runs.length === 0) {
                dashboardStats.lastRun = null;
                renderAdminStats();
                panel.innerHTML = '<div class="mini">Aucun test lance depuis le dashboard.</div>';
                return;
            }

            dashboardStats.lastRun = runs[0];
            renderAdminStats();

            panel.innerHTML = runs.slice(0, 12).map((run) => {
                const dt = new Date(run.executed_at_epoch * 1000).toLocaleString();
                const cases = Array.isArray(run.cases)
                    ? run.cases.map((c) => '<li>' + (c.ok ? 'OK' : 'KO') + ' - ' + c.name + ' - ' + c.detail + '</li>').join('')
                    : '';
                return '<div class="test-run">'
                    + '<div class="test-head">Run #' + run.run_id + ' - ' + run.passed + '/' + run.total + ' - ' + dt + '</div>'
                    + '<ul class="test-cases">' + cases + '</ul>'
                    + '</div>';
            }).join('');
        }

        async function loadTestsHistory() {
            try {
                const res = await fetch('/japprends/tests/history', { cache: 'no-store' });
                const data = await res.json();
                renderTestsHistory(data);
            } catch (_err) {
                const panel = document.getElementById('tests-history');
                if (panel) panel.innerHTML = '<div class="mini">Impossible de charger l\'historique des tests.</div>';
            }
        }

        async function launchTestsNow() {
            try {
                await fetch('/japprends/tests/launch', { method: 'POST' });
                await loadTestsHistory();
            } catch (_err) {
                const panel = document.getElementById('tests-history');
                if (panel) panel.innerHTML = '<div class="mini">Echec du lancement des tests.</div>';
            }
        }

        async function loadEndpoints() {
            const panel = document.getElementById('endpoints-system-list');
            if (!panel) return;

            try {
                const res = await fetch('/japprends/endpoints', { cache: 'no-store' });
                const data = await res.json();
                dashboardStats.endpoints = Array.isArray(data) ? data : [];
                chainState.endpointRegistry = Array.isArray(data) && data.length > 0;
                renderChainChecks();
                renderEndpointsSystem();
            } catch (_err) {
                chainState.endpointRegistry = false;
                renderChainChecks();
                panel.textContent = 'Erreur chargement endpoints.';
            }
        }

        function bindTabs() {
            const buttons = document.querySelectorAll('[data-tab-btn]');
            const pages = document.querySelectorAll('.tab-page');

            buttons.forEach((btn) => {
                btn.addEventListener('click', () => {
                    const tab = btn.getAttribute('data-tab-btn');
                    buttons.forEach((b) => b.classList.remove('active'));
                    pages.forEach((p) => p.classList.remove('active'));

                    btn.classList.add('active');
                    const page = document.getElementById('tab-' + tab);
                    if (page) page.classList.add('active');
                });
            });
        }

        async function loadUsers() {
            const panel = document.getElementById('users-list');
            if (!panel) return;

            const res = await fetch('/users', { cache: 'no-store' });
            const list = await res.json();
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucun utilisateur pour le moment.';
                return;
            }

            const html = list.map(user => {
                const dt = new Date(user.created_at_epoch * 1000).toLocaleDateString('fr-FR');
                let statusDisplay = '🔴 Inactif';
                if (user.status === 'actif') statusDisplay = '🟢 Actif';
                else if (user.status === 'occupe') statusDisplay = '🟡 Occupe';
                
                const role = user.role.charAt(0).toUpperCase() + user.role.slice(1);
                const reqBadges = [];
                if (user.request_github) reqBadges.push('GitHub');
                if (user.request_jellyfin) reqBadges.push('Jellyfin');
                if (user.request_songsurf) reqBadges.push('Songsurf');
                const reqLabel = reqBadges.length > 0
                    ? '<div class="user-meta user-request-badges">Demandes: ' + reqBadges.join(', ') + '</div>'
                    : '';

                const ghLabel = user.access_github ? 'GitHub ON' : 'GitHub OFF';
                const jfLabel = user.access_jellyfin ? 'Jellyfin ON' : 'Jellyfin OFF';
                const ssLabel = user.access_songsurf ? 'Songsurf ON' : 'Songsurf OFF';

                return '<div class="user-card clickable" onclick="openUserProfile(\'' + user.pseudo + '\')">'
                    + '<div class="user-info">'
                    + '<div class="user-name">' + user.pseudo + '</div>'
                    + '<div class="user-meta">' + role + ' • ' + dt + ' • ' + statusDisplay + '</div>'
                    + reqLabel
                    + '</div>'
                    + '<div class="user-actions">'
                    + '<button class="btn-small warn" onclick="event.stopPropagation(); openUserProfile(\'' + user.pseudo + '\')">Profil complet</button>'
                    + '<button class="btn-small ' + (user.access_github ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'github\', ' + (!user.access_github) + ')">' + ghLabel + '</button>'
                    + '<button class="btn-small ' + (user.access_jellyfin ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'jellyfin\', ' + (!user.access_jellyfin) + ')">' + jfLabel + '</button>'
                    + '<button class="btn-small ' + (user.access_songsurf ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'songsurf\', ' + (!user.access_songsurf) + ')">' + ssLabel + '</button>'
                    + '<button class="btn-small" onclick="event.stopPropagation(); deleteUser(\'' + user.pseudo + '\')">🗑 Supprimer</button>'
                    + '</div>'
                    + '</div>';
            }).join('');

            dashboardStats.users = list;
            renderAdminStats();
            panel.innerHTML = html;
        }

        async function toggleServiceAccess(pseudo, service, nextValue) {
            const payload = {};
            if (service === 'github') payload.access_github = !!nextValue;
            if (service === 'jellyfin') payload.access_jellyfin = !!nextValue;
            if (service === 'songsurf') payload.access_songsurf = !!nextValue;

            try {
                const res = await fetch('/japprends/users/' + pseudo, {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify(payload)
                });
                const data = await res.json();
                if (!data.ok) {
                    alert('Erreur: ' + data.message);
                    return;
                }
                await loadUsers();
            } catch (err) {
                alert('Erreur: ' + err.message);
            }
        }

        function openUserProfile(pseudo) {
            const target = '/members/profile?pseudo=' + encodeURIComponent(pseudo) + '&admin=1';
            window.location.href = target;
        }

        // User management functions
        async function createUser() {
            const pseudoInput = document.getElementById('new-pseudo');
            const msg = document.getElementById('create-msg');
            if (!pseudoInput || !msg) return;
            const pseudo = pseudoInput.value.trim();

            if (!pseudo) {
                msg.style.color = '#dc4f2f';
                msg.textContent = 'Entre un pseudo.';
                msg.style.display = 'block';
                return;
            }

            try {
                const res = await fetch('/japprends/users', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo })
                });
                const data = await res.json();

                if (data.ok) {
                    msg.style.color = '#0d9b73';
                    msg.textContent = '✓ ' + data.message + (data.temp_password ? ' Mot de passe: ' + data.temp_password : '');
                    if (data.temp_password) copyTempPassword(data.temp_password);
                    pseudoInput.value = '';
                    loadUsers();
                } else {
                    msg.style.color = '#dc4f2f';
                    msg.textContent = '✗ ' + data.message;
                }
                msg.style.display = 'block';
            } catch (err) {
                msg.style.color = '#dc4f2f';
                msg.textContent = 'Erreur: ' + err.message;
                msg.style.display = 'block';
            }
        }

        async function deleteUser(pseudo) {
            if (!confirm('Supprimer l\'utilisateur ' + pseudo + ' ?')) return;

            try {
                const res = await fetch('/japprends/users/' + pseudo, {
                    method: 'DELETE'
                });
                const data = await res.json();

                if (data.ok) {
                    loadUsers();
                } else {
                    alert('Erreur: ' + data.message);
                }
            } catch (err) {
                alert('Erreur: ' + err.message);
            }
        }

        const createUserBtn = document.getElementById('create-user-btn');
        if (createUserBtn) {
            createUserBtn.addEventListener('click', createUser);
        }
        document.getElementById('admin-reply-send').addEventListener('click', sendAdminReply);
        bindEnterToClick('admin-reply-to', 'admin-reply-send');
        bindEnterToClick('admin-reply-subject', 'admin-reply-send');
        document.getElementById('launch-tests-now').addEventListener('click', launchTestsNow);
        document.getElementById('onboarding-submit').addEventListener('click', submitOnboarding);
        bindEnterToClick('onboarding-current-password', 'onboarding-submit');
        bindEnterToClick('onboarding-new-password', 'onboarding-submit');

        // ---- WebAuthn helpers (registration) ----
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

        async function loadWebAuthnStatus() {
            try {
                const res = await fetch('/japprends/webauthn/status');
                const data = await res.json();
                const block = document.getElementById('webauthn-status-block');
                const removeBtn = document.getElementById('webauthn-remove-btn');

                if (data.registered) {
                    block.innerHTML = '<span style="color:var(--color-success);font-weight:600">● Cle YubiKey enregistree</span> — La cle sera exigee a chaque connexion admin.';
                    if (removeBtn) removeBtn.style.display = 'inline-flex';
                } else {
                    block.innerHTML = '<span style="color:var(--color-muted)">○ Aucune cle enregistree</span> — Connexion par mot de passe seul.';
                    if (removeBtn) removeBtn.style.display = 'none';
                }

                const rpId = document.getElementById('webauthn-rp-id');
                const rpOrigin = document.getElementById('webauthn-rp-origin');
                if (rpId) rpId.textContent = data.rp_id || '--';
                if (rpOrigin) rpOrigin.textContent = data.rp_origin || '--';
            } catch(e) {
                console.warn('webauthn status error', e);
            }
        }

        async function registerYubiKey() {
            const msg = document.getElementById('webauthn-register-msg');
            const exportDiv = document.getElementById('webauthn-credential-export');

            msg.textContent = 'Demarrage de l\'enregistrement...';
            msg.className = 'mini';
            msg.style.display = 'block';
            exportDiv.style.display = 'none';

            try {
                const startRes = await fetch('/japprends/webauthn/register/start');
                if (!startRes.ok) throw new Error('Erreur serveur start');
                const challengeJSON = await startRes.json();

                const options = challengeJSON.publicKey;
                options.challenge = base64urlToBuffer(options.challenge);
                options.user.id = base64urlToBuffer(options.user.id);
                if (options.excludeCredentials) {
                    options.excludeCredentials = options.excludeCredentials.map(c => ({ ...c, id: base64urlToBuffer(c.id) }));
                }

                msg.textContent = 'Touche ta cle YubiKey maintenant...';

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
                    msg.textContent = data.message;
                    msg.style.color = 'var(--color-success)';
                    if (data.credential_json) {
                        document.getElementById('webauthn-credential-json').value = data.credential_json;
                        exportDiv.style.display = 'block';
                    }
                    loadWebAuthnStatus();
                } else {
                    msg.textContent = 'Erreur: ' + data.message;
                    msg.style.color = 'var(--color-danger)';
                }
            } catch (err) {
                msg.textContent = 'Erreur: ' + err.message;
                msg.style.color = 'var(--color-danger)';
            }
        }

        async function removeYubiKey() {
            if (!confirm('Retirer la cle YubiKey ? La connexion par mot de passe seul sera reactivee.')) return;
            try {
                const res = await fetch('/japprends/webauthn/remove', { method: 'POST' });
                const data = await res.json();
                const msg = document.getElementById('webauthn-register-msg');
                msg.textContent = data.message;
                msg.style.color = data.ok ? 'var(--color-success)' : 'var(--color-danger)';
                msg.style.display = 'block';
                document.getElementById('webauthn-credential-export').style.display = 'none';
                loadWebAuthnStatus();
            } catch (e) {
                console.warn('webauthn remove error', e);
            }
        }

        document.getElementById('webauthn-register-btn').addEventListener('click', registerYubiKey);
        document.getElementById('webauthn-remove-btn').addEventListener('click', removeYubiKey);
        document.getElementById('webauthn-copy-btn').addEventListener('click', () => {
            const ta = document.getElementById('webauthn-credential-json');
            if (navigator.clipboard) {
                navigator.clipboard.writeText(ta.value).then(() => {
                    document.getElementById('webauthn-copy-btn').textContent = 'Copie!';
                    setTimeout(() => { document.getElementById('webauthn-copy-btn').textContent = 'Copier'; }, 1500);
                });
            }
        });

        bindTabs();
        themeEditorModule.initThemeEditor();
        refreshStatus();
        loadTestsHistory();
        loadEndpoints();
        loadAdminSignupQueue();
        loadUsers();
        loadAdminMessages();
        loadAdminDonations();
        loadWebAuthnStatus();
        setInterval(refreshStatus, 4000);
        setInterval(loadTestsHistory, 12000);
        setInterval(loadAdminSignupQueue, 6000);
        setInterval(loadUsers, 8000);
        setInterval(loadAdminMessages, 10000);
        setInterval(loadAdminDonations, 12000);
    </script>
</body>
</html>
"##
    .replace("%%ADMIN_PSEUDO%%", &admin_pseudo);

    Html(dashboard_page_assembly::assemble_dashboard_page(&template))
}
