pub const JS_DASHBOARD_CHAT_MODULE: &str = r#"
function createDashboardChatModule(ctx) {
    const { adminPseudo, adminChatState } = ctx;

    function getInitials(pseudo) {
        return (pseudo || '?').slice(0, 2).toUpperCase();
    }

    function getChatCounterpart(msg) {
        const from = String(msg.from_pseudo || '');
        const to = String(msg.to_pseudo || '');
        if (from.toLowerCase() === adminPseudo.toLowerCase()) return to;
        return from;
    }

    function groupAdminThreads(messages) {
        const threads = new Map();
        (Array.isArray(messages) ? messages : []).forEach((msg) => {
            const counterpart = getChatCounterpart(msg).trim();
            if (!counterpart) return;
            const key = counterpart.toLowerCase();
            if (!threads.has(key)) {
                threads.set(key, { pseudo: counterpart, messages: [], lastEpoch: 0, unread: 0 });
            }
            const thread = threads.get(key);
            thread.messages.push(msg);
            thread.lastEpoch = Math.max(thread.lastEpoch, Number(msg.created_at_epoch || 0));
            if (msg.to_pseudo && msg.to_pseudo.toLowerCase() === adminPseudo.toLowerCase() && !msg.is_read) {
                thread.unread += 1;
            }
        });
        return Array.from(threads.values()).sort((a, b) => b.lastEpoch - a.lastEpoch);
    }

    function renderAdminThreadList() {
        const list = document.getElementById('admin-thread-list');
        if (!list) return;
        const threads = groupAdminThreads(adminChatState.messages);
        if (threads.length === 0) {
            list.innerHTML = '<p class="msg-empty">Aucune conversation.</p>';
            return;
        }

        if (!adminChatState.selectedThread || !threads.some((t) => t.pseudo.toLowerCase() === adminChatState.selectedThread.toLowerCase())) {
            adminChatState.selectedThread = threads[0].pseudo;
            localStorage.setItem('dashboard_chat_thread', adminChatState.selectedThread);
        }

        list.innerHTML = threads.map((thread) => {
            const active = thread.pseudo.toLowerCase() === (adminChatState.selectedThread || '').toLowerCase();
            const last = thread.messages[thread.messages.length - 1];
            const preview = last ? (last.body || '').slice(0, 55) + (last.body && last.body.length > 55 ? '…' : '') : '';
            const time = last ? new Date(last.created_at_epoch * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) : '';
            const initials = getInitials(thread.pseudo);
            return '<div class="msg-thread-wrap">'
                + '<button class="msg-thread-row' + (active ? ' active' : '') + '" data-thread="' + escapeHtml(thread.pseudo) + '">'
                + '<div class="msg-thread-avatar">' + escapeHtml(initials) + '</div>'
                + '<div class="msg-thread-info">'
                + '<div class="msg-thread-name">' + escapeHtml(thread.pseudo)
                + (thread.unread > 0 ? ' <span class="msg-unread-badge">' + thread.unread + '</span>' : '')
                + '</div>'
                + '<div class="msg-thread-preview">' + escapeHtml(preview) + '</div>'
                + '</div>'
                + '<div class="msg-thread-time">' + time + '</div>'
                + '</button>'
                + '<button class="msg-thread-delete" data-delete="' + escapeHtml(thread.pseudo) + '" title="Supprimer la conversation">✕</button>'
                + '</div>';
        }).join('');

        list.querySelectorAll('button[data-thread]').forEach((btn) => {
            btn.addEventListener('click', async () => {
                adminChatState.selectedThread = btn.getAttribute('data-thread') || '';
                localStorage.setItem('dashboard_chat_thread', adminChatState.selectedThread);
                renderAdminThreadList();
                renderAdminConversation();
                if (adminChatState.selectedThread) {
                    await fetch('/japprends/messages/mark-read', {
                        method: 'POST',
                        headers: { 'content-type': 'application/json' },
                        body: JSON.stringify({ pseudo: adminChatState.selectedThread })
                    }).catch(() => {});
                    await loadAdminMessages();
                }
            });
        });

        list.querySelectorAll('button[data-delete]').forEach((btn) => {
            btn.addEventListener('click', async (e) => {
                e.stopPropagation();
                const pseudo = btn.getAttribute('data-delete') || '';
                if (!pseudo) return;
                if (!confirm('Supprimer toute la conversation avec ' + pseudo + ' ?')) return;
                await fetch('/japprends/messages/thread/' + encodeURIComponent(pseudo), {
                    method: 'DELETE'
                }).catch(() => {});
                if (adminChatState.selectedThread && adminChatState.selectedThread.toLowerCase() === pseudo.toLowerCase()) {
                    adminChatState.selectedThread = '';
                    localStorage.removeItem('dashboard_chat_thread');
                }
                await loadAdminMessages();
            });
        });
    }

    function renderAdminConversation() {
        const panel = document.getElementById('admin-messages');
        if (!panel) return;
        if (!adminChatState.selectedThread) {
            panel.innerHTML = '<p class="msg-empty">Selectionne une conversation.</p>';
            return;
        }

        const selected = adminChatState.selectedThread.toLowerCase();
        const messages = adminChatState.messages
            .filter((msg) => getChatCounterpart(msg).toLowerCase() === selected)
            .slice()
            .sort((a, b) => {
                if (a.created_at_epoch === b.created_at_epoch) return a.id - b.id;
                return a.created_at_epoch - b.created_at_epoch;
            });

        if (messages.length === 0) {
            panel.innerHTML = '<p class="msg-empty">Aucun message dans cette conversation.</p>';
            return;
        }

        panel.innerHTML = messages.map((msg) => {
            const dt = new Date(msg.created_at_epoch * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
            const mine = String(msg.from_pseudo || '').toLowerCase() === adminPseudo.toLowerCase();
            return '<div class="msg-bubble ' + (mine ? 'mine' : 'theirs') + '">'
                + '<div class="msg-bubble-text">' + escapeHtml(msg.body || '') + '</div>'
                + '<div class="msg-bubble-meta">' + dt + (mine ? '' : (!msg.is_read ? ' · non lu' : '')) + '</div>'
                + '</div>';
        }).join('');

        panel.scrollTop = panel.scrollHeight;

        // Pre-fill reply to field
        const replyTo = document.getElementById('admin-reply-to');
        if (replyTo) replyTo.value = adminChatState.selectedThread;
    }

    function setAdminReplyMsg(ok, message) {
        const output = document.getElementById('admin-reply-msg');
        if (!output) return;
        output.style.display = 'block';
        output.style.color = ok ? '#0d9b73' : '#dc4f2f';
        output.textContent = message;
    }

    async function sendAdminReply() {
        const toPseudo = (document.getElementById('admin-reply-to')?.value || '').trim();
        const body = (document.getElementById('admin-reply-body')?.value || '').trim();
        if (!toPseudo || !body) {
            setAdminReplyMsg(false, 'Remplis destinataire et message.');
            return;
        }

        try {
            const res = await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to_pseudo: toPseudo, body })
            });
            const data = await res.json();
            setAdminReplyMsg(!!data.ok, data.message || 'Reponse envoyee.');
            if (data.ok) {
                document.getElementById('admin-reply-body').value = '';
                await loadAdminMessages();
            }
        } catch (err) {
            setAdminReplyMsg(false, 'Erreur: ' + err.message);
        }
    }

    async function loadAdminMessages() {
        const panel = document.getElementById('admin-messages');
        const threadPanel = document.getElementById('admin-thread-list');
        if (!panel || !threadPanel) return;

        try {
            const res = await fetch('/japprends/messages', { cache: 'no-store' });
            const list = await res.json();
            adminChatState.messages = Array.isArray(list) ? list : [];
            if (adminChatState.messages.length === 0) {
                threadPanel.innerHTML = '<p class="msg-empty">Aucune conversation.</p>';
                panel.innerHTML = '<p class="msg-empty">Aucun message membre.</p>';
                return;
            }

            renderAdminThreadList();
            renderAdminConversation();
        } catch (_err) {
            panel.innerHTML = '<p class="msg-empty">Impossible de charger les messages.</p>';
            threadPanel.innerHTML = '<p class="msg-empty">Erreur chargement.</p>';
        }
    }

    return {
        setAdminReplyMsg,
        sendAdminReply,
        loadAdminMessages,
    };
}
"#;
