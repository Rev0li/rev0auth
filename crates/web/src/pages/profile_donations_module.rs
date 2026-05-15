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

    async function loadCryptoAddresses() {
        try {
            const res = await fetch('/members/donations/crypto-addresses', { cache: 'no-store' });
            const list = await res.json();
            const container = document.getElementById('crypto-addresses-list');
            if (!container) return;
            if (!Array.isArray(list) || list.length === 0) {
                container.innerHTML = '<span style="color:var(--muted-foreground);font-size:0.875rem">Aucune adresse configuree.</span>';
                return;
            }
            container.innerHTML = list.map((a) =>
                '<div class="crypto-addr-row">'
                + '<span class="crypto-addr-name">' + escapeHtml(a.name) + '</span>'
                + '<code class="crypto-addr-val" title="Cliquer pour copier" onclick="navigator.clipboard.writeText(\'' + escapeHtml(a.address) + '\').then(() => this.classList.add(\'copied\'))">'
                + escapeHtml(a.address)
                + '</code>'
                + '</div>'
            ).join('');
        } catch (_) {}
    }

    function onMethodChange() {
        const method = document.getElementById('donation-method').value;
        const cryptoSection = document.getElementById('crypto-addresses-section');
        const pcsSection = document.getElementById('pcs-info-section');
        if (cryptoSection) {
            if (method === 'crypto') {
                cryptoSection.style.display = 'block';
                loadCryptoAddresses();
            } else {
                cryptoSection.style.display = 'none';
            }
        }
        if (pcsSection) {
            pcsSection.style.display = method === 'pcs' ? 'flex' : 'none';
        }
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
                const dt = new Date(row.created_at_epoch * 1000).toLocaleDateString('fr-FR');
                const isApproved = row.reviewed && row.approved;
                const isRefused = row.reviewed && !row.approved;
                const chipCls = isApproved ? ' approved' : isRefused ? ' refused' : '';
                const verdict = !row.reviewed
                    ? '<span class="don-pending">En attente</span>'
                    : (row.approved
                        ? '<span class="don-ok">✓ Validée</span>'
                        : '<span class="don-ko">✗ Refusée</span>');
                return '<div class="don-chip' + chipCls + '">'
                    + '<span class="don-ref">#' + row.id + '</span>'
                    + '<span class="don-sep">•</span>'
                    + escapeHtml(row.method)
                    + '<span class="don-sep">•</span>'
                    + escapeHtml(row.code)
                    + '<span class="don-sep">•</span>'
                    + dt
                    + '<span class="don-sep">•</span>'
                    + verdict
                    + '</div>';
            }).join('');
        } catch (err) {
            setMsg(false, 'Impossible de charger les preuves: ' + err.message);
        }
    }

    async function uploadDonation() {
        const method = document.getElementById('donation-method').value;
        const code = document.getElementById('donation-code').value.trim();
        if (!code) {
            setMsg(false, 'Entre un code/reference de transaction.');
            return;
        }

        try {
            const res = await fetch('/members/donations/proof', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: currentPseudo, method, code })
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Preuve envoyee.');
            if (data.ok) {
                document.getElementById('donation-code').value = '';
                await loadDonations();
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    const methodSelect = document.getElementById('donation-method');
    if (methodSelect) {
        methodSelect.addEventListener('change', onMethodChange);
        onMethodChange();
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
