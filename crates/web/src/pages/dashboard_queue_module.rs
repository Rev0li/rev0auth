// Dashboard signup queue module
pub const JS_DASHBOARD_QUEUE_MODULE: &str = r#"
function createDashboardQueueModule(ctx) {

    async function loadAdminSignupQueue() {
        try {
            const res = await fetch('/japprends/signup-requests', { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('admin-queue');
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<p>Aucune demande en attente.</p>';
                return;
            }

            const html = '<div class="grid">' + list.map((request) => {
                const dt = new Date(request.epoch * 1000).toLocaleString();
                return '<div class="card">'
                    + '<h3 class="queue-title">Pseudo tentative</h3>'
                    + '<p class="meta queue-meta">' + dt + '</p>'
                    + '</div>';
            }).join('') + '</div>';

            panel.innerHTML = html;
        } catch (err) {
            panel.innerHTML = '<p>Erreur: ' + err.message + '</p>';
        }
    }

    return {
        loadAdminSignupQueue
    };
}
"#;
