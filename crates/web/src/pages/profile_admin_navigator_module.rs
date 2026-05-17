// Profile admin navigator module for switching between users
pub const JS_PROFILE_ADMIN_NAVIGATOR_MODULE: &str = r#"
function createProfileAdminNavigatorModule(ctx) {
    const { adminMode } = ctx;
    let currentPseudo = ctx.pseudo;
    let adminUsers = [];

    function updateAdminNavMeta() {
        if (!adminMode) return;
        const idx = adminUsers.findIndex((p) => p.toLowerCase() === currentPseudo.toLowerCase());
        const total = adminUsers.length;
        const meta = document.getElementById('admin-nav-meta');
        if (idx >= 0 && total > 0) {
            meta.textContent = 'User ' + (idx + 1) + ' / ' + total;
        } else {
            meta.textContent = 'User -- / --';
        }
    }

    async function loadAdminUsersNavigator() {
        if (!adminMode) return;
        try {
            const res = await fetch('/users', { cache: 'no-store' });
            const list = await res.json();
            adminUsers = Array.isArray(list) ? list.map((u) => u.pseudo) : [];
            if (adminUsers.length > 0 && !adminUsers.some((p) => p.toLowerCase() === currentPseudo.toLowerCase())) {
                currentPseudo = adminUsers[0];
            }
            updateAdminNavMeta();
        } catch (_err) {
            document.getElementById('admin-nav-meta').textContent = 'Navigation users indisponible';
        }
    }

    function goToAdminUser(offset) {
        if (!adminMode || adminUsers.length === 0) return;
        const idx = adminUsers.findIndex((p) => p.toLowerCase() === currentPseudo.toLowerCase());
        if (idx < 0) return;

        const nextIdx = idx + offset;
        if (nextIdx < 0 || nextIdx >= adminUsers.length) return;

        const nextPseudo = adminUsers[nextIdx];
        const url = '/members/profile?pseudo=' + encodeURIComponent(nextPseudo) + '&admin=1';
        window.location.href = url;
    }

    async function deleteUser(pseudo) {
        if (!confirm('Supprimer l\'utilisateur ' + pseudo + ' ?')) return;

        try {
            const res = await fetch('/japprends/users/' + pseudo, {
                method: 'DELETE'
            });
            const data = await res.json();

            if (data.ok) {
                await loadAdminUsersNavigator();
                if (adminUsers.length > 0) {
                    const url = '/members/profile?pseudo=' + encodeURIComponent(adminUsers[0]) + '&admin=1';
                    window.location.href = url;
                } else {
                    window.location.href = '/dashboard';
                }
            } else {
                alert('Erreur: ' + data.message);
            }
        } catch (err) {
            alert('Erreur: ' + err.message);
        }
    }

    function openUserProfile(pseudo) {
        const target = '/members/profile?pseudo=' + encodeURIComponent(pseudo) + '&admin=1';
        window.location.href = target;
    }

    async function loadApprovalStatus(pseudo) {
        if (!adminMode) return;
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
            const data = await res.json();
            const approved = !!data.approved;
            const btn = document.getElementById('approve-toggle-btn');
            const label = document.getElementById('approve-status-label');
            if (!btn || !label) return;
            if (approved) {
                btn.textContent = '✗ Révoquer';
                btn.className = 'btn-profile-action danger';
                label.textContent = 'Compte approuvé';
            } else {
                btn.textContent = '✓ Approuver';
                btn.className = 'btn-profile-action';
                label.textContent = 'En attente d\'approbation';
            }
            btn.style.display = 'inline-block';
            btn.onclick = () => toggleApprove(pseudo, !approved);
        } catch (_) {}
    }

    async function toggleApprove(pseudo, approve) {
        const msg = document.getElementById('approve-msg');
        try {
            const res = await fetch('/japprends/users/' + encodeURIComponent(pseudo), {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ approved: approve })
            });
            const data = await res.json();
            if (msg) {
                msg.className = 'msg ' + (data.ok ? 'ok' : 'down');
                msg.textContent = data.ok ? (approve ? 'Compte approuvé.' : 'Approbation révoquée.') : data.message;
                msg.style.display = 'block';
            }
            if (data.ok) loadApprovalStatus(pseudo);
        } catch (err) {
            if (msg) { msg.className = 'msg down'; msg.textContent = 'Erreur: ' + err.message; msg.style.display = 'block'; }
        }
    }

    async function sendAdminMessage(pseudo) {
        const body = (document.getElementById('admin-msg-body') || {}).value?.trim();
        const result = document.getElementById('admin-msg-result');
        if (!body) {
            if (result) { result.className = 'msg down'; result.textContent = 'Message vide.'; result.style.display = 'block'; }
            return;
        }
        try {
            const res = await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to_pseudo: pseudo, body })
            });
            const data = await res.json();
            if (result) {
                result.className = 'msg ' + (data.ok ? 'ok' : 'down');
                result.textContent = data.ok ? 'Message envoyé.' : data.message;
                result.style.display = 'block';
            }
            if (data.ok) { const ta = document.getElementById('admin-msg-body'); if (ta) ta.value = ''; }
        } catch (err) {
            if (result) { result.className = 'msg down'; result.textContent = 'Erreur: ' + err.message; result.style.display = 'block'; }
        }
    }

    if (adminMode) {
        document.getElementById('prev-user').addEventListener('click', () => goToAdminUser(-1));
        document.getElementById('next-user').addEventListener('click', () => goToAdminUser(1));
        const sendBtn = document.getElementById('admin-send-msg-btn');
        if (sendBtn) sendBtn.addEventListener('click', () => sendAdminMessage(ctx.pseudo));
    }

    return {
        loadAdminUsersNavigator,
        loadApprovalStatus,
        goToAdminUser,
        deleteUser,
        openUserProfile,
        updateAdminNavMeta,
        getUsers: () => adminUsers.slice(),
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
