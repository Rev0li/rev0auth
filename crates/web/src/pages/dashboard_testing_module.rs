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
                    + '<code class="test-path">' + cleanPath + '</code>'
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
                return '<div class="test-result-row">'
                    + '<span class="test-result-status ' + (result.ok ? 'ok' : 'down') + '">' + ok + '</span>'
                    + '<span class="test-result-name">' + (result.test_name || 'Test') + '</span>'
                    + '<span class="test-result-time">' + dt + '</span>'
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
