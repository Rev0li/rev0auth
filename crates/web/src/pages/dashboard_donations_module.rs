// Dashboard donations review module
pub const JS_DASHBOARD_DONATIONS_MODULE: &str = r#"
function createDashboardDonationsModule(ctx) {

    async function loadAdminDonations() {
        try {
            const res = await fetch('/japprends/donations', { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('admin-donations');
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<div class="card"><p>Aucune donation.</p></div>';
                return;
            }

            const html = list.map((row) => {
                const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                const status = !row.reviewed ? 'En attente' : (row.approved ? '✓ Validee' : '✗ Refusee');
                return '<div class="card">'
                    + '<h3 style="margin:0 0 4px;">#' + row.id + ' de ' + row.pseudo + '</h3>'
                    + '<p class="label">' + (row.method || 'N/A') + ' • ' + (row.code || 'N/A') + '</p>'
                    + '<p class="meta">' + dt + ' • ' + status + '</p>'
                    + '<p style="font-size:0.8rem;margin:0;"><a href="/members/donations/proof/' + row.id + '/photo" target="_blank">Voir photo</a></p>'
                    + '</div>';
            }).join('');

            panel.innerHTML = html;
        } catch (err) {
            console.log('Error loading donations:', err);
        }
    }

    return {
        loadAdminDonations
    };
}
"#;
