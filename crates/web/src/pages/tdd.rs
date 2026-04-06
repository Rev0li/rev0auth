use axum::response::Html;

use super::frontend_theme;

pub async fn tdd_dashboard() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - TDD Dashboard</title>
    %%FRONTEND_THEME_BOOT%%
    <style>
        %%FRONTEND_SHARED_CSS%%
        :root {
            --ink: #10202e;
            --panel: rgba(255, 255, 255, 0.92);
            --ok: #0d9b73;
            --down: #dc4f2f;
            --wait: #927d2a;
            --bg1: #fff0df;
            --bg2: #dcedff;
        }
        * { box-sizing: border-box; }
        body {
            margin: 0;
            font-family: var(--font-sans);
            color: var(--ink);
            background:
                radial-gradient(circle at 8% 0%, var(--bg1) 0%, transparent 34%),
                radial-gradient(circle at 92% 10%, var(--bg2) 0%, transparent 38%),
                linear-gradient(145deg, #f4f8ff 0%, #edf8f0 100%);
            min-height: 100vh;
        }
        .wrap {
            max-width: 1180px;
            margin: 0 auto;
            padding: 24px;
        }
        .top {
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 12px;
            flex-wrap: wrap;
            margin-bottom: 16px;
        }
        h1 { margin: 0; font-size: clamp(1.6rem, 4vw, 2.6rem); }
        .chip {
            border: 1px solid rgba(17, 33, 48, 0.12);
            background: #f8fbff;
            border-radius: 999px;
            padding: 8px 12px;
            font-size: 0.86rem;
            font-weight: 700;
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
            box-shadow: 0 16px 38px rgba(17, 33, 48, 0.12);
        }
        .card h2 {
            margin: 0 0 8px;
            font-size: 1.05rem;
        }
        .state {
            font-size: 1.35rem;
            font-weight: 800;
            margin-bottom: 8px;
        }
        .ok { color: var(--ok); }
        .down { color: var(--down); }
        .wait { color: var(--wait); }
        .mini { font-size: 0.92rem; line-height: 1.5; opacity: 0.85; }
        .row {
            margin-top: 16px;
            background: var(--panel);
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 18px;
            padding: 16px;
        }
        .actions {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 10px;
        }
        button, a.btn {
            border: 1px solid rgba(17, 33, 48, 0.16);
            background: #fff;
            color: var(--ink);
            border-radius: 10px;
            padding: 9px 12px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
        }
        .suite-list {
            display: grid;
            gap: 10px;
            margin-top: 12px;
        }
        .suite {
            background: #fff;
            border: 1px solid rgba(17, 33, 48, 0.1);
            border-radius: 12px;
            padding: 12px;
        }
        .suite-top {
            display: flex;
            justify-content: space-between;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
        .suite-name { font-weight: 800; }
        .suite-command {
            margin-top: 6px;
            font-size: 0.88rem;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            background: #f7fafe;
            border: 1px solid #d9e8f5;
            border-radius: 8px;
            padding: 8px 10px;
            overflow-x: auto;
        }
        .log {
            margin-top: 10px;
            padding: 12px;
            background: #0f1a24;
            color: #d9e6f2;
            border-radius: 12px;
            min-height: 160px;
            white-space: pre-wrap;
            font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
            font-size: 0.84rem;
            line-height: 1.45;
        }
        code {
            background: #f2f9ff;
            border: 1px solid #d4e7f8;
            border-radius: 6px;
            padding: 1px 6px;
            font-size: 0.84em;
        }
        @media (max-width: 900px) {
            .grid { grid-template-columns: 1fr; }
        }
    </style>
</head>
<body>
    <main class="wrap">
        <header class="top">
            <div>
                <h1>TDD Dashboard</h1>
                <div class="mini">Training launch local. Les checks ici restent safe: ils valident l'UI et les endpoints deja exposes, pas un lancement de shell arbitraire depuis le navigateur.</div>
            </div>
            <div class="chip" id="last-run">Dernier sweep: --</div>
        </header>

        <section class="grid">
            <article class="card">
                <h2>API</h2>
                <div class="state wait" id="api-state">En attente</div>
                <div class="mini">Couvre le ping logique et la disponibilite du socle web/API.</div>
            </article>
            <article class="card">
                <h2>Web</h2>
                <div class="state wait" id="web-state">En attente</div>
                <div class="mini">Couvre les pages principales et la zone dashboard.</div>
            </article>
            <article class="card">
                <h2>Suite</h2>
                <div class="state wait" id="suite-state">Pret</div>
                <div class="mini">Resume du sweep local + commandes Rust a lancer pour les vrais tests.</div>
            </article>
        </section>

        <section class="row">
            <strong>Sweep local</strong>
            <div class="actions">
                <button id="run-sweep">Lancer le sweep</button>
                <button id="clear-log">Effacer le log</button>
                <a class="btn" href="/dashboard">Retour dashboard ops</a>
            </div>

            <div class="suite-list" id="suite-list">
                <div class="suite">
                    <div class="suite-top">
                        <div class="suite-name">Smoke web</div>
                        <div class="chip">Commande locale</div>
                    </div>
                    <div class="suite-command">make launch-all</div>
                    <div class="mini">Demarre API + web en local pour valider le socle avant les tests manuels.</div>
                </div>
                <div class="suite">
                    <div class="suite-top">
                        <div class="suite-name">API tests</div>
                        <div class="chip">Cargo</div>
                    </div>
                    <div class="suite-command">~/.cargo/bin/cargo test -p rev0auth-api</div>
                    <div class="mini">Lance les tests unitaires et d'integration du backend auth.</div>
                </div>
                <div class="suite">
                    <div class="suite-top">
                        <div class="suite-name">Web checks</div>
                        <div class="chip">Cargo</div>
                    </div>
                    <div class="suite-command">~/.cargo/bin/cargo check -p rev0auth-web</div>
                    <div class="mini">Vérifie que le frontend compile avant de pousser plus loin le dashboard.</div>
                </div>
            </div>

            <div class="log" id="log">Pret pour un sweep local. Clique sur &quot;Lancer le sweep&quot; pour verifier les routes visibles.</div>
        </section>
    </main>

    <script>
        const log = document.getElementById('log');

        function stamp() {
            const now = new Date();
            document.getElementById('last-run').textContent = 'Dernier sweep: ' + now.toLocaleTimeString();
            return now;
        }

        function setState(id, ok, label) {
            const el = document.getElementById(id);
            if (!el) return;
            el.textContent = label;
            el.className = 'state ' + (ok ? 'ok' : 'down');
        }

        function appendLog(text) {
            log.textContent += '\n' + text;
            log.scrollTop = log.scrollHeight;
        }

        async function check(path) {
            const response = await fetch(path, { cache: 'no-store' });
            const body = await response.text();
            return { ok: response.ok, body };
        }

        async function runSweep() {
            log.textContent = '';
            const now = stamp();
            appendLog('Sweep local demarre a ' + now.toLocaleTimeString());

            try {
                const [dashboardRes, statusRes, rootRes] = await Promise.all([
                    check('/status/all'),
                    check('/status'),
                    check('/dashboard')
                ]);

                setState('api-state', statusRes.ok, statusRes.ok ? 'API OK' : 'API DOWN');
                setState('web-state', dashboardRes.ok && rootRes.ok, dashboardRes.ok && rootRes.ok ? 'WEB OK' : 'WEB DOWN');
                setState('suite-state', dashboardRes.ok && statusRes.ok && rootRes.ok, dashboardRes.ok && statusRes.ok && rootRes.ok ? 'SWEEP OK' : 'SWEEP PARTIEL');

                appendLog('/status -> ' + (statusRes.ok ? 'OK' : 'FAIL'));
                appendLog('/status/all -> ' + (dashboardRes.ok ? 'OK' : 'FAIL'));
                appendLog('/dashboard -> ' + (rootRes.ok ? 'OK' : 'FAIL'));
                appendLog('Resultat: ' + (dashboardRes.ok && statusRes.ok && rootRes.ok ? 'tout est vert' : 'au moins un point est rouge'));
            } catch (error) {
                setState('api-state', false, 'API DOWN');
                setState('web-state', false, 'WEB DOWN');
                setState('suite-state', false, 'SWEEP FAIL');
                appendLog('Erreur sweep: ' + error.message);
            }
        }

        document.getElementById('run-sweep').addEventListener('click', runSweep);
        document.getElementById('clear-log').addEventListener('click', () => {
            log.textContent = 'Pret pour un sweep local. Clique sur "Lancer le sweep" pour verifier les routes visibles.';
            setState('api-state', false, 'En attente');
            setState('web-state', false, 'En attente');
            setState('suite-state', false, 'Pret');
            document.getElementById('last-run').textContent = 'Dernier sweep: --';
        });

        runSweep();
    </script>
</body>
</html>
"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}