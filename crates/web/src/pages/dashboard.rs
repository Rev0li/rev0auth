use axum::response::Html;

pub async fn dashboard() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Dashboard ALL Include</title>
    <style>
        :root {
            --ink: #10202e;
            --panel: rgba(255, 255, 255, 0.9);
            --ok: #0d9b73;
            --down: #dc4f2f;
            --tab: #edf7ff;
            --tab-active: #ffdfd2;
        }
        * { box-sizing: border-box; }
        body {
            margin: 0;
            font-family: "Space Grotesk", "Avenir Next", sans-serif;
            color: var(--ink);
            background:
                radial-gradient(circle at 7% 0%, #ffe8ce 0%, transparent 34%),
                radial-gradient(circle at 95% 15%, #d9ecff 0%, transparent 40%),
                linear-gradient(145deg, #eef7ff 0%, #e4f6ec 100%);
            min-height: 100vh;
        }
        .wrap {
            max-width: 1100px;
            margin: 0 auto;
            padding: 22px;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            margin-bottom: 14px;
            flex-wrap: wrap;
        }
        .header h1 { margin: 0; font-size: clamp(1.5rem, 4vw, 2.4rem); }
        .chip {
            border: 1px solid rgba(17, 33, 48, 0.12);
            background: #f6fbff;
            border-radius: 999px;
            padding: 7px 12px;
            font-weight: 700;
            font-size: 0.85rem;
        }

        .tabs {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin: 6px 0 16px;
        }

        .tab-btn {
            border: 1px solid rgba(17, 33, 48, 0.13);
            background: var(--tab);
            color: var(--ink);
            border-radius: 10px;
            padding: 8px 12px;
            font-weight: 700;
            cursor: pointer;
        }

        .tab-btn.active {
            background: var(--tab-active);
            border-color: rgba(237, 86, 42, 0.35);
        }

        .grid {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 14px;
        }
        .card {
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 18px;
            box-shadow: 0 16px 38px rgba(17, 33, 48, 0.14);
            animation: reveal .45s ease both;
        }
        @keyframes reveal {
            from { opacity: 0; transform: translateY(12px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .label { text-transform: uppercase; letter-spacing: .05em; font-size: .77rem; opacity: .75; }
        .state {
            margin-top: 8px;
            font-size: 1.4rem;
            font-weight: 800;
        }
        .ok { color: var(--ok); }
        .down { color: var(--down); }
        .meta { margin-top: 10px; font-size: .9rem; opacity: .85; line-height: 1.5; }
        .row {
            margin-top: 16px;
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 16px;
        }
        .actions { display: flex; gap: 10px; flex-wrap: wrap; margin-top: 10px; }
        a.btn {
            text-decoration: none;
            border-radius: 10px;
            padding: 9px 12px;
            font-weight: 700;
            border: 1px solid rgba(17, 33, 48, 0.16);
            color: var(--ink);
            background: white;
        }

        .tab-page { display: none; }
        .tab-page.active { display: block; }

        .mini {
            margin-top: 10px;
            font-size: 0.88rem;
            line-height: 1.5;
            opacity: 0.85;
        }

        .timeline {
            list-style: none;
            padding: 0;
            margin: 10px 0 0;
            display: grid;
            gap: 7px;
        }

        .timeline li {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 8px 10px;
            font-size: 0.86rem;
        }
        .tests-history {
            margin-top: 12px;
            display: grid;
            gap: 8px;
        }
        .test-run {
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 10px;
            background: #fff;
            padding: 10px;
        }
        .test-head {
            font-weight: 700;
            font-size: 0.9rem;
        }
        .test-cases {
            margin: 6px 0 0;
            padding-left: 16px;
            font-size: 0.85rem;
            opacity: 0.88;
        }

        code {
            background: #f2f9ff;
            border: 1px solid #d4e7f8;
            border-radius: 6px;
            padding: 1px 6px;
            font-size: 0.84em;
        }

        .user-card {
            border: 1px solid rgba(17, 33, 48, 0.13);
            border-radius: 10px;
            padding: 12px;
            margin: 8px 0;
            background: #fff;
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 10px;
            flex-wrap: wrap;
        }

        .user-info { flex: 1; }
        .user-name { font-weight: 700; }
        .user-meta { font-size: 0.85rem; opacity: 0.75; }
        .user-actions { display: flex; gap: 6px; flex-wrap: wrap; }

        .btn-small {
            padding: 6px 10px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 6px;
            background: #fff;
            color: var(--ink);
            font-weight: 600;
            font-size: 0.85rem;
            cursor: pointer;
        }

        .btn-small:hover { background: #f0f5fa; }
        .btn-small.danger {
            color: #dc4f2f;
            border-color: #fcc5b7;
        }

        .btn-small.danger:hover { background: #fff0ed; }

        .form-group {
            margin-bottom: 10px;
        }

        .form-group label {
            display: block;
            font-weight: 600;
            font-size: 0.9rem;
            margin-bottom: 4px;
        }

        .form-group input {
            width: 100%;
            padding: 8px;
            border: 1px solid rgba(17, 33, 48, 0.16);
            border-radius: 8px;
            font-size: 0.9rem;
            box-sizing: border-box;
        }

        @media (max-width: 900px) {
            .grid { grid-template-columns: 1fr; }
        }
    </style>
</head>
<body>
    <main class="wrap">
        <header class="header">
            <h1>Dashboard ALL Include</h1>
            <div class="chip" id="last-check">Derniere verification: --</div>
        </header>

        <nav class="tabs">
            <button class="tab-btn active" data-tab-btn="overview">Overview</button>
            <button class="tab-btn" data-tab-btn="admin">Admin</button>
            <button class="tab-btn" data-tab-btn="user">User</button>
            <button class="tab-btn" data-tab-btn="system">System</button>
            <button class="tab-btn" data-tab-btn="docs">Docs</button>
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
                    <div class="meta">Connectivite web -> API Rust (port 8080).</div>
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
        </section>

        <section class="tab-page" id="tab-admin">
            <div class="grid">
                <article class="card">
                    <div class="label">Admin Core</div>
                    <div class="state" id="admin-state-2">Chargement...</div>
                    <div class="mini">Endpoint controle: <code>/japprends/ping</code></div>
                </article>
                <article class="card">
                    <div class="label">VALIDATION INSCRIPTIONS</div>
                    <div class="mini">Validation manuelle des demandes user.</div>
                    <div id="admin-signup-queue" class="mini">Chargement...</div>
                </article>
                <article class="card">
                    <div class="label">SLO</div>
                    <div class="mini">Objectif: uptime panel admin >= 99.9%</div>
                </article>
            </div>
            <div class="row">
                <strong>All endpoints (admin view)</strong>
                <div id="endpoints-list" class="mini">Chargement...</div>
            </div>
        </section>

        <section class="tab-page" id="tab-user">
            <div class="row">
                <strong>Créer un nouvel utilisateur</strong>
                <div class="form-group" style="margin-top: 10px;">
                    <label for="new-pseudo">Pseudo:</label>
                    <input type="text" id="new-pseudo" placeholder="nouveau_pseudo" />
                    <button class="btn-small" style="margin-top: 8px;" id="create-user-btn">+ Créer</button>
                    <div id="create-msg" style="margin-top: 8px; font-size: 0.9rem; display: none;"></div>
                </div>
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
                <strong>Endpoints rapides</strong>
                <div class="actions">
                    <a class="btn" href="/status">/status</a>
                    <a class="btn" href="/status/all">/status/all</a>
                    <a class="btn" href="/japprends/ping">/japprends/ping</a>
                    <a class="btn" href="/user/ping">/user/ping</a>
                    <a class="btn" href="/japprends/tdd">TDD dashboard</a>
                    <a class="btn" href="/">Landing</a>
                </div>
            </div>
        </section>

        <section class="tab-page" id="tab-docs">
            <div class="row">
                <strong>Documentation projet</strong>
                <ul>
                    <li><code>docs/roadmap-detailed.md</code></li>
                    <li><code>docs/FunFront.md</code></li>
                    <li><code>docs/Outtime.md</code></li>
                    <li><code>docs/tickets-auth.md</code></li>
                    <li><code>Portail user: /portal</code></li>
                </ul>
            </div>
        </section>
    </main>

    <script>
        function paint(el, ok, label) {
            el.textContent = ok ? label + ' OK' : label + ' DOWN';
            el.className = 'state ' + (ok ? 'ok' : 'down');
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

            return '<div style="border:1px solid rgba(17,33,48,.13);border-radius:10px;padding:8px;margin:6px 0;background:#fff">'
                + '<strong>#' + req.id + ' - ' + req.pseudo + '</strong> [' + req.status + ']'
                + '<br>raison: ' + req.reason
                + '<br>referral: ' + req.referral
                + '<br>cree a: ' + dt
                + (actions ? '<div style="margin-top:6px">' + actions + '</div>' : '')
                + '</div>';
        }

        async function loadAdminSignupQueue() {
            const panel = document.getElementById('admin-signup-queue');
            if (!panel) return;

            const res = await fetch('/japprends/signup-requests', { cache: 'no-store' });
            const list = await res.json();
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucune demande pour le moment.';
                return;
            }

            panel.innerHTML = list.slice().reverse().map(requestRow).join('');
            panel.querySelectorAll('button[data-act]').forEach((btn) => {
                btn.addEventListener('click', async () => {
                    const id = btn.getAttribute('data-id');
                    const act = btn.getAttribute('data-act');
                    await fetch('/japprends/signup-requests/' + id + '/' + act, { method: 'POST' });
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

                paint(document.getElementById('admin-state'), adminOk, 'ADMIN');
                paint(document.getElementById('admin-state-2'), adminOk, 'ADMIN');
                paint(document.getElementById('user-state'), userOk, 'USER');
                paint(document.getElementById('user-state-2'), userOk, 'USER');
                paint(document.getElementById('api-state'), data.api_ok, 'API');

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
                pushTimeline(new Date().toLocaleTimeString() + ' | erreur de monitoring');
            }
        }

        function renderTestsHistory(runs) {
            const panel = document.getElementById('tests-history');
            if (!panel) return;
            if (!Array.isArray(runs) || runs.length === 0) {
                panel.innerHTML = '<div class="mini">Aucun test lance depuis le dashboard.</div>';
                return;
            }

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
            const panel = document.getElementById('endpoints-list');
            if (!panel) return;

            try {
                const res = await fetch('/japprends/endpoints', { cache: 'no-store' });
                const data = await res.json();
                if (!Array.isArray(data) || data.length === 0) {
                    panel.textContent = 'Aucun endpoint trouve.';
                    return;
                }

                panel.innerHTML = data.map((ep) => {
                    return '<div style="border:1px solid rgba(17,33,48,.1);border-radius:8px;padding:6px 8px;margin:6px 0;background:#fff">'
                        + '<strong>' + ep.method + '</strong> ' + ep.path + ' <span style="opacity:.75">(' + ep.scope + ')</span>'
                        + '</div>';
                }).join('');
            } catch (_err) {
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
                return '<div class="user-card">'
                    + '<div class="user-info">'
                    + '<div class="user-name">' + user.pseudo + '</div>'
                    + '<div class="user-meta">' + role + ' • ' + dt + ' • ' + statusDisplay + '</div>'
                    + '</div>'
                    + '<div class="user-actions">'
                    + '<button class="btn-small" onclick="deleteUser(\'' + user.pseudo + '\')">🗑 Supprimer</button>'
                    + '</div>'
                    + '</div>';
            }).join('');

            panel.innerHTML = html;
        }

        // User management functions
        async function createUser() {
            const pseudo = document.getElementById('new-pseudo').value.trim();
            const msg = document.getElementById('create-msg');

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
                    msg.textContent = '✓ ' + data.message;
                    document.getElementById('new-pseudo').value = '';
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

        document.getElementById('create-user-btn').addEventListener('click', createUser);
        document.getElementById('launch-tests-now').addEventListener('click', launchTestsNow);

        bindTabs();
        refreshStatus();
        loadTestsHistory();
        loadEndpoints();
        loadAdminSignupQueue();
        loadUsers();
        setInterval(refreshStatus, 4000);
        setInterval(loadTestsHistory, 12000);
        setInterval(loadAdminSignupQueue, 6000);
        setInterval(loadUsers, 8000);
    </script>
</body>
</html>
"##,
    )
}
