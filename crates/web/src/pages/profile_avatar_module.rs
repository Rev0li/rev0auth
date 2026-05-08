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

    // Default animal avatars
    const DEFAULT_AVATARS = [
        { id: 'fox', name: 'Renard', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%23d4500a'/><polygon points='20,55 30,20 42,55' fill='%23d4500a'/><polygon points='58,55 70,20 80,55' fill='%23d4500a'/><polygon points='23,52 30,27 39,52' fill='%23f9b084'/><polygon points='61,52 70,27 77,52' fill='%23f9b084'/><circle cx='50' cy='60' r='22' fill='%23f9b084'/><ellipse cx='43' cy='54' rx='4' ry='4.5' fill='%231a1a1a'/><ellipse cx='57' cy='54' rx='4' ry='4.5' fill='%231a1a1a'/><circle cx='44' cy='53' r='1.2' fill='white'/><circle cx='58' cy='53' r='1.2' fill='white'/><ellipse cx='50' cy='64' rx='3' ry='2' fill='%231a1a1a'/><ellipse cx='50' cy='68' rx='9' ry='5' fill='%23fde4cc' opacity='0.7'/></svg>` },
        { id: 'wolf', name: 'Loup', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%234a5568'/><polygon points='18,52 28,15 40,52' fill='%234a5568'/><polygon points='60,52 72,15 82,52' fill='%234a5568'/><polygon points='21,50 28,22 37,50' fill='%239aa5b4'/><polygon points='63,50 72,22 79,50' fill='%239aa5b4'/><ellipse cx='50' cy='62' rx='24' ry='20' fill='%239aa5b4'/><ellipse cx='50' cy='71' rx='13' ry='9' fill='%23bec5cf'/><ellipse cx='42' cy='54' rx='4.5' ry='4' fill='%231a1a1a'/><ellipse cx='58' cy='54' rx='4.5' ry='4' fill='%231a1a1a'/><circle cx='43' cy='53' r='1.3' fill='%23e8f0fe'/><circle cx='59' cy='53' r='1.3' fill='%23e8f0fe'/><ellipse cx='50' cy='65' rx='4' ry='2.5' fill='%232d3748'/></svg>` },
        { id: 'cat', name: 'Chat', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%236b46c1'/><polygon points='22,52 32,18 44,52' fill='%236b46c1'/><polygon points='56,52 68,18 78,52' fill='%236b46c1'/><polygon points='25,50 32,25 41,50' fill='%23f9a8d4'/><polygon points='59,50 68,25 75,50' fill='%23f9a8d4'/><circle cx='50' cy='60' r='22' fill='%239f7aea'/><ellipse cx='42' cy='54' rx='5' ry='4' fill='%231a1a1a'/><ellipse cx='58' cy='54' rx='5' ry='4' fill='%231a1a1a'/><ellipse cx='42' cy='54' rx='2' ry='3.5' fill='%2352b788'/><ellipse cx='58' cy='54' rx='2' ry='3.5' fill='%2352b788'/><circle cx='43' cy='53' r='1' fill='white'/><polygon points='50,62 47,65 53,65' fill='%23f9a8d4'/><line x1='28' y1='64' x2='43' y2='67' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='28' y1='68' x2='43' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='67' x2='72' y2='64' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='68' x2='72' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/></svg>` },
        { id: 'eagle', name: 'Aigle', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%231a202c'/><circle cx='50' cy='56' r='24' fill='%23744210'/><circle cx='50' cy='48' r='17' fill='%23f7fafc'/><circle cx='44' cy='46' r='5' fill='%23f6ad55'/><circle cx='44' cy='46' r='3' fill='%231a1a1a'/><circle cx='45' cy='45' r='1' fill='white'/><polygon points='35,52 50,48 37,60' fill='%23f6ad55'/><ellipse cx='63' cy='62' rx='12' ry='8' fill='%232d3748'/><ellipse cx='37' cy='63' rx='10' ry='7' fill='%23744210'/></svg>` },
        { id: 'dragon', name: 'Dragon', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%23065f46'/><polygon points='38,30 34,10 42,28' fill='%2334d399'/><polygon points='62,30 66,10 58,28' fill='%2334d399'/><circle cx='50' cy='58' r='24' fill='%23059669'/><ellipse cx='50' cy='70' rx='12' ry='9' fill='%2334d399'/><circle cx='46' cy='69' r='2' fill='%23065f46'/><circle cx='54' cy='69' r='2' fill='%23065f46'/><ellipse cx='41' cy='52' rx='5.5' ry='4' fill='%23f59e0b'/><ellipse cx='59' cy='52' rx='5.5' ry='4' fill='%23f59e0b'/><ellipse cx='41' cy='52' rx='1.5' ry='4' fill='%231a1a1a'/><ellipse cx='59' cy='52' rx='1.5' ry='4' fill='%231a1a1a'/><path d='M38,64 Q50,58 62,64' fill='none' stroke='%2334d399' stroke-width='1.5' opacity='0.6'/></svg>` },
    ];

    function renderDefaultAvatars() {
        const grid = document.getElementById('default-avatar-grid');
        if (!grid) return;
        grid.innerHTML = DEFAULT_AVATARS.map((a) => {
            const src = 'data:image/svg+xml;charset=UTF-8,' + a.svg;
            return '<button class="default-avatar-btn" data-id="' + a.id + '" title="' + a.name + '" type="button">'
                + '<img src="' + src + '" alt="' + a.name + '" />'
                + '<span>' + a.name + '</span>'
                + '</button>';
        }).join('');

        grid.querySelectorAll('.default-avatar-btn').forEach((btn) => {
            btn.addEventListener('click', async () => {
                const id = btn.getAttribute('data-id');
                const found = DEFAULT_AVATARS.find((a) => a.id === id);
                if (!found) return;
                grid.querySelectorAll('.default-avatar-btn').forEach((b) => b.classList.remove('selected'));
                btn.classList.add('selected');
                await applyDefaultAvatar(found);
            });
        });
    }

    async function applyDefaultAvatar(avatarDef) {
        const svgBlob = new Blob([avatarDef.svg.replace(/%23/g, '#').replace(/%27/g, "'")], { type: 'image/svg+xml' });
        const file = new File([svgBlob], avatarDef.id + '.svg', { type: 'image/svg+xml' });
        const form = new FormData();
        form.append('pseudo', currentPseudo);
        form.append('avatar', file);
        try {
            const res = await fetch('/members/avatar', { method: 'POST', body: form });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Avatar applique.');
            if (data.ok) await loadAvatarState();
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('delete-avatar').addEventListener('click', deleteAvatar);
    renderDefaultAvatars();

    return {
        loadAvatarState,
        uploadAvatar,
        deleteAvatar,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
