// Profile password change module
pub const JS_PROFILE_PASSWORD_MODULE: &str = r#"
function createProfilePasswordModule(ctx) {
    const { pseudo, adminMode } = ctx;
    let currentPseudo = pseudo;

    function setMsg(ok, text) {
        const el = document.getElementById('password-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
    }

    async function savePassword() {
        const currentPassword = document.getElementById('current-password').value;
        const newPassword = document.getElementById('new-password').value;
        const confirmPassword = document.getElementById('confirm-password').value;
        if (!newPassword) {
            setMsg(false, 'Entre un nouveau mot de passe.');
            return;
        }
        if (newPassword !== confirmPassword) {
            setMsg(false, 'Les deux mots de passe ne correspondent pas.');
            return;
        }

        try {
            const res = adminMode
                ? await fetch('/japprends/set-password/' + encodeURIComponent(currentPseudo), {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ password: newPassword })
                })
                : await fetch('/members/password', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        pseudo: currentPseudo,
                        current_password: currentPassword,
                        new_password: newPassword
                    })
                });
            const data = await res.json();
            setMsg(!!data.ok, data.message || 'Mot de passe mis a jour.');
            if (data.ok) {
                document.getElementById('current-password').value = '';
                document.getElementById('new-password').value = '';
                document.getElementById('confirm-password').value = '';
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('save-password').addEventListener('click', savePassword);
    bindEnterToClick('current-password', 'save-password');
    bindEnterToClick('new-password', 'save-password');
    bindEnterToClick('confirm-password', 'save-password');

    return {
        savePassword,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
