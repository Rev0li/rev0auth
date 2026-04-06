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

    if (adminMode) {
        document.getElementById('prev-user').addEventListener('click', () => goToAdminUser(-1));
        document.getElementById('next-user').addEventListener('click', () => goToAdminUser(1));
    }

    return {
        loadAdminUsersNavigator,
        goToAdminUser,
        deleteUser,
        openUserProfile,
        updateAdminNavMeta,
        getUsers: () => adminUsers.slice(),
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
