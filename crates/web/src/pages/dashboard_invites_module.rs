pub const JS_DASHBOARD_INVITES_MODULE: &str = r#"
async function loadInvites() {
    const list = document.getElementById('invites-list');
    if (!list) return;
    try {
        const res = await fetch('/japprends/invites', { cache: 'no-store' });
        const data = await res.json();
        if (!Array.isArray(data) || data.length === 0) {
            list.innerHTML = '<p class="invites-empty">Aucune invitation générée.</p>';
            return;
        }
        const now = Math.floor(Date.now() / 1000);
        list.innerHTML = data.map(inv => {
            const expired = inv.expires_at_epoch < now;
            const used = !!inv.used_by;
            const dt = new Date(inv.created_at_epoch * 1000).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' });
            const expDt = new Date(inv.expires_at_epoch * 1000).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' });
            let badge = '';
            if (used) badge = '<span class="inv-badge used">Utilisé par ' + escapeHtml(inv.used_by) + '</span>';
            else if (expired) badge = '<span class="inv-badge expired">Expiré</span>';
            else badge = '<span class="inv-badge active">Actif</span>';
            const noteHtml = inv.note ? '<span class="inv-note">' + escapeHtml(inv.note) + '</span>' : '';
            const revokeBtn = (!used && !expired)
                ? '<button class="inv-revoke-btn" data-id="' + inv.id + '">Révoquer</button>'
                : '';
            const link = window.location.origin + '/signup?invite=' + inv.code;
            const copyBtn = (!used && !expired)
                ? '<button class="inv-copy-btn" data-link="' + link + '">Copier le lien</button>'
                : '';
            return '<div class="inv-row">'
                + '<div class="inv-meta">'
                +   '<div class="inv-meta-top">' + badge + noteHtml + '</div>'
                +   '<span class="inv-dates">Créé ' + dt + ' · Expire ' + expDt + '</span>'
                + '</div>'
                + '<div class="inv-actions">' + copyBtn + revokeBtn + '</div>'
                + '</div>';
        }).join('');

        list.querySelectorAll('.inv-revoke-btn').forEach(btn => {
            btn.addEventListener('click', async () => {
                const id = btn.getAttribute('data-id');
                if (!confirm('Révoquer cette invitation ?')) return;
                const r = await fetch('/japprends/invites/' + id, { method: 'DELETE' });
                const d = await r.json();
                if (d.ok) loadInvites();
                else alert(d.message || 'Erreur.');
            });
        });

        list.querySelectorAll('.inv-copy-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                navigator.clipboard.writeText(btn.getAttribute('data-link')).then(() => {
                    btn.textContent = '✓ Copié';
                    setTimeout(() => { btn.textContent = 'Copier le lien'; }, 2000);
                });
            });
        });
    } catch (e) {
        list.innerHTML = '<p class="invites-empty">Erreur de chargement.</p>';
    }
}

function escapeHtml(s) {
    return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}

(function initInvites() {
    const form = document.getElementById('invite-gen-form');
    const msgEl = document.getElementById('invite-gen-msg');
    if (!form) return;

    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const note = document.getElementById('invite-note').value.trim();
        msgEl.className = 'inv-gen-msg';
        msgEl.textContent = '';
        try {
            const res = await fetch('/japprends/invites', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ note })
            });
            const data = await res.json();
            if (data.ok) {
                msgEl.className = 'inv-gen-msg ok';
                const link = window.location.origin + '/signup?invite=' + data.code;
                msgEl.innerHTML = 'Lien généré : <button class="inv-copy-btn" data-link="' + link + '" style="display:inline-block;margin-left:6px">Copier</button>';
                msgEl.querySelector('.inv-copy-btn').addEventListener('click', () => {
                    navigator.clipboard.writeText(link).then(() => {
                        msgEl.querySelector('.inv-copy-btn').textContent = '✓ Copié';
                    });
                });
                document.getElementById('invite-note').value = '';
                loadInvites();
            } else {
                msgEl.className = 'inv-gen-msg err';
                msgEl.textContent = data.message || 'Erreur.';
            }
        } catch {
            msgEl.className = 'inv-gen-msg err';
            msgEl.textContent = 'Erreur réseau.';
        }
    });

    loadInvites();
})();
"#;


pub const HTML_INVITES_TAB: &str = r#"
<section class="tab-page" id="tab-invitations">
    <div class="card">
        <h2 class="card-title">Générer une invitation</h2>
        <form id="invite-gen-form" class="inv-gen-form">
            <input id="invite-note" type="text" placeholder="Note optionnelle (ex: pour Alice)" maxlength="80" />
            <button type="submit">Générer un lien</button>
        </form>
        <div id="invite-gen-msg" class="inv-gen-msg"></div>
        <div id="invites-list"></div>
    </div>
</section>
"#;
