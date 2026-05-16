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
    <title>rev0auth — Dashboard</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRONTEND_SHARED_CSS%%
        %%DASHBOARD_PAGE_STYLES%%
    </style>
</head>
<body>
    <main class="wrap">
        <header class="header">
            <h1>Dashboard</h1>
            <div class="chip" id="last-check">—</div>
        </header>

        <nav class="tabs">
            <button class="tab-btn active" data-tab-btn="status">Status</button>
            <button class="tab-btn" data-tab-btn="members">Members</button>
            <button class="tab-btn" data-tab-btn="messages">Messages</button>
            <button class="tab-btn" data-tab-btn="donations">Donations</button>
            <button class="tab-btn" data-tab-btn="logs">Logs</button>
        </nav>

        <!-- ====== STATUS ====== -->
        <section class="tab-page active" id="tab-status">
            <div class="grid">
                <article class="card">
                    <div class="label">Admin</div>
                    <div class="state" id="admin-state">—</div>
                </article>
                <article class="card">
                    <div class="label">User</div>
                    <div class="state" id="user-state">—</div>
                </article>
                <article class="card">
                    <div class="label">API</div>
                    <div class="state" id="api-state">—</div>
                </article>
            </div>

            <div class="row">
                <strong>Chain checks</strong>
                <div id="chain-checks" class="chain-grid">—</div>
            </div>

            <div class="row">
                <strong>Endpoints</strong>
                <div class="mini" style="margin-bottom:8px">Etat par scope — non cliquable.</div>
                <div id="endpoints-system-list" class="endpoint-grid">—</div>
            </div>

        </section>

        <!-- ====== MEMBERS ====== -->
        <section class="tab-page" id="tab-members">
            <div class="row">
                <strong>Membres</strong>
                <div id="users-list" style="margin-top:10px">Chargement...</div>
            </div>
            <div class="row">
                <div style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:6px">
                    <strong>Mur communautaire</strong>
                    <button class="btn-small" id="wall-refresh-btn">↺ Rafraîchir</button>
                </div>
                <div style="display:flex;gap:8px;margin-top:10px">
                    <input id="admin-wall-input" type="text" class="field-input" placeholder="Écrire un post (140 car. max)…" maxlength="140" style="flex:1" />
                    <button class="btn-small grant" id="admin-wall-send-btn">Poster</button>
                </div>
                <div id="admin-wall-msg" class="mini" style="display:none;margin-top:4px"></div>
                <div id="admin-wall-list" class="mini" style="margin-top:8px">Chargement...</div>
            </div>
        </section>

        <!-- ====== MESSAGES ====== -->
        <section class="tab-page" id="tab-messages">
            <div class="row">
                <strong>Conversations membres</strong>
                <div class="msg-admin-layout" style="margin-top:12px">
                    <aside id="admin-thread-list" class="msg-thread-list">Chargement...</aside>
                    <div class="msg-admin-panel">
                        <div id="admin-messages" class="msg-conversation"></div>
                        <div class="msg-compose">
                            <input id="admin-reply-to" placeholder="Destinataire" style="display:none" />
                            <div class="msg-compose-row">
                                <div class="msg-emoji-wrap">
                                    <button id="admin-emoji-btn" class="msg-emoji-btn" type="button" title="Emojis">😊</button>
                                    <div id="admin-emoji-panel" class="msg-emoji-panel"></div>
                                </div>
                                <textarea id="admin-reply-body" class="msg-compose-input" rows="1" placeholder="Répondre..."></textarea>
                                <button class="msg-compose-send" id="admin-reply-send">➤</button>
                            </div>
                            <div id="admin-reply-msg" class="msg-reply-status"></div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- ====== DONATIONS ====== -->
        <section class="tab-page" id="tab-donations">
            <div class="row">
                <div style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:6px">
                    <strong>Preuves donations</strong>
                    <button class="btn-small" id="donations-refresh-btn">↺ Rafraîchir</button>
                </div>
                <div id="admin-donations" class="mini" style="margin-top:8px">Chargement...</div>
            </div>
        </section>

        <!-- ====== LOGS ====== -->
        <section class="tab-page" id="tab-logs">
            <div class="row">
                <div style="display:flex;align-items:center;gap:10px;flex-wrap:wrap">
                    <strong>Sweep routes</strong>
                    <button class="btn-small grant" id="run-sweep-btn">Lancer le sweep</button>
                    <button class="btn-small" id="clear-sweep-btn">Effacer</button>
                    <span class="chip" id="sweep-time" style="margin-left:auto">—</span>
                </div>
                <pre id="sweep-log" class="sweep-log">Pret. Clique sur "Lancer le sweep".</pre>
            </div>
        </section>

    </main>

    <script>
        const adminPseudo = "%%ADMIN_PSEUDO%%";
        const monitorState = { adminOk: false, userOk: false, apiOk: false };
        const dashboardStats = { users: [], pendingSignups: 0, lastRun: null, endpoints: [] };
        const adminChatState = {
            messages: [],
            selectedThread: localStorage.getItem('dashboard_chat_thread') || ''
        };
        const chainState = {
            webToApi: false, adminPing: false, userPing: false, endpointRegistry: false
        };

        %%COMMON_JS_UTILS%%
        %%DASHBOARD_CHAT_MODULE%%
        %%DASHBOARD_USERS_MODULE%%
        %%DASHBOARD_DONATIONS_MODULE%%
