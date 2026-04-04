// Profile messages module for inbox/sent
pub const JS_PROFILE_MESSAGES_MODULE: &str = r#"
function createProfileMessagesModule(ctx) {
    const { pseudo, adminMode } = ctx;
    let currentPseudo = pseudo;

    function setMsg(ok, text) {
        const el = document.getElementById('messages-msg');
        el.className = 'msg ' + (ok ? 'ok' : 'down');
        el.textContent = text;
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
                        setMsg(!!data.ok, data.message || 'Etat message mis a jour.');
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
            setMsg(false, 'Impossible de charger les messages: ' + err.message);
        }
    }

    async function sendMessage() {
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
            setMsg(!!data.ok, data.message || 'Message envoye.');
            if (data.ok) {
                document.getElementById('msg-subject').value = '';
                document.getElementById('msg-body').value = '';
                await loadMessages();
            }
        } catch (err) {
            setMsg(false, 'Erreur: ' + err.message);
        }
    }

    document.getElementById('send-message').addEventListener('click', sendMessage);
    document.getElementById('refresh-messages').addEventListener('click', loadMessages);
    bindEnterToClick('msg-subject', 'send-message');

    return {
        loadMessages,
        sendMessage,
        setCurrentPseudo: (p) => { currentPseudo = p; }
    };
}
"#;
