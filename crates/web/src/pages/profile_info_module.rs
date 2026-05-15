// Profile info display module
pub const JS_PROFILE_INFO_MODULE: &str = r#"
function createProfileInfoModule(ctx) {
    const { pseudo, adminMode } = ctx;
    let currentPseudo = pseudo;

    function statusEmoji(status) {
        const raw = String(status || '').toLowerCase();
        if (raw === 'actif') return '😀';
        if (raw === 'occupe') return '😐';
        return '❓';
    }

    async function loadProfile() {
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
            const data = await res.json();
            if (data.ok && typeof data.bio === 'string') {
                document.getElementById('info-pseudo').textContent = data.pseudo || '--';
                document.getElementById('info-role').textContent = data.role || '--';
                document.getElementById('info-status').textContent = data.status || '--';
                document.getElementById('info-status-emoji').textContent = statusEmoji(data.status);
                document.getElementById('info-commentary').textContent = data.commentary || 'Aucun commentaire.';
                document.getElementById('info-created').textContent = data.created_at_epoch ? new Date(data.created_at_epoch * 1000).toLocaleString() : '--';
                return data;
            }
        } catch (_err) {
            // Silent fail
        }
        return null;
    }

    return {
        loadProfile,
        statusEmoji,
        getCurrentPseudo: () => currentPseudo,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
