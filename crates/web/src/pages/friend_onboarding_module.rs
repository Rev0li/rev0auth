// Onboarding module for first-time login password change and avatar selection
pub const JS_FRIEND_ONBOARDING_MODULE: &str = r#"
function createFriendOnboardingModule(ctx) {
    const { pseudo, avatarModule } = ctx;

    const DEFAULT_AVATARS = [
        { id: 'fox', name: 'Renard', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%23d4500a'/><polygon points='20,55 30,20 42,55' fill='%23d4500a'/><polygon points='58,55 70,20 80,55' fill='%23d4500a'/><polygon points='23,52 30,27 39,52' fill='%23f9b084'/><polygon points='61,52 70,27 77,52' fill='%23f9b084'/><circle cx='50' cy='60' r='22' fill='%23f9b084'/><ellipse cx='43' cy='54' rx='4' ry='4.5' fill='%231a1a1a'/><ellipse cx='57' cy='54' rx='4' ry='4.5' fill='%231a1a1a'/><circle cx='44' cy='53' r='1.2' fill='white'/><circle cx='58' cy='53' r='1.2' fill='white'/><ellipse cx='50' cy='64' rx='3' ry='2' fill='%231a1a1a'/><ellipse cx='50' cy='68' rx='9' ry='5' fill='%23fde4cc' opacity='0.7'/></svg>` },
        { id: 'wolf', name: 'Loup', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%234a5568'/><polygon points='18,52 28,15 40,52' fill='%234a5568'/><polygon points='60,52 72,15 82,52' fill='%234a5568'/><polygon points='21,50 28,22 37,50' fill='%239aa5b4'/><polygon points='63,50 72,22 79,50' fill='%239aa5b4'/><ellipse cx='50' cy='62' rx='24' ry='20' fill='%239aa5b4'/><ellipse cx='50' cy='71' rx='13' ry='9' fill='%23bec5cf'/><ellipse cx='42' cy='54' rx='4.5' ry='4' fill='%231a1a1a'/><ellipse cx='58' cy='54' rx='4.5' ry='4' fill='%231a1a1a'/><circle cx='43' cy='53' r='1.3' fill='%23e8f0fe'/><circle cx='59' cy='53' r='1.3' fill='%23e8f0fe'/><ellipse cx='50' cy='65' rx='4' ry='2.5' fill='%232d3748'/></svg>` },
        { id: 'cat', name: 'Chat', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%236b46c1'/><polygon points='22,52 32,18 44,52' fill='%236b46c1'/><polygon points='56,52 68,18 78,52' fill='%236b46c1'/><polygon points='25,50 32,25 41,50' fill='%23f9a8d4'/><polygon points='59,50 68,25 75,50' fill='%23f9a8d4'/><circle cx='50' cy='60' r='22' fill='%239f7aea'/><ellipse cx='42' cy='54' rx='5' ry='4' fill='%231a1a1a'/><ellipse cx='58' cy='54' rx='5' ry='4' fill='%231a1a1a'/><ellipse cx='42' cy='54' rx='2' ry='3.5' fill='%2352b788'/><ellipse cx='58' cy='54' rx='2' ry='3.5' fill='%2352b788'/><circle cx='43' cy='53' r='1' fill='white'/><polygon points='50,62 47,65 53,65' fill='%23f9a8d4'/><line x1='28' y1='64' x2='43' y2='67' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='28' y1='68' x2='43' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='67' x2='72' y2='64' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='68' x2='72' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/></svg>` },
        { id: 'eagle', name: 'Aigle', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%231a202c'/><circle cx='50' cy='56' r='24' fill='%23744210'/><circle cx='50' cy='48' r='17' fill='%23f7fafc'/><circle cx='44' cy='46' r='5' fill='%23f6ad55'/><circle cx='44' cy='46' r='3' fill='%231a1a1a'/><circle cx='45' cy='45' r='1' fill='white'/><polygon points='35,52 50,48 37,60' fill='%23f6ad55'/><ellipse cx='63' cy='62' rx='12' ry='8' fill='%232d3748'/><ellipse cx='37' cy='63' rx='10' ry='7' fill='%23744210'/></svg>` },
        { id: 'dragon', name: 'Dragon', svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='%23065f46'/><polygon points='38,30 34,10 42,28' fill='%2334d399'/><polygon points='62,30 66,10 58,28' fill='%2334d399'/><circle cx='50' cy='58' r='24' fill='%23059669'/><ellipse cx='50' cy='70' rx='12' ry='9' fill='%2334d399'/><circle cx='46' cy='69' r='2' fill='%23065f46'/><circle cx='54' cy='69' r='2' fill='%23065f46'/><ellipse cx='41' cy='52' rx='5.5' ry='4' fill='%23f59e0b'/><ellipse cx='59' cy='52' rx='5.5' ry='4' fill='%23f59e0b'/><ellipse cx='41' cy='52' rx='1.5' ry='4' fill='%231a1a1a'/><ellipse cx='59' cy='52' rx='1.5' ry='4' fill='%231a1a1a'/><path d='M38,64 Q50,58 62,64' fill='none' stroke='%2334d399' stroke-width='1.5' opacity='0.6'/></svg>` },
    ];

    let selectedAvatarId = null;

    function setOnboardingMsg(ok, message) {
        const el = document.getElementById('onboarding-msg');
        el.className = 'onboarding-msg ' + (ok ? 'ok' : 'error');
        el.textContent = message;
    }

    function openOnboardingModal() {
        document.getElementById('onboarding-modal').style.display = 'flex';
    }

    function closeOnboardingModal() {
        document.getElementById('onboarding-modal').style.display = 'none';
    }

    function renderAvatarGrid() {
        const grid = document.getElementById('onboarding-avatar-grid');
        if (!grid) return;
        grid.innerHTML = DEFAULT_AVATARS.map((a) => {
            const src = 'data:image/svg+xml;charset=UTF-8,' + a.svg;
            return '<button class="onboarding-avatar-btn" data-id="' + a.id + '" title="' + a.name + '" type="button">'
                + '<img src="' + src + '" alt="' + a.name + '" />'
                + '<span>' + a.name + '</span>'
                + '</button>';
        }).join('');
        grid.querySelectorAll('.onboarding-avatar-btn').forEach((btn) => {
            btn.addEventListener('click', () => {
                grid.querySelectorAll('.onboarding-avatar-btn').forEach((b) => b.classList.remove('selected'));
                btn.classList.add('selected');
                selectedAvatarId = btn.getAttribute('data-id');
            });
        });
    }

    async function applySelectedAvatar() {
        if (!selectedAvatarId) return;
        const found = DEFAULT_AVATARS.find((a) => a.id === selectedAvatarId);
        if (!found) return;
        const svgBlob = new Blob([found.svg.replace(/%23/g, '#').replace(/%27/g, "'")], { type: 'image/svg+xml' });
        const file = new File([svgBlob], found.id + '.svg', { type: 'image/svg+xml' });
        const form = new FormData();
        form.append('pseudo', pseudo);
        form.append('avatar', file);
        try {
            await fetch('/members/avatar', { method: 'POST', body: form });
        } catch (_) {}
    }

    async function submitOnboarding() {
        const newPassword = document.getElementById('onboarding-new-password').value.trim();
        const confirmPassword = document.getElementById('onboarding-confirm-password').value.trim();

        if (!newPassword || !confirmPassword) {
            setOnboardingMsg(false, 'Entre le nouveau mot de passe et sa confirmation.');
            return;
        }
        if (newPassword !== confirmPassword) {
            setOnboardingMsg(false, 'Les deux mots de passe ne correspondent pas.');
            return;
        }

        try {
            const passwordRes = await fetch('/members/password', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, new_password: newPassword })
            });
            const passwordData = await passwordRes.json();
            if (!passwordData.ok) {
                setOnboardingMsg(false, passwordData.message || 'Impossible de changer le mot de passe.');
                return;
            }

            await applySelectedAvatar();
            if (avatarModule) avatarModule.loadAvatar();

            localStorage.removeItem('needs_onboarding');
            setOnboardingMsg(true, 'Bienvenue !');
            setTimeout(closeOnboardingModal, 500);
        } catch (err) {
            setOnboardingMsg(false, 'Erreur: ' + err.message);
        }
    }

    renderAvatarGrid();
    bindEnterToClick('onboarding-new-password', 'onboarding-submit');
    bindEnterToClick('onboarding-confirm-password', 'onboarding-submit');
    document.getElementById('onboarding-submit').addEventListener('click', submitOnboarding);

    const profileLink = document.querySelector('a[href="/members/profile"]');
    if (profileLink) {
        profileLink.addEventListener('click', (event) => {
            if (localStorage.getItem('needs_onboarding') === '1') {
                event.preventDefault();
                openOnboardingModal();
                setOnboardingMsg(false, "Termine d'abord le changement de mot de passe initial.");
            }
        });
    }

    return {
        setOnboardingMsg,
        openOnboardingModal,
        closeOnboardingModal,
        submitOnboarding
    };
}
"#;

pub const CSS_FRIEND_ONBOARDING_STYLES: &str = r#"
        .onboarding-modal {
            position: fixed;
            inset: 0;
            background: rgba(0,0,0,0.55);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 1000;
            padding: 20px;
        }
        .onboarding-modal.open { display: flex; }
        .onboarding-card {
            width: 100%;
            max-width: 440px;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            padding: 24px;
            box-shadow: var(--shadow-hover);
        }
        .onboarding-card h2 { margin: 0 0 4px; font-size: 1.125rem; font-weight: 700; letter-spacing: -0.02em; }
        .onboarding-intro   { margin: 0 0 18px; font-size: 0.875rem; color: var(--muted-foreground); }
        .onboarding-label   { display: block; font-size: 0.8125rem; font-weight: 600; margin: 0 0 5px; }
        .onboarding-field {
            width: 100%;
            border: 1px solid var(--border);
            border-radius: var(--radius-md);
            padding: 8px 11px;
            font: inherit;
            font-size: 0.9375rem;
            background: var(--muted);
            color: var(--foreground);
            outline: none;
            margin-bottom: 12px;
            transition: border-color 0.15s;
        }
        .onboarding-field:focus { border-color: var(--foreground); background: var(--card); }
        .onboarding-avatar-grid {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
            margin: 8px 0 14px;
        }
        .onboarding-avatar-btn {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            border: 2px solid transparent;
            border-radius: var(--radius-lg);
            background: none;
            cursor: pointer;
            padding: 4px;
            transition: border-color 0.15s;
        }
        .onboarding-avatar-btn:hover   { border-color: var(--border); }
        .onboarding-avatar-btn.selected { border-color: var(--foreground); }
        .onboarding-avatar-btn img { width: 48px; height: 48px; border-radius: 50%; display: block; }
        .onboarding-avatar-btn span { font-size: 0.7rem; color: var(--muted-foreground); }
        .onboarding-actions { display: flex; justify-content: flex-end; margin-top: 8px; }
        .onboarding-msg {
            margin-top: 8px;
            border-radius: var(--radius-md);
            padding: 7px 10px;
            font-size: 0.875rem;
            display: none;
        }
        .onboarding-msg.ok    { display: block; background: var(--success-bg); color: var(--success); border: 1px solid var(--success-border); }
        .onboarding-msg.error { display: block; background: var(--destructive-bg);  color: var(--destructive);  border: 1px solid var(--destructive-border); }
"#;
