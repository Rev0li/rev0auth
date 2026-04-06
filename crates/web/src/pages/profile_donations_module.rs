// Profile donations module
pub const JS_PROFILE_DONATIONS_MODULE: &str = r#"
function createProfileDonationsModule(ctx) {
    const { pseudo, adminMode } = ctx;
    let currentPseudo = pseudo;

    function setMsg(ok, text) {
        const el = document.getElementById('donation-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
    }

    async function loadDonations() {
        if (adminMode) return;
        try {
            const res = await fetch('/members/donations?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('donations-list');
            if (!Array.isArray(list) || list.length === 0) {
                panel.textContent = 'Aucune preuve envoyee.';
                return;
            }

            panel.innerHTML = list.slice().reverse().map((row) => {
                const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                const verdict = !row.reviewed
                    ? 'En attente'
                    : (row.approved ? 'Validee' : 'Refusee');
                return '<div class="list-item">'
                    + '<div><strong>#' + row.id + '</strong> • ' + escapeHtml(row.method) + ' • ' + escapeHtml(row.code) + '</div>'
                    + '<div class="meta">' + dt + ' • ' + verdict + '</div>'
                    + '<div class="donation-proof-link"><a class="btn secondary" href="/members/donations/proof/' + row.id + '/photo" target="_blank" rel="noopener noreferrer">Voir photo</a></div>'
                    + '</div>';
            }).join('');
        } catch (err) {
            setMsg(false, 'Impossible de charger les preuves: ' + err.message);
        }
    }

    async function uploadDonation() {
        const method = document.getElementById('donation-method').value;
        const code = document.getElementById('donation-code').value.trim();
        const photo = document.getElementById('donation-photo');
        if (!code) {
            setMsg(false, 'Entre un code/reference.');
            return;
        }
        if (!photo.files || photo.files.length === 0) {
            setMsg(false, 'Ajoute une photo justificative.');
            return;
        }

        const form = new FormData();
        form.append('pseudo', currentPseudo);
        form.append('method', method);
        form.append('code', code);
        form.append('photo', photo.files[0]);

        try {
            const res = await fetch('/members/donations/proof', {
                method: 'POST',
                body: form
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Preuve envoyee.');
            if (data.ok) {
                document.getElementById('donation-code').value = '';
                photo.value = '';
                await loadDonations();
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('upload-donation').addEventListener('click', uploadDonation);
    document.getElementById('refresh-donations').addEventListener('click', loadDonations);
    bindEnterToClick('donation-code', 'upload-donation');

    return {
        loadDonations,
        uploadDonation,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
