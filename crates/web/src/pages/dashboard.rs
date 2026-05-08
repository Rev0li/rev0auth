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
                <div style="display:flex;align-items:center;gap:12px;flex-wrap:wrap">
                    <strong>Tests</strong>
                    <button class="btn-small grant" id="launch-tests-now">Lancer maintenant</button>
                </div>
                <div id="tests-progress-wrap" style="display:none;margin-top:10px">
                    <div style="font-size:0.78rem;color:var(--color-muted);margin-bottom:4px" id="tests-progress-label">Execution en cours...</div>
                    <div style="background:var(--bg-card);border:1px solid var(--color-panel-border);border-radius:4px;height:6px;overflow:hidden">
                        <div id="tests-progress-bar" style="height:100%;width:0%;background:var(--color-accent);transition:width 0.4s linear"></div>
                    </div>
                </div>
                <div id="tests-history" class="tests-history" style="margin-top:10px"></div>
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
                <strong>Demandes d'inscription</strong>
                <div id="admin-signup-queue" class="mini" style="margin-top:8px">Chargement...</div>
            </div>
            <div class="row">
                <strong>Utilisateurs</strong>
                <div id="users-list" class="mini" style="margin-top:8px">Chargement...</div>
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
                <strong>Preuves donations</strong>
                <div id="admin-donations" class="mini" style="margin-top:8px">Chargement...</div>
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
        %%DASHBOARD_TESTING_MODULE%%
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

        // ---- signup queue ----
        function requestRow(req) {
            const dt = new Date(req.created_at_epoch * 1000).toLocaleTimeString();
            const canAct = req.status === 'pending';
            const actions = canAct
                ? '<button data-act="approve" data-id="' + req.id + '">Approuver</button> '
                    + '<button data-act="reject" data-id="' + req.id + '">Rejeter</button>'
                : '';
            return '<div class="request-row">'
                + '<strong>#' + req.id + ' — ' + req.pseudo + '</strong> [' + req.status + ']'
                + '<br>referral: ' + req.referral + ' &nbsp;•&nbsp; ' + dt
                + (actions ? '<div class="request-actions">' + actions + '</div>' : '')
                + '</div>';
        }

        async function loadAdminSignupQueue() {
            const panel = document.getElementById('admin-signup-queue');
            if (!panel) return;
            const res = await fetch('/japprends/signup-requests', { cache: 'no-store' });
            const list = await res.json();
            dashboardStats.pendingSignups = Array.isArray(list)
                ? list.filter(r => r.status === 'pending').length : 0;
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucune demande.'; return;
            }
            panel.innerHTML = list.slice().reverse().map(requestRow).join('');
            panel.querySelectorAll('button[data-act]').forEach(btn => {
                btn.addEventListener('click', async () => {
                    const id = btn.getAttribute('data-id');
                    const act = btn.getAttribute('data-act');
                    const res = await fetch('/japprends/signup-requests/' + id + '/' + act, { method: 'POST' });
                    const data = await res.json();
                    if (data?.ok && data.temp_password) {
                        copyTempPassword(data.temp_password);
                        alert('Mot de passe temporaire: ' + data.temp_password);
                    }
                    await loadAdminSignupQueue();
                });
            });
        }

        // ---- users ----
        async function loadUsers() {
            const panel = document.getElementById('users-list');
            if (!panel) return;
            const res = await fetch('/users', { cache: 'no-store' });
            const list = await res.json();
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucun utilisateur.'; return;
            }
            dashboardStats.users = list;
            panel.innerHTML = list.map(user => {
                const dt = new Date(user.created_at_epoch * 1000).toLocaleDateString('fr-FR');
                const statusDot = user.status === 'actif' ? '🟢' : user.status === 'occupe' ? '🟡' : '🔴';
                const reqBadges = [
                    user.request_github && 'GitHub',
                    user.request_jellyfin && 'Jellyfin',
                    user.request_songsurf && 'Songsurf'
                ].filter(Boolean);
                return '<div class="user-card">'
                    + '<div class="user-info">'
                    + '<div class="user-name">' + user.pseudo + ' <span style="font-weight:400;color:var(--color-muted)">' + statusDot + ' ' + dt + '</span></div>'
                    + (reqBadges.length ? '<div class="user-meta user-request-badges">Demandes: ' + reqBadges.join(', ') + '</div>' : '')
                    + '</div>'
                    + '<div class="user-actions">'
                    + '<button class="btn-small warn" onclick="openUserProfile(\'' + user.pseudo + '\')">Profil</button>'
                    + '<button class="btn-small ' + (user.access_github ? 'danger' : 'grant') + '" onclick="toggleServiceAccess(\'' + user.pseudo + '\',\'github\',' + (!user.access_github) + ')">GitHub ' + (user.access_github ? 'ON' : 'OFF') + '</button>'
                    + '<button class="btn-small ' + (user.access_jellyfin ? 'danger' : 'grant') + '" onclick="toggleServiceAccess(\'' + user.pseudo + '\',\'jellyfin\',' + (!user.access_jellyfin) + ')">Jellyfin ' + (user.access_jellyfin ? 'ON' : 'OFF') + '</button>'
                    + '<button class="btn-small ' + (user.access_songsurf ? 'danger' : 'grant') + '" onclick="toggleServiceAccess(\'' + user.pseudo + '\',\'songsurf\',' + (!user.access_songsurf) + ')">Songsurf ' + (user.access_songsurf ? 'ON' : 'OFF') + '</button>'
                    + '<button class="btn-small" onclick="deleteUser(\'' + user.pseudo + '\')">🗑</button>'
                    + '</div>'
                    + '</div>';
            }).join('');
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
                        + '<div class="donation-actions">'
                        + '<a class="btn-small warn" target="_blank" href="/members/donations/proof/' + row.id + '/photo">Photo</a>'
                        + actions
                        + '</div></div>';
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

        // ---- tests ----
        let testsRunning = false;
        let progressTimer = null;

        function renderTestsHistory(runs) {
            const panel = document.getElementById('tests-history');
            if (!panel) return;
            if (!Array.isArray(runs) || runs.length === 0) {
                panel.innerHTML = '<div class="mini">Aucun test lance.</div>'; return;
            }
            dashboardStats.lastRun = runs[0];
            panel.innerHTML = runs.slice(0, 8).map((run, idx) => {
                const dt = new Date(run.executed_at_epoch * 1000).toLocaleString();
                const allOk = run.passed === run.total;
                return '<div class="test-run" id="run-' + run.run_id + '">'
                    + '<div class="test-head' + (allOk ? ' ok' : ' fail') + '">'
                    + 'Run #' + run.run_id + ' — ' + run.passed + '/' + run.total + ' — ' + dt
                    + '</div>'
                    + '<ul class="test-cases" id="cases-' + run.run_id + '"></ul>'
                    + '</div>';
            }).join('');

            runs.slice(0, 8).forEach((run, runIdx) => {
                const ul = document.getElementById('cases-' + run.run_id);
                if (!ul || !Array.isArray(run.cases)) return;
                run.cases.forEach((c, i) => {
                    const delay = runIdx === 0 ? i * 28 : 0;
                    setTimeout(() => {
                        const li = document.createElement('li');
                        li.className = c.ok ? 'case-ok' : 'case-fail';
                        li.textContent = (c.ok ? '✓ ' : '✗ ') + c.name;
                        ul.appendChild(li);
                    }, delay);
                });
            });
        }

        function startProgressBar(estimatedMs) {
            const wrap = document.getElementById('tests-progress-wrap');
            const bar = document.getElementById('tests-progress-bar');
            const label = document.getElementById('tests-progress-label');
            if (!wrap || !bar) return;
            wrap.style.display = 'block';
            bar.style.transition = 'none';
            bar.style.width = '0%';
            label.textContent = 'Execution en cours...';
            requestAnimationFrame(() => {
                bar.style.transition = 'width ' + (estimatedMs / 1000).toFixed(1) + 's linear';
                bar.style.width = '88%';
            });
            if (progressTimer) clearTimeout(progressTimer);
        }

        function completeProgressBar(passed, total) {
            const bar = document.getElementById('tests-progress-bar');
            const label = document.getElementById('tests-progress-label');
            const wrap = document.getElementById('tests-progress-wrap');
            if (!bar) return;
            const allOk = passed === total;
            bar.style.transition = 'width 0.25s ease';
            bar.style.width = '100%';
            bar.style.background = allOk ? 'var(--color-success)' : 'var(--color-error, #e55)';
            label.textContent = passed + '/' + total + ' tests passes';
            setTimeout(() => { if (wrap) wrap.style.display = 'none'; bar.style.background = ''; }, 2800);
        }

        async function loadTestsHistory() {
            try {
                const res = await fetch('/japprends/tests/history', { cache: 'no-store' });
                renderTestsHistory(await res.json());
            } catch (_) {}
        }

        async function launchTestsNow() {
            if (testsRunning) return;
            testsRunning = true;
            document.getElementById('launch-tests-now').disabled = true;
            startProgressBar(11000);
            try {
                const res = await fetch('/japprends/tests/launch', { method: 'POST' });
                const run = await res.json();
                completeProgressBar(run.passed, run.total);
                loadTestsHistory();
            } catch (_) {
                completeProgressBar(0, 1);
            } finally {
                testsRunning = false;
                document.getElementById('launch-tests-now').disabled = false;
            }
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
            panel.innerHTML = data.map(ep => {
                const ok = endpointScopeOk(ep);
                return '<div class="endpoint-item">'
                    + '<div><strong>' + ep.method + '</strong> ' + ep.path
                    + '<div class="endpoint-meta">scope: ' + ep.scope + '</div></div>'
                    + (ok ? '<span class="badge-ok">OK</span>' : '<span class="badge-ko">KO</span>')
                    + '</div>';
            }).join('');
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
        function bindTabs() {
            const buttons = document.querySelectorAll('[data-tab-btn]');
            const pages = document.querySelectorAll('.tab-page');
            buttons.forEach(btn => {
                btn.addEventListener('click', () => {
                    const tab = btn.getAttribute('data-tab-btn');
                    buttons.forEach(b => b.classList.remove('active'));
                    pages.forEach(p => p.classList.remove('active'));
                    btn.classList.add('active');
                    const page = document.getElementById('tab-' + tab);
                    if (page) page.classList.add('active');
                    if (tab === 'messages') loadAdminMessages();
                    if (tab === 'donations') loadAdminDonations();
                    if (tab === 'members') { loadAdminSignupQueue(); loadUsers(); }
                });
            });
        }

        // ---- init ----
        document.getElementById('admin-reply-send').addEventListener('click', sendAdminReply);
        document.getElementById('admin-reply-body').addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendAdminReply(); }
        });
        document.getElementById('launch-tests-now').addEventListener('click', launchTestsNow);

        bindTabs();
        refreshStatus();
        loadTestsHistory();
        loadEndpoints();
        loadAdminSignupQueue();
        loadUsers();
        loadAdminMessages();
        setInterval(refreshStatus, 5000);
        setInterval(loadTestsHistory, 15000);
        setInterval(loadAdminSignupQueue, 8000);
        setInterval(loadUsers, 10000);
        setInterval(loadAdminMessages, 12000);
    </script>
</body>
</html>
"##
    .replace("%%ADMIN_PSEUDO%%", &admin_pseudo);

    Html(dashboard_page_assembly::assemble_dashboard_page(&template))
}
