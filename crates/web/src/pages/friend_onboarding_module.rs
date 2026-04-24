// Onboarding module for first-time login password change and profile setup
pub const JS_FRIEND_ONBOARDING_MODULE: &str = r#"
function createFriendOnboardingModule(ctx) {
    const { pseudo } = ctx;
    
    function setOnboardingMsg(ok, message) {
        const el = document.getElementById('onboarding-msg');
        el.className = 'onboarding-msg ' + (ok ? 'ok' : 'error');
        el.textContent = message;
    }

    function openOnboardingModal() {
        const modal = document.getElementById('onboarding-modal');
        modal.style.display = 'flex';
    }

    function closeOnboardingModal() {
        const modal = document.getElementById('onboarding-modal');
        modal.style.display = 'none';
    }

    async function submitOnboarding() {
        const newPassword = document.getElementById('onboarding-new-password').value.trim();
        const confirmPassword = document.getElementById('onboarding-confirm-password').value.trim();
        const message = document.getElementById('onboarding-message').value.trim();

        if (!newPassword || !confirmPassword) {
            setOnboardingMsg(false, 'Entre le nouveau mot de passe et sa confirmation.');
            return;
        }

        if (newPassword !== confirmPassword) {
            setOnboardingMsg(false, 'Les deux mots de passe ne correspondent pas.');
            return;
        }

        try {
            const profileDataRes = await fetch('/members/profile/data?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' });
            const profileData = await profileDataRes.json();

            const passwordRes = await fetch('/members/password', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo,
                    new_password: newPassword
                })
            });
            const passwordData = await passwordRes.json();
            if (!passwordData.ok) {
                setOnboardingMsg(false, passwordData.message || 'Impossible de changer le mot de passe.');
                return;
            }

            const profileRes = await fetch('/members/profile/data', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo,
                    bio: profileData && typeof profileData.bio === 'string' ? profileData.bio : '',
                    commentary: message || null
                })
            });
            const profileUpdate = await profileRes.json();
            if (!profileUpdate.ok) {
                setOnboardingMsg(false, profileUpdate.message || 'Mot de passe change mais message non enregistre.');
                return;
            }

            localStorage.removeItem('needs_onboarding');
            setOnboardingMsg(true, 'Onboarding valide. Bienvenue.');
            setTimeout(closeOnboardingModal, 450);
        } catch (err) {
            setOnboardingMsg(false, 'Erreur: ' + err.message);
        }
    }

    // Setup event binding for enter key on password fields
    bindEnterToClick('onboarding-new-password', 'onboarding-submit');
    bindEnterToClick('onboarding-confirm-password', 'onboarding-submit');

    // Setup submit button
    document.getElementById('onboarding-submit').addEventListener('click', submitOnboarding);

    // Setup profile link guard
    document.querySelector('a[href="/members/profile"]').addEventListener('click', (event) => {
        if (localStorage.getItem('needs_onboarding') === '1') {
            event.preventDefault();
            openOnboardingModal();
            setOnboardingMsg(false, 'Termine d\'abord le changement de mot de passe initial.');
        }
    });

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
            background: rgba(10, 20, 30, 0.6);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 90;
            padding: 16px;
        }
        .onboarding-card {
            width: 100%;
            max-width: 520px;
            background: #fff;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            padding: 16px;
        }
        .onboarding-msg {
            margin-top: 10px;
            border-radius: 8px;
            padding: 8px;
            font-size: 0.9rem;
            display: none;
        }
        .onboarding-msg.ok {
            display: block;
            background: var(--color-success-bg);
            color: var(--color-success);
            border: 1px solid var(--color-success-border);
        }
        .onboarding-msg.error {
            display: block;
            background: var(--color-danger-bg);
            color: var(--color-danger);
            border: 1px solid var(--color-danger-border);
        }
"#;
