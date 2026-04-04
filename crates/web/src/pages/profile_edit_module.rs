// Profile edit module for bio and commentary
pub const JS_PROFILE_EDIT_MODULE: &str = r#"
function createProfileEditModule(ctx) {
    const { pseudo } = ctx;
    let currentPseudo = pseudo;

    function setMsg(ok, text) {
        const el = document.getElementById('profile-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
    }

    async function loadProfileData() {
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
            const data = await res.json();
            if (data.ok && typeof data.bio === 'string') {
                document.getElementById('bio').value = data.bio;
                document.getElementById('commentary').value = data.commentary || '';
            }
        } catch (_err) {
            // Silent fail
        }
    }

    async function saveProfile() {
        const bio = document.getElementById('bio').value;
        const commentary = document.getElementById('commentary').value;
        try {
            const res = await fetch('/members/profile/data', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: currentPseudo, bio, commentary })
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Profil mis a jour.');
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('save-profile').addEventListener('click', saveProfile);

    return {
        loadProfileData,
        saveProfile,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
