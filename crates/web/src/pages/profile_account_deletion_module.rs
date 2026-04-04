// Profile account deletion module
pub const JS_PROFILE_ACCOUNT_DELETION_MODULE: &str = r#"
function createProfileAccountDeletionModule(ctx) {
    const { pseudo, adminMode } = ctx;
    let currentPseudo = pseudo;
    let adminNavigatorModule = null;

    function setMsg(ok, text) {
        const el = document.getElementById('delete-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
    }

    async function deleteAccount() {
        if (!confirm('Supprimer ton compte definitivement ?')) return;
        try {
            const res = await fetch('/members/account', {
                method: 'DELETE',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: currentPseudo })
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Action terminee.');
            if (data.ok) {
                if (!adminMode) {
                    localStorage.removeItem('logged_pseudo');
                    setTimeout(() => {
                        window.location.href = '/';
                    }, 500);
                } else {
                    // Admin mode: use navigator to go to next user or dashboard
                    if (adminNavigatorModule) {
                        await adminNavigatorModule.loadAdminUsersNavigator();
                        adminNavigatorModule.deleteUser(currentPseudo);
                    } else {
                        window.location.href = '/dashboard';
                    }
                }
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('delete-account').addEventListener('click', deleteAccount);

    return {
        deleteAccount,
        setAdminNavigatorModule: (m) => { adminNavigatorModule = m; },
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
