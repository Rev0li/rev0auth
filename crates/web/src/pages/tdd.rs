use axum::response::Html;

use super::{frontend_theme, tdd_page_styles};

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
        %%TDD_PAGE_STYLES%%
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
                <a class="btn btn-songsurf" href="/japprends/songsurf-access">♪ SongSurf Admin →</a>
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
    .replace("%%TDD_PAGE_STYLES%%", tdd_page_styles::TDD_PAGE_STYLES)
        .replace("%%FRONTEND_SHARED_CSS%%", frontend_theme::FRONTEND_SHARED_CSS),
    )
}