// Profile avatar module for upload/delete
pub const JS_PROFILE_AVATAR_MODULE: &str = r#"
function createProfileAvatarModule(ctx) {
    const { pseudo } = ctx;
    let currentPseudo = pseudo;
    let currentAvatarObjectUrl = null;

    function setMsg(ok, text) {
        const el = document.getElementById('avatar-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
    }

    function setAvatarState(text, visible) {
        const state = document.getElementById('avatar-state');
        state.textContent = text;
        const preview = document.getElementById('avatar-preview');
        preview.style.display = visible ? 'block' : 'none';
    }

    function clearAvatarObjectUrl() {
        if (currentAvatarObjectUrl) {
            URL.revokeObjectURL(currentAvatarObjectUrl);
            currentAvatarObjectUrl = null;
        }
    }

    function showAvatarPreviewFromFile(file) {
        clearAvatarObjectUrl();
        currentAvatarObjectUrl = URL.createObjectURL(file);
        const preview = document.getElementById('avatar-preview');
        preview.src = currentAvatarObjectUrl;
        preview.style.display = 'block';
        setAvatarState('Prévisualisation locale avant envoi.', true);
    }

    async function loadAvatarState() {
        try {
            const res = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
            const data = await res.json();
            const preview = document.getElementById('avatar-preview');
            if (data.avatar_present) {
                clearAvatarObjectUrl();
                preview.src = '/members/avatar/' + encodeURIComponent(currentPseudo) + '?t=' + (data.created_at_epoch || Date.now());
                setAvatarState(data.avatar_filename ? 'Avatar: ' + data.avatar_filename : 'Avatar present', true);
            } else {
                clearAvatarObjectUrl();
                preview.removeAttribute('src');
                setAvatarState('Aucun avatar pour le moment.', false);
            }
        } catch (_err) {
            // Silent fail
        }
    }

    async function uploadAvatar() {
        const input = document.getElementById('avatar');
        if (!input.files || input.files.length === 0) {
            setMsg(false, 'Choisis un fichier image.');
            return;
        }

        showAvatarPreviewFromFile(input.files[0]);

        const form = new FormData();
        form.append('pseudo', currentPseudo);
        form.append('avatar', input.files[0]);

        try {
            const res = await fetch('/members/avatar', {
                method: 'POST',
                body: form
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Avatar mis a jour.');
            if (data.ok) {
                input.value = '';
                await loadAvatarState();
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    async function deleteAvatar() {
        if (!confirm('Supprimer l\'avatar de ce profil ?')) return;
        try {
            const res = await fetch('/members/avatar/' + encodeURIComponent(currentPseudo), {
                method: 'DELETE'
            });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Avatar supprime.');
            if (data.ok) {
                clearAvatarObjectUrl();
                await loadAvatarState();
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('upload-avatar').addEventListener('click', uploadAvatar);
    document.getElementById('delete-avatar').addEventListener('click', deleteAvatar);

    return {
        loadAvatarState,
        uploadAvatar,
        deleteAvatar,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
