// Dashboard status and monitoring module
pub const JS_DASHBOARD_STATUS_MODULE: &str = r#"
function createDashboardStatusModule(ctx) {
    const { dashboardStats } = ctx;

    function renderAdminStats() {
        const stats = dashboardStats;
        const userCount = (stats.users || []).length;
        const loggedCount = (stats.logged_in_users || []).length;
        const html = '<div class="grid">'
            + '<div class="card"><h2 style="margin-top:0;">' + userCount + '</h2><p class="label">Total users</p></div>'
            + '<div class="card"><h2 style="margin-top:0;">' + loggedCount + '</h2><p class="label">Logged in now</p></div>'
            + '</div>';
        
        const container = document.getElementById('admin-stats-summary');
        if (container) {
            container.innerHTML = html;
        }
    }

    async function refreshStatus() {
        try {
            const res = await fetch('/status', { cache: 'no-store' });
            const data = await res.json();
            
            dashboardStats.logged_in_users = data.logged_in_users || [];
            dashboardStats.uptime = data.uptime || 0;
            
            renderAdminStats();
        } catch (_err) {
            // Silent fail
        }
    }

    return {
        refreshStatus,
        renderAdminStats
    };
}
"#;
