// Dashboard testing and endpoints module
pub const JS_DASHBOARD_TESTING_MODULE: &str = r#"
function createDashboardTestingModule(ctx) {

    async function loadEndpoints() {
        try {
            const res = await fetch('/japprends/endpoints', { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('admin-endpoints');
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<p class="label">Aucun endpoint.</p>';
                return;
            }

            const html = '<div class="grid">' + list.map((ep) => {
                const cleanPath = (ep.path || '').replace(/^\//, '').slice(0, 50);
                return '<div class="endpoint-card" title="' + (ep.method || 'N/A') + '">'
                    + '<code style="word-break:break-all;font-size:0.75rem;">' + cleanPath + '</code>'
                    + '</div>';
            }).join('') + '</div>';

            panel.innerHTML = html;
        } catch (err) {
            panel.innerHTML = '<p class="label">Erreur: ' + err.message + '</p>';
        }
    }

    async function loadTestsHistory() {
        try {
            const res = await fetch('/japprends/test-history', { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('admin-test-history');
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<p>Aucun test.</p>';
                return;
            }

            const html = list.reverse().slice(0, 8).map((result) => {
                const dt = new Date(result.epoch * 1000).toLocaleTimeString();
                const ok = result.ok ? '✓' : '✗';
                const color = result.ok ? '#0d9b73' : '#dc4f2f';
                return '<div style="display:flex;gap:8px;margin-bottom:6px;align-items:center;">'
                    + '<span style="color:' + color + ';font-weight:700;">' + ok + '</span>'
                    + '<span style="font-size:0.85rem;flex:1;">' + (result.test_name || 'Test') + '</span>'
                    + '<span style="font-size:0.75rem;opacity:0.7;">' + dt + '</span>'
                    + '</div>';
            }).join('');

            panel.innerHTML = html;
        } catch (err) {
            panel.innerHTML = '<p>Erreur: ' + err.message + '</p>';
        }
    }

    async function launchTestsNow() {
        try {
            const res = await fetch('/japprends/run-tests', { method: 'POST', cache: 'no-store' });
            const data = await res.json();
            alert((data.ok ? '✓ ' : '✗ ') + (data.message || 'Tests lancessss!'));
            await loadTestsHistory();
        } catch (err) {
            alert('Erreur: ' + err.message);
        }
    }

    document.getElementById('launch-tests-now').addEventListener('click', launchTestsNow);

    return {
        loadEndpoints,
        loadTestsHistory,
        launchTestsNow
    };
}
"#;