%%DASHBOARD_QUEUE_MODULE%%
        %%DASHBOARD_STATUS_MODULE%%
        const adminChatModule = createDashboardChatModule({ adminPseudo, adminChatState });
        const { setAdminReplyMsg, sendAdminReply, loadAdminMessages } = adminChatModule;

        // ---- utils ----
        function paint(el, ok, label) {
            if (!el) return;
            el.textContent = ok ? label + ' OK' : label + ' DOWN';
            el.className = 'state ' + (ok ? 'ok' : 'down');
        }

        function copyTempPassword(password) {
            if (!password) return;
            if (navigator.clipboard?.writeText) navigator.clipboard.writeText(password).catch(() => {});
        }

        // ---- users ----
        async function loadUsers() {
            const panel = document.getElementById('users-list');
            if (!panel) return;
            const res = await fetch('/users', { cache: 'no-store' });
            const list = await res.json();
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<span class="mini">Aucun utilisateur.</span>'; return;
            }
            dashboardStats.users = list;
            panel.innerHTML = '<div class="member-gallery">'
                + list.map(user => {
                    const dt = new Date(user.created_at_epoch * 1000).toLocaleDateString('fr-FR');
                    const statusLabel = user.status === 'actif' ? 'actif' : user.status === 'occupe' ? 'occupé' : 'inactif';
                    const statusCls = user.status === 'actif' ? 'active' : user.status === 'occupe' ? 'pending' : 'inactive';
                    const roleCls = ['admin','mod','member','guest'].includes(user.role) ? user.role : 'guest';
                    const p = encodeURIComponent(user.pseudo);
                    const reqBadges = [
                        user.request_github && 'GH',
                        user.request_jellyfin && 'JF',
                        user.request_songsurf && 'SS'
                    ].filter(Boolean);
                    const safePseudo = escapeHtml(user.pseudo);
                    return '<div class="member-card" onclick="openUserProfile(\'' + safePseudo + '\')">'
                        + '<div class="member-card-avatar-wrap">'
                        + '<img class="member-card-avatar" src="/members/avatar/' + p + '?t=' + Date.now() + '" alt="" '
                        + 'onerror="this.onerror=null;this.style.display=\'none\';this.nextElementSibling.style.display=\'flex\'">'
                        + '<div class="member-card-avatar-fallback" style="display:none">' + safePseudo.charAt(0).toUpperCase() + '</div>'
                        + '</div>'
                        + '<div class="member-card-pseudo">' + safePseudo + '</div>'
                        + '<div class="member-card-meta">'
                        + '<span class="member-status ' + statusCls + '">● ' + statusLabel + '</span>'
                        + '<span class="member-badge ' + roleCls + '">' + user.role + '</span>'
                        + '</div>'
                        + '<div class="member-card-meta">' + dt + '</div>'
                        + (reqBadges.length ? '<div class="member-card-pending">⏳ ' + reqBadges.join(' · ') + '</div>' : '')
                        + '</div>';
                }).join('')
                + '</div>';
        }

        async function toggleServiceAccess(pseudo, service, nextValue) {
            const payload = {};
            payload['access_' + service] = !!nextValue;
            const res = await fetch('/japprends/users/' + pseudo, {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(payload)
            });
            const data = await res.json();
            if (data.ok) loadUsers(); else alert('Erreur: ' + data.message);
        }

        function openUserProfile(pseudo) {
            window.location.href = '/members/profile?pseudo=' + encodeURIComponent(pseudo) + '&admin=1';
        }

        async function deleteUser(pseudo) {
            if (!confirm('Supprimer ' + pseudo + ' ?')) return;
            const res = await fetch('/japprends/users/' + pseudo, { method: 'DELETE' });
            const data = await res.json();
            if (data.ok) loadUsers(); else alert('Erreur: ' + data.message);
        }

        // ---- wall ----
        async function loadAdminWall() {
            const panel = document.getElementById('admin-wall-list');
            if (!panel) return;
            try {
                const res = await fetch('/members/wall', { cache: 'no-store' });
                const list = await res.json();
                if (!Array.isArray(list) || list.length === 0) {
                    panel.textContent = 'Aucun post sur le mur.'; return;
                }
                panel.innerHTML = list.map(p => {
                    const dt = new Date(p.created_at_epoch * 1000).toLocaleString();
                    return '<div class="donation-row" style="display:flex;align-items:flex-start;gap:8px;justify-content:space-between">'
                        + '<div><strong>' + escapeHtml(p.pseudo) + '</strong><span class="mini" style="margin-left:8px">' + dt + '</span>'
                        + '<div style="margin-top:4px">' + escapeHtml(p.body) + '</div></div>'
                        + '<button class="btn-small danger" onclick="deleteWallPost(' + p.id + ')">🗑</button>'
                        + '</div>';
                }).join('');
            } catch (_) { panel.textContent = 'Erreur chargement mur.'; }
        }

        async function deleteWallPost(id) {
            if (!confirm('Supprimer ce post ?')) return;
            try {
                await fetch('/japprends/wall/' + id, { method: 'DELETE' });
                await loadAdminWall();
            } catch (_) {}
        }

        async function postAdminWallMessage() {
            const input = document.getElementById('admin-wall-input');
            const msgEl = document.getElementById('admin-wall-msg');
            const body = input.value.trim();
            if (!body) return;
            try {
                const res = await fetch('/members/wall', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo: adminPseudo, body })
                });
                const data = await res.json();
                if (data.ok) {
                    input.value = '';
                    msgEl.style.display = 'none';
                    await loadAdminWall();
                } else {
                    msgEl.textContent = data.message || 'Erreur.';
                    msgEl.style.display = 'block';
                }
            } catch (err) {
                msgEl.textContent = 'Erreur: ' + err.message;
                msgEl.style.display = 'block';
            }
        }

        // ---- donations ----
        async function loadAdminDonations() {
            const panel = document.getElementById('admin-donations');
            if (!panel) return;
            try {
                const res = await fetch('/japprends/donations', { cache: 'no-store' });
                const list = await res.json();
                if (!Array.isArray(list) || list.length === 0) {
                    panel.textContent = 'Aucune preuve donation.'; return;
                }
                panel.innerHTML = list.slice().reverse().map(row => {
                    const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                    const state = !row.reviewed ? 'pending' : (row.approved ? 'approuvee' : 'refusee');
                    const actions = row.reviewed ? '' :
                        '<button data-donation-review="1" data-id="' + row.id + '" data-approved="true">Valider</button> '
                        + '<button data-donation-review="1" data-id="' + row.id + '" data-approved="false">Refuser</button>';
                    return '<div class="donation-row">'
                        + '<strong>#' + row.id + '</strong> • ' + row.pseudo + ' • ' + row.method
                        + '<br>code: ' + row.code + ' • etat: ' + state + ' • ' + dt
                        + (actions ? '<div class="donation-actions">' + actions + '</div>' : '')
                        + '</div>';
                }).join('');
                panel.querySelectorAll('button[data-donation-review="1"]').forEach(btn => {
                    btn.addEventListener('click', async () => {
                        const approved = btn.getAttribute('data-approved') === 'true';
                        await fetch('/japprends/donations/' + btn.getAttribute('data-id') + '/review', {
                            method: 'POST',
                            headers: { 'content-type': 'application/json' },
                            body: JSON.stringify({ approved })
                        });
                        loadAdminDonations();
                    });
                });
            } catch (_) { panel.textContent = 'Erreur chargement donations.'; }
        }

        // ---- endpoints + chain ----
        function endpointScopeOk(ep) {
            if (ep.scope === 'admin') return monitorState.adminOk && monitorState.apiOk;
            if (ep.scope === 'member') return monitorState.userOk && monitorState.apiOk;
            return true;
        }

        function renderEndpointsSystem() {
            const panel = document.getElementById('endpoints-system-list');
            if (!panel) return;
            const data = dashboardStats.endpoints;
            if (!Array.isArray(data) || data.length === 0) { panel.textContent = '—'; return; }

            const SCOPE_ORDER = ['admin', 'member', 'public'];
            const SCOPE_LABEL = { admin: 'Admin', member: 'Membre', public: 'Public' };
            const groups = {};
            data.forEach(ep => {
                const s = SCOPE_ORDER.includes(ep.scope) ? ep.scope : 'public';
                if (!groups[s]) groups[s] = [];
                groups[s].push(ep);
            });

            // Preserve open state across re-renders (refreshStatus fires every 5s)
            const openScopes = new Set();
            panel.querySelectorAll('.ep-section.open').forEach(el => openScopes.add(el.getAttribute('data-scope')));

            panel.innerHTML = SCOPE_ORDER.filter(s => groups[s] && groups[s].length > 0).map(scope => {
                const eps = groups[scope];
                const allOk = eps.every(ep => endpointScopeOk(ep));
                const label = SCOPE_LABEL[scope] || scope;
                const isOpen = openScopes.has(scope);
                const items = eps.map(ep => {
                    const ok = endpointScopeOk(ep);
                    return '<div class="endpoint-item">'
                        + '<div><strong>' + ep.method + '</strong> ' + ep.path + '</div>'
                        + (ok ? '<span class="badge-ok">OK</span>' : '<span class="badge-ko">KO</span>')
                        + '</div>';
                }).join('');
                return '<div class="ep-section' + (isOpen ? ' open' : '') + '" data-scope="' + scope + '">'
                    + '<div class="ep-section-head">'
                    + '<span class="ep-section-label">' + label + ' <span class="ep-section-count">(' + eps.length + ')</span></span>'
                    + (allOk ? '<span class="badge-ok">OK</span>' : '<span class="badge-ko">KO</span>')
                    + '<span class="ep-chevron">›</span>'
                    + '</div>'
                    + '<div class="ep-section-items">' + items + '</div>'
                    + '</div>';
            }).join('');

            panel.querySelectorAll('.ep-section-head').forEach(head => {
                head.addEventListener('click', () => {
                    head.closest('.ep-section').classList.toggle('open');
                });
            });
        }

        function renderChainChecks() {
            const panel = document.getElementById('chain-checks');
            if (!panel) return;
            const items = [
                { key: 'webToApi', label: 'Web → API' },
                { key: 'adminPing', label: 'Admin ping' },
                { key: 'userPing', label: 'User ping' },
                { key: 'endpointRegistry', label: 'Endpoint registry' }
            ];
            panel.innerHTML = items.map(item =>
                '<div class="endpoint-item"><div><strong>' + item.label + '</strong></div>'
                + (chainState[item.key] ? '<span class="badge-ok">OK</span>' : '<span class="badge-ko">KO</span>')
                + '</div>'
            ).join('');
        }

        async function loadEndpoints() {
            const panel = document.getElementById('endpoints-system-list');
            if (!panel) return;
            try {
                const data = await fetch('/japprends/endpoints', { cache: 'no-store' }).then(r => r.json());
                dashboardStats.endpoints = Array.isArray(data) ? data : [];
                chainState.endpointRegistry = Array.isArray(data) && data.length > 0;
                renderChainChecks();
                renderEndpointsSystem();
            } catch (_) { chainState.endpointRegistry = false; renderChainChecks(); }
        }

        // ---- monitor ----
        async function refreshStatus() {
            try {
                const [basicRes, adminRes, userRes] = await Promise.all([
                    fetch('/status', { cache: 'no-store' }),
                    fetch('/japprends/ping', { cache: 'no-store' }),
                    fetch('/user/ping', { cache: 'no-store' })
                ]);
                const data = await basicRes.json();
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

                paint(document.getElementById('admin-state'), adminOk, 'Admin');
                paint(document.getElementById('user-state'), userOk, 'User');
                paint(document.getElementById('api-state'), data.api_ok, 'API');
                renderChainChecks();
                renderEndpointsSystem();

                const dt = new Date(data.checked_at_epoch * 1000);
                document.getElementById('last-check').textContent = dt.toLocaleTimeString();
            } catch (_) {
                ['admin-state', 'user-state', 'api-state'].forEach(id => {
                    const el = document.getElementById(id);
                    if (el) { el.textContent = 'DOWN'; el.className = 'state down'; }
                });
                chainState.webToApi = chainState.adminPing = chainState.userPing = false;
                renderChainChecks();
            }
        }

        // ---- tabs ----
        function activateTab(tab) {
            const buttons = document.querySelectorAll('[data-tab-btn]');
            const pages = document.querySelectorAll('.tab-page');
            buttons.forEach(b => b.classList.remove('active'));
            pages.forEach(p => p.classList.remove('active'));
            const btn = document.querySelector('[data-tab-btn="' + tab + '"]');
            if (btn) btn.classList.add('active');
            const page = document.getElementById('tab-' + tab);
            if (page) page.classList.add('active');
            history.replaceState(null, '', '#' + tab);
            if (tab === 'messages') loadAdminMessages();
            if (tab === 'donations') loadAdminDonations();
            if (tab === 'members') { loadUsers(); loadAdminWall(); }
            if (tab === 'logs') runSweep();
        }

        function bindTabs() {
            document.querySelectorAll('[data-tab-btn]').forEach(btn => {
                btn.addEventListener('click', () => {
                    activateTab(btn.getAttribute('data-tab-btn'));
                });
            });
        }

        // ---- sweep logs ----
        const SWEEP_ROUTES = [
            { method: 'GET', path: '/status' },
            { method: 'GET', path: '/status/all' },
            { method: 'GET', path: '/japprends/ping' },
            { method: 'GET', path: '/user/ping' },
            { method: 'GET', path: '/japprends/tdd' },
            { method: 'GET', path: '/home/friend' },
            { method: 'GET', path: '/' },
        ];

        async function runSweep() {
            const log = document.getElementById('sweep-log');
            const timeChip = document.getElementById('sweep-time');
            if (!log) return;
            const started = new Date();
            timeChip.textContent = started.toLocaleTimeString();
            log.textContent = '[' + started.toLocaleTimeString() + '] Sweep demarre...\n';

            for (const route of SWEEP_ROUTES) {
                try {
                    const res = await fetch(route.path, { cache: 'no-store' });
                    const status = res.status;
                    const ok = status >= 200 && status < 400;
                    log.textContent += (ok ? '  OK  ' : ' FAIL ') + ' ' + status + '  ' + route.method + ' ' + route.path + '\n';
                } catch (err) {
                    log.textContent += ' ERR   ---  ' + route.method + ' ' + route.path + '  (' + err.message + ')\n';
                }
                log.scrollTop = log.scrollHeight;
            }
            const elapsed = ((Date.now() - started.getTime()) / 1000).toFixed(2);
            log.textContent += '\nTermine en ' + elapsed + 's.\n';
            log.scrollTop = log.scrollHeight;
        }

        // ---- init ----
        document.getElementById('admin-reply-send').addEventListener('click', sendAdminReply);
        document.getElementById('admin-reply-body').addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendAdminReply(); }
        });
