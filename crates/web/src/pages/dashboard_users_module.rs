// Dashboard users management module
pub const JS_DASHBOARD_USERS_MODULE: &str = r#"
function createDashboardUsersModule(ctx) {
    const { dashboardStats } = ctx;

    function renderAdminStats() {
        if (!document.getElementById('admin-stats')) return;
        const users = dashboardStats.users || [];
        const total = users.length;
        
        // Stats are rendered by parent, we just store the data
        console.log('Users stats updated: ' + total + ' users');
    }

    async function loadUsers() {
        try {
            const res = await fetch('/users', { cache: 'no-store' });
            const list = await res.json();
            const panel = document.getElementById('admin-stats');
            if (!Array.isArray(list) || list.length === 0) {
                panel.innerHTML = '<div class="card"><p>Aucun utilisateur.</p></div>';
                dashboardStats.users = [];
                return;
            }

            const html = list.map((user) => {
                const ghLabel = user.access_github ? '🟢 Revoke GH' : '🔴 Grant GH';
                const jfLabel = user.access_jellyfin ? '🟢 Revoke JF' : '🔴 Grant JF';
                const ssLabel = user.access_songsurf ? '🟢 Revoke SS' : '🔴 Grant SS';

                return '<div class="card" onclick="openUserProfile(\'' + user.pseudo + '\')" style="cursor:pointer;">'
                    + '<h3 style="margin:0 0 8px;">' + user.pseudo + '</h3>'
                    + '<p class="label">Status: ' + (user.status ? user.status.toUpperCase() : 'UNKNOWN') + '</p>'
                    + '<p class="label">Requete ou accès</p>'
                    + '<div class="user-actions">'
                    + '<button class="btn-small warn" onclick="event.stopPropagation(); openUserProfile(\'' + user.pseudo + '\')">Profil complet</button>'
                    + '<button class="btn-small ' + (user.access_github ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'github\', ' + (!user.access_github) + ')">' + ghLabel + '</button>'
                    + '<button class="btn-small ' + (user.access_jellyfin ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'jellyfin\', ' + (!user.access_jellyfin) + ')">' + jfLabel + '</button>'
                    + '<button class="btn-small ' + (user.access_songsurf ? 'danger' : 'grant') + '" onclick="event.stopPropagation(); toggleServiceAccess(\'' + user.pseudo + '\', \'songsurf\', ' + (!user.access_songsurf) + ')">' + ssLabel + '</button>'
                    + '<button class="btn-small" onclick="event.stopPropagation(); deleteUser(\'' + user.pseudo + '\')">🗑 Supprimer</button>'
                    + '</div>'
                    + '</div>';
            }).join('');

            dashboardStats.users = list;
            renderAdminStats();
            panel.innerHTML = html;
        } catch (err) {
            console.log('Error loading users:', err);
        }
    }

    async function toggleServiceAccess(pseudo, service, nextValue) {
        const payload = {};
        if (service === 'github') payload.access_github = !!nextValue;
        if (service === 'jellyfin') payload.access_jellyfin = !!nextValue;
        if (service === 'songsurf') payload.access_songsurf = !!nextValue;

        try {
            const res = await fetch('/japprends/users/' + pseudo, {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(payload)
            });
            const data = await res.json();
            if (!data.ok) {
                alert('Erreur: ' + data.message);
                return;
            }
            await loadUsers();
        } catch (err) {
            alert('Erreur: ' + err.message);
        }
    }

    async function createUser() {
        const pseudoInput = document.getElementById('new-pseudo');
        const msg = document.getElementById('create-msg');
        if (!pseudoInput || !msg) return;
        const pseudo = pseudoInput.value.trim();

        if (!pseudo) {
            msg.style.color = '#dc4f2f';
            msg.textContent = 'Entre un pseudo.';
            msg.style.display = 'block';
            return;
        }

        try {
            const res = await fetch('/japprends/users', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo })
            });
            const data = await res.json();

            if (data.ok) {
                msg.style.color = '#0d9b73';
                msg.textContent = '✓ ' + data.message + (data.temp_password ? ' Mot de passe: ' + data.temp_password : '');
                if (data.temp_password) copyTempPassword(data.temp_password);
                pseudoInput.value = '';
                loadUsers();
            } else {
                msg.style.color = '#dc4f2f';
                msg.textContent = '✗ ' + data.message;
            }
            msg.style.display = 'block';
        } catch (err) {
            msg.style.color = '#dc4f2f';
            msg.textContent = 'Erreur: ' + err.message;
            msg.style.display = 'block';
        }
    }

    async function deleteUser(pseudo) {
        if (!confirm('Supprimer l\'utilisateur ' + pseudo + ' ?')) return;

        try {
            const res = await fetch('/japprends/users/' + pseudo, {
                method: 'DELETE'
            });
            const data = await res.json();

            if (data.ok) {
                loadUsers();
            } else {
                alert('Erreur: ' + data.message);
            }
        } catch (err) {
            alert('Erreur: ' + err.message);
        }
    }

    const createUserBtn = document.getElementById('create-user-btn');
    if (createUserBtn) {
        createUserBtn.addEventListener('click', createUser);
    }

    return {
        loadUsers,
        createUser,
        deleteUser,
        toggleServiceAccess,
        renderAdminStats
    };
}
"#;
