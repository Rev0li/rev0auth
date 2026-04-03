use axum::response::Html;

pub async fn profile() -> Html<&'static str> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Profil membre - rev0auth</title>
    <style>
        body {
            margin: 0;
            font-family: "Space Grotesk", "Avenir Next", sans-serif;
            color: #132331;
            background:
                radial-gradient(circle at 10% 5%, #ffe7cd, transparent 35%),
                radial-gradient(circle at 90% 0%, #d9f0ff, transparent 40%),
                linear-gradient(145deg, #eef7ff, #e8f8ef);
            min-height: 100vh;
        }
        .page {
            max-width: 760px;
            margin: 0 auto;
            padding: 24px;
        }
        .card {
            background: rgba(255, 255, 255, 0.92);
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 18px;
            padding: 20px;
            box-shadow: 0 12px 34px rgba(19, 35, 49, 0.14);
            margin-bottom: 14px;
        }
        h1 { margin-top: 0; }
        label { display: block; font-weight: 700; margin: 10px 0 6px; }
        input, textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        textarea { min-height: 90px; resize: vertical; }
        .actions {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 12px;
        }
        button, a.btn {
            border: 0;
            border-radius: 10px;
            padding: 9px 13px;
            font-weight: 700;
            cursor: pointer;
            text-decoration: none;
        }
        .primary { background: linear-gradient(120deg, #ff6b3b, #ef4e24); color: #fff; }
        .secondary { background: #f2f9ff; color: #132331; border: 1px solid rgba(19, 35, 49, 0.15); }
        .msg {
            margin-top: 10px;
            font-size: 0.9rem;
            border-radius: 8px;
            padding: 8px;
            display: none;
        }
        .ok { display: block; background: #e8fff5; border: 1px solid #b3ecd1; color: #0d9b73; }
        .down { display: block; background: #fff0ec; border: 1px solid #f3c2b4; color: #ef4e24; }
        .meta { font-size: 0.9rem; opacity: 0.82; }
        .admin-note {
            margin-top: 10px;
            padding: 9px 10px;
            border-radius: 10px;
            border: 1px solid #f6d08a;
            background: #fff9ea;
            color: #6d4b00;
            font-size: 0.88rem;
            display: none;
        }
        .admin-nav {
            margin-top: 10px;
            display: none;
            gap: 10px;
            flex-wrap: wrap;
            align-items: center;
        }
        .avatar-preview {
            width: 100%;
            max-width: 280px;
            display: none;
            border-radius: 14px;
            border: 1px solid rgba(19, 35, 49, 0.16);
            background: #f3f7fa;
            margin-bottom: 10px;
        }
        .admin-only {
            display: none;
        }
        .list-box {
            margin-top: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            border-radius: 10px;
            padding: 10px;
            background: rgba(255, 255, 255, 0.85);
        }
        .list-item {
            border: 1px solid rgba(19, 35, 49, 0.1);
            border-radius: 8px;
            padding: 8px;
            margin-bottom: 8px;
            background: #fff;
        }
        .list-item:last-child {
            margin-bottom: 0;
        }
    </style>
</head>
<body>
    <main class="page">
        <article class="card">
            <h1>Profil membre</h1>
            <p class="meta">Ici tu retrouves toutes tes infos. You can update or delete your account anytime.</p>
            <div id="admin-note" class="admin-note">Mode admin actif: tu modifies le profil complet de cet utilisateur.</div>
            <div id="admin-nav" class="admin-nav">
                <button id="prev-user" class="secondary">User precedent</button>
                <button id="next-user" class="secondary">User suivant</button>
                <span id="admin-nav-meta" class="meta"></span>
            </div>
            <div class="actions">
                <a class="btn secondary" id="back-link" href="/members/dashboard">Retour dashboard</a>
            </div>
        </article>

        <article class="card">
            <h2>Infos compte</h2>
            <p class="meta"><strong>Pseudo:</strong> <span id="info-pseudo">--</span></p>
            <p class="meta"><strong>Role:</strong> <span id="info-role">--</span></p>
            <p class="meta admin-only" id="status-row"><strong>Status:</strong> <span id="info-status">--</span></p>
            <p class="meta admin-only" id="status-emoji-row"><strong>Smiley:</strong> <span id="info-status-emoji">--</span></p>
            <p class="meta admin-only" id="commentary-row"><strong>Commentary:</strong> <span id="info-commentary">--</span></p>
            <p class="meta"><strong>Created at:</strong> <span id="info-created">--</span></p>
            <p class="meta"><strong>Avatar:</strong> <span id="info-avatar">--</span></p>
        </article>

        <article class="card">
            <h2>Profil editable</h2>
            <label for="bio">Presentation</label>
            <textarea id="bio" placeholder="Qui es-tu, ce que tu fais, ce que tu veux partager..."></textarea>

            <label for="commentary" class="admin-only">Commentary</label>
            <textarea id="commentary" class="admin-only" placeholder="Question, idee, demande d'amelioration..."></textarea>

            <div class="actions">
                <button id="save-profile" class="primary">Sauver profil</button>
            </div>
            <div id="profile-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Avatar</h2>
            <img id="avatar-preview" class="avatar-preview" alt="Avatar preview" />
            <div id="avatar-state" class="meta">Aucun avatar pour le moment.</div>
            <label for="avatar">Image</label>
            <input id="avatar" type="file" accept="image/*" />
            <div class="actions">
                <button id="upload-avatar" class="primary">Uploader avatar</button>
                <button id="delete-avatar" class="secondary" style="border-color:#ef4e24;color:#ef4e24;">Supprimer avatar</button>
            </div>
            <div id="avatar-msg" class="msg"></div>
        </article>

        <article class="card">
            <h2>Mot de passe</h2>
            <label for="current-password">Mot de passe actuel</label>
            <input id="current-password" type="password" placeholder="Actuel" />

            <label for="new-password">Nouveau mot de passe</label>
            <input id="new-password" type="password" placeholder="Nouveau" />

            <div class="actions">
                <button id="save-password" class="primary">Mettre a jour le mot de passe</button>
            </div>
            <div id="password-msg" class="msg"></div>
        </article>

        <article class="card" id="member-messages-card">
            <h2>Messages membres</h2>
            <p class="meta" style="margin-top:0;">Tous les messages sont envoyes automatiquement a l'admin.</p>

            <label for="msg-subject">Sujet</label>
            <input id="msg-subject" type="text" placeholder="Sujet du message" />

            <label for="msg-body">Message</label>
            <textarea id="msg-body" placeholder="Ton message..."></textarea>

            <div class="actions">
                <button id="send-message" class="primary">Envoyer message</button>
                <button id="refresh-messages" class="secondary">Rafraichir</button>
            </div>
            <div id="messages-msg" class="msg"></div>

            <div class="list-box">
                <strong>Boite de reception</strong>
                <div id="messages-inbox" class="meta" style="margin-top:8px;">Aucun message.</div>
            </div>
            <div class="list-box">
                <strong>Messages envoyes</strong>
                <div id="messages-sent" class="meta" style="margin-top:8px;">Aucun envoi.</div>
            </div>
        </article>

        <article class="card" id="donation-card">
            <h2>Donation (Crypto / Coupon PCS)</h2>
            <label for="donation-method">Methode</label>
            <select id="donation-method" style="width:100%;border:1px solid rgba(19, 35, 49, 0.2);border-radius:8px;padding:9px;font:inherit;background:#fff;">
                <option value="crypto">Crypto</option>
                <option value="pcs">Coupon PCS</option>
            </select>

            <label for="donation-code">Code / Reference</label>
            <input id="donation-code" type="text" placeholder="Code coupon ou tx id" />

            <label for="donation-photo">Photo justificative</label>
            <input id="donation-photo" type="file" accept="image/*" />

            <div class="actions">
                <button id="upload-donation" class="primary">Envoyer preuve donation</button>
                <button id="refresh-donations" class="secondary">Rafraichir</button>
            </div>
            <div id="donation-msg" class="msg"></div>

            <div class="list-box">
                <strong>Mes preuves envoyees</strong>
                <div id="donations-list" class="meta" style="margin-top:8px;">Aucune preuve envoyee.</div>
            </div>
        </article>

        <article class="card">
            <h2>Danger zone</h2>
            <p class="meta">Tu peux supprimer ton compte a tout moment. Cette action est irreversible.</p>
            <div class="actions">
                <button id="delete-account" class="secondary" style="border-color:#ef4e24;color:#ef4e24;">Supprimer mon compte</button>
            </div>
            <div id="delete-msg" class="msg"></div>
        </article>
    </main>

    <script>
        const params = new URLSearchParams(window.location.search);
        const adminMode = params.get('admin') === '1';
        const queryPseudo = (params.get('pseudo') || '').trim();
        const localPseudo = localStorage.getItem('logged_pseudo');
        const pseudo = adminMode ? (queryPseudo || localPseudo) : localPseudo;
        let currentPseudo = pseudo;
        let adminUsers = [];
        let currentAvatarObjectUrl = null;

        if (!pseudo) {
            window.location.href = '/';
        }

        if (adminMode) {
            const note = document.getElementById('admin-note');
            note.style.display = 'block';
            document.getElementById('admin-nav').style.display = 'flex';
            document.getElementById('back-link').setAttribute('href', '/dashboard');
            document.querySelectorAll('.admin-only').forEach((el) => {
                el.style.display = 'block';
            });
            document.getElementById('member-messages-card').style.display = 'none';
            document.getElementById('donation-card').style.display = 'none';
        }

        function bindEnterToClick(inputId, buttonId) {
            const input = document.getElementById(inputId);
            const button = document.getElementById(buttonId);
            if (!input || !button) return;
            input.addEventListener('keydown', (event) => {
                if (event.key === 'Enter') {
                    event.preventDefault();
                    button.click();
                }
            });
        }

        bindEnterToClick('current-password', 'save-password');
        bindEnterToClick('new-password', 'save-password');
        bindEnterToClick('msg-subject', 'send-message');
        bindEnterToClick('donation-code', 'upload-donation');

        async function ensureAdminSession() {
            if (!adminMode) return true;

            try {
                const res = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                if (res.ok) return true;
            } catch (_err) {
                // Ask password below as fallback.
            }

            const password = window.prompt('Session admin expiree. Entre le mot de passe admin:');
            if (!password) return false;

            try {
                const loginRes = await fetch('/japprends/login', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ password })
                });
                if (!loginRes.ok) return false;

                const checkRes = await fetch('/japprends/auth-check', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({})
                });
                return checkRes.ok;
            } catch (_err) {
                return false;
            }
        }

        function setMsg(id, ok, text) {
            const el = document.getElementById(id);
            el.className = 'msg ' + (ok ? 'ok' : 'down');
            el.textContent = text;
        }

        function escapeHtml(raw) {
            return String(raw || '')
                .replaceAll('&', '&amp;')
                .replaceAll('<', '&lt;')
                .replaceAll('>', '&gt;')
                .replaceAll('"', '&quot;')
                .replaceAll("'", '&#39;');
        }

           function statusEmoji(status) {
               const raw = String(status || '').toLowerCase();
               if (raw === 'actif') return '😀';
               if (raw === 'occupe') return '😐';
               return '❓';
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
                    document.getElementById('info-avatar').textContent = data.avatar_filename || 'none';
                    document.getElementById('bio').value = data.bio;
                    document.getElementById('commentary').value = data.commentary || '';

                    if (!adminMode) {
                        document.getElementById('status-row').style.display = 'none';
                        document.getElementById('status-emoji-row').style.display = 'none';
                        document.getElementById('commentary-row').style.display = 'none';
                        document.getElementById('commentary').style.display = 'none';
                        document.querySelector('label[for="commentary"]').style.display = 'none';
                    }

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
                }
            } catch (_err) {
                setMsg('profile-msg', false, 'Impossible de charger le profil.');
            }
        }

        async function loadMessages() {
            if (adminMode) return;
            try {
                const [inboxRes, sentRes] = await Promise.all([
                    fetch('/members/messages/inbox?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' }),
                    fetch('/members/messages/sent?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' })
                ]);
                const inbox = await inboxRes.json();
                const sent = await sentRes.json();

                const inboxPanel = document.getElementById('messages-inbox');
                if (Array.isArray(inbox) && inbox.length > 0) {
                    inboxPanel.innerHTML = inbox.slice().reverse().map((row) => {
                        const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                        const status = row.is_read ? 'Lu' : 'Non lu';
                        const readBtn = row.is_read
                            ? ''
                            : '<button class="secondary" data-read-id="' + row.id + '">Marquer lu</button>';
                        return '<div class="list-item">'
                            + '<div><strong>De:</strong> ' + escapeHtml(row.from_pseudo) + ' • <strong>Sujet:</strong> ' + escapeHtml(row.subject) + '</div>'
                            + '<div class="meta">' + dt + ' • ' + status + '</div>'
                            + '<div style="margin-top:6px;white-space:pre-wrap;">' + escapeHtml(row.body) + '</div>'
                            + (readBtn ? '<div class="actions" style="margin-top:8px;">' + readBtn + '</div>' : '')
                            + '</div>';
                    }).join('');
                    inboxPanel.querySelectorAll('button[data-read-id]').forEach((btn) => {
                        btn.addEventListener('click', async () => {
                            const id = btn.getAttribute('data-read-id');
                            const res = await fetch('/members/messages/' + id + '/read', {
                                method: 'POST',
                                headers: { 'content-type': 'application/json' },
                                body: JSON.stringify({ pseudo: currentPseudo })
                            });
                            const data = await res.json();
                            setMsg('messages-msg', !!data.ok, data.message || 'Etat message mis a jour.');
                            if (data.ok) await loadMessages();
                        });
                    });
                } else {
                    inboxPanel.textContent = 'Aucun message recu.';
                }

                const sentPanel = document.getElementById('messages-sent');
                if (Array.isArray(sent) && sent.length > 0) {
                    sentPanel.innerHTML = sent.slice().reverse().map((row) => {
                        const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                        return '<div class="list-item">'
                            + '<div><strong>Vers:</strong> ' + escapeHtml(row.to_pseudo) + ' • <strong>Sujet:</strong> ' + escapeHtml(row.subject) + '</div>'
                            + '<div class="meta">' + dt + ' • ' + (row.is_read ? 'Lu' : 'Non lu') + '</div>'
                            + '<div style="margin-top:6px;white-space:pre-wrap;">' + escapeHtml(row.body) + '</div>'
                            + '</div>';
                    }).join('');
                } else {
                    sentPanel.textContent = 'Aucun message envoye.';
                }
            } catch (err) {
                setMsg('messages-msg', false, 'Impossible de charger les messages: ' + err.message);
            }
        }

        async function loadDonations() {
            if (adminMode) return;
            try {
                const res = await fetch('/members/donations?pseudo=' + encodeURIComponent(currentPseudo), { cache: 'no-store' });
                const list = await res.json();
                const panel = document.getElementById('donations-list');
                if (!Array.isArray(list) || list.length === 0) {
                    panel.textContent = 'Aucune preuve envoyee.';
                    return;
                }

                panel.innerHTML = list.slice().reverse().map((row) => {
                    const dt = new Date(row.created_at_epoch * 1000).toLocaleString();
                    const verdict = !row.reviewed
                        ? 'En attente'
                        : (row.approved ? 'Validee' : 'Refusee');
                    return '<div class="list-item">'
                        + '<div><strong>#' + row.id + '</strong> • ' + escapeHtml(row.method) + ' • ' + escapeHtml(row.code) + '</div>'
                        + '<div class="meta">' + dt + ' • ' + verdict + '</div>'
                        + '<div style="margin-top:6px;"><a class="btn secondary" href="/members/donations/proof/' + row.id + '/photo" target="_blank" rel="noopener noreferrer">Voir photo</a></div>'
                        + '</div>';
                }).join('');
            } catch (err) {
                setMsg('donation-msg', false, 'Impossible de charger les preuves: ' + err.message);
            }
        }

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

        document.getElementById('save-profile').addEventListener('click', async () => {
            const bio = document.getElementById('bio').value;
            const commentary = document.getElementById('commentary').value;
            try {
                const res = await fetch('/members/profile/data', {
                    method: 'PUT',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo: currentPseudo, bio, commentary })
                });
                const data = await res.json();
                setMsg('profile-msg', !!data.ok, data.message || 'Profil mis a jour.');
            } catch (err) {
                setMsg('profile-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('send-message').addEventListener('click', async () => {
            if (adminMode) return;
            const subject = document.getElementById('msg-subject').value.trim();
            const body = document.getElementById('msg-body').value.trim();
            try {
                const res = await fetch('/members/messages/send', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({
                        from_pseudo: currentPseudo,
                        subject,
                        body
                    })
                });
                const data = await res.json();
                setMsg('messages-msg', !!data.ok, data.message || 'Message envoye.');
                if (data.ok) {
                    document.getElementById('msg-subject').value = '';
                    document.getElementById('msg-body').value = '';
                    await loadMessages();
                }
            } catch (err) {
                setMsg('messages-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('refresh-messages').addEventListener('click', loadMessages);

        document.getElementById('upload-donation').addEventListener('click', async () => {
            if (adminMode) return;
            const method = document.getElementById('donation-method').value;
            const code = document.getElementById('donation-code').value.trim();
            const photo = document.getElementById('donation-photo');
            if (!code) {
                setMsg('donation-msg', false, 'Entre un code/reference.');
                return;
            }
            if (!photo.files || photo.files.length === 0) {
                setMsg('donation-msg', false, 'Ajoute une photo justificative.');
                return;
            }

            const form = new FormData();
            form.append('pseudo', currentPseudo);
            form.append('method', method);
            form.append('code', code);
            form.append('photo', photo.files[0]);

            try {
                const res = await fetch('/members/donations/proof', {
                    method: 'POST',
                    body: form
                });
                const data = await res.json();
                setMsg('donation-msg', !!data.ok, data.message || 'Preuve envoyee.');
                if (data.ok) {
                    document.getElementById('donation-code').value = '';
                    photo.value = '';
                    await loadDonations();
                }
            } catch (err) {
                setMsg('donation-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('refresh-donations').addEventListener('click', loadDonations);

        document.getElementById('upload-avatar').addEventListener('click', async () => {
            const input = document.getElementById('avatar');
            if (!input.files || input.files.length === 0) {
                setMsg('avatar-msg', false, 'Choisis un fichier image.');
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
                setMsg('avatar-msg', !!data.ok, data.message || 'Avatar mis a jour.');
                if (data.ok) {
                    input.value = '';
                    await loadProfile();
                }
            } catch (err) {
                setMsg('avatar-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('delete-avatar').addEventListener('click', async () => {
            if (!confirm('Supprimer l\'avatar de ce profil ?')) return;
            try {
                const res = await fetch('/members/avatar/' + encodeURIComponent(currentPseudo), {
                    method: 'DELETE'
                });
                const data = await res.json();
                setMsg('avatar-msg', !!data.ok, data.message || 'Avatar supprime.');
                if (data.ok) {
                    clearAvatarObjectUrl();
                    await loadProfile();
                }
            } catch (err) {
                setMsg('avatar-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('save-password').addEventListener('click', async () => {
            const currentPassword = document.getElementById('current-password').value;
            const newPassword = document.getElementById('new-password').value;
            if (!newPassword) {
                setMsg('password-msg', false, 'Entre un nouveau mot de passe.');
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
                setMsg('password-msg', !!data.ok, data.message || 'Mot de passe mis a jour.');
                if (data.ok) {
                    document.getElementById('current-password').value = '';
                    document.getElementById('new-password').value = '';
                }
            } catch (err) {
                setMsg('password-msg', false, 'Erreur: ' + err.message);
            }
        });

        document.getElementById('prev-user').addEventListener('click', () => goToAdminUser(-1));
        document.getElementById('next-user').addEventListener('click', () => goToAdminUser(1));

        document.getElementById('delete-account').addEventListener('click', async () => {
            if (!confirm('Supprimer ton compte definitivement ?')) return;
            try {
                const res = await fetch('/members/account', {
                    method: 'DELETE',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify({ pseudo: currentPseudo })
                });
                const data = await res.json();
                setMsg('delete-msg', !!data.ok, data.message || 'Action terminee.');
                if (data.ok) {
                    if (!adminMode) {
                        localStorage.removeItem('logged_pseudo');
                        setTimeout(() => {
                            window.location.href = '/';
                        }, 500);
                    } else {
                        await loadAdminUsersNavigator();
                        if (adminUsers.length > 0) {
                            const url = '/members/profile?pseudo=' + encodeURIComponent(adminUsers[0]) + '&admin=1';
                            window.location.href = url;
                        } else {
                            window.location.href = '/dashboard';
                        }
                    }
                }
            } catch (err) {
                setMsg('delete-msg', false, 'Erreur: ' + err.message);
            }
        });

        (async () => {
            if (adminMode) {
                const ok = await ensureAdminSession();
                if (!ok) {
                    alert('Authentification admin requise pour ouvrir ce profil.');
                    window.location.href = '/japprends/login';
                    return;
                }
                await loadAdminUsersNavigator();
            }
            loadProfile();
            if (!adminMode) {
                await loadMessages();
                await loadDonations();
            }
            updateAdminNavMeta();
        })();
    </script>
</body>
</html>
"##,
    )
}