document.getElementById('run-sweep-btn').addEventListener('click', runSweep);
        document.getElementById('clear-sweep-btn').addEventListener('click', () => {
            const log = document.getElementById('sweep-log');
            if (log) log.textContent = 'Pret. Clique sur "Lancer le sweep".';
        });
        document.getElementById('wall-refresh-btn').addEventListener('click', loadAdminWall);
        document.getElementById('admin-wall-send-btn').addEventListener('click', postAdminWallMessage);
        document.getElementById('admin-wall-input').addEventListener('keydown', (e) => {
            if (e.key === 'Enter') { e.preventDefault(); postAdminWallMessage(); }
        });
        document.getElementById('donations-refresh-btn').addEventListener('click', loadAdminDonations);

        bindTabs();

        // Restore tab from URL hash (e.g. back-navigation from profile with #members)
        const initHash = window.location.hash.slice(1);
        if (initHash && document.querySelector('[data-tab-btn="' + initHash + '"]')) {
            activateTab(initHash);
        }

        refreshStatus();
        loadEndpoints();
        loadUsers();
        loadAdminWall();
        loadAdminMessages();
        setInterval(refreshStatus, 5000);
setInterval(loadUsers, 10000);
        setInterval(loadAdminMessages, 12000);
    </script>
</body>
</html>
"##
    .replace("%%ADMIN_PSEUDO%%", &admin_pseudo);

    Html(dashboard_page_assembly::assemble_dashboard_page(&template))
}
