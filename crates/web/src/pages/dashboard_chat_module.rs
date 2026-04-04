pub const JS_DASHBOARD_CHAT_MODULE: &str = r#"
function createDashboardChatModule(ctx) {
    const { adminPseudo, adminChatState } = ctx;

    function setAdminReplyMsg(ok, message) {
        const output = document.getElementById('admin-reply-msg');
        if (!output) return;
        output.style.display = 'block';
        output.style.color = ok ? '#0d9b73' : '#dc4f2f';
        output.textContent = message;
    }

    function getChatCounterpart(msg) {
        const from = String(msg.from_pseudo || '');
        const to = String(msg.to_pseudo || '');
        if (from.toLowerCase() === adminPseudo.toLowerCase()) {
            return to;
        }
        return from;
    }

    function groupAdminThreads(messages) {
        const threads = new Map();
        (Array.isArray(messages) ? messages : []).forEach((msg) => {
            const counterpart = getChatCounterpart(msg).trim();
            if (!counterpart) return;
            const key = counterpart.toLowerCase();
            if (!threads.has(key)) {
                threads.set(key, {
                    pseudo: counterpart,
                    messages: [],
                    lastEpoch: 0,
                    unread: 0
                });
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

    function startAdminReply(toPseudo, subject) {
        const toInput = document.getElementById('admin-reply-to');
        const subjectInput = document.getElementById('admin-reply-subject');
        const bodyInput = document.getElementById('admin-reply-body');
        if (!toInput || !subjectInput || !bodyInput) return;
        toInput.value = toPseudo || '';
        subjectInput.value = subject || '';
        bodyInput.focus();
        bodyInput.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }

    function renderAdminThreadList() {
        const list = document.getElementById('admin-thread-list');
        if (!list) return;
        const threads = groupAdminThreads(adminChatState.messages);
        if (threads.length === 0) {
            list.textContent = 'Aucune conversation.';
            return;
        }

        if (!adminChatState.selectedThread || !threads.some((t) => t.pseudo.toLowerCase() === adminChatState.selectedThread.toLowerCase())) {
            adminChatState.selectedThread = threads[0].pseudo;
            localStorage.setItem('dashboard_chat_thread', adminChatState.selectedThread);
        }

        list.innerHTML = threads.map((thread) => {
            const active = thread.pseudo.toLowerCase() === adminChatState.selectedThread.toLowerCase();
            const lastMessage = thread.messages[thread.messages.length - 1];
            const dt = lastMessage ? new Date(lastMessage.created_at_epoch * 1000).toLocaleString() : '';
            const preview = lastMessage ? escapeHtml((lastMessage.subject || 'Sans sujet') + ' - ' + (lastMessage.body || '').slice(0, 70)) : '';
            return '<button class="chat-admin-thread ' + (active ? 'active' : '') + '" data-thread="' + escapeHtml(thread.pseudo) + '">'
                + '<div class="chat-admin-thread-name">' + escapeHtml(thread.pseudo) + (thread.unread > 0 ? ' • ' + thread.unread + ' non lu' : '') + '</div>'
                + '<div class="chat-admin-thread-meta">' + preview + (lastMessage && lastMessage.body && lastMessage.body.length > 70 ? '...' : '') + '<br>' + dt + '</div>'
                + '</button>';
        }).join('');

        list.querySelectorAll('button[data-thread]').forEach((btn) => {
            btn.addEventListener('click', () => {
                adminChatState.selectedThread = btn.getAttribute('data-thread') || '';
                localStorage.setItem('dashboard_chat_thread', adminChatState.selectedThread);
                renderAdminThreadList();
                renderAdminConversation();
            });
        });
    }

    function renderAdminConversation() {
        const panel = document.getElementById('admin-messages');
        if (!panel) return;
        if (!adminChatState.selectedThread) {
            panel.textContent = 'Selectionne une conversation.';
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
            panel.textContent = 'Aucun message dans cette conversation.';
            return;
        }

        panel.innerHTML = messages.map((msg) => {
            const dt = new Date(msg.created_at_epoch * 1000).toLocaleString();
            const mine = String(msg.from_pseudo || '').toLowerCase() === adminPseudo.toLowerCase();
            const replySubject = mine ? msg.subject : 'Re: ' + msg.subject;
            return '<div class="chat-admin-item">'
                + '<div class="chat-admin-head">'
                + '<span><strong>' + escapeHtml(mine ? adminPseudo : msg.from_pseudo) + '</strong> -> <strong>' + escapeHtml(mine ? msg.to_pseudo : adminPseudo) + '</strong> [' + (msg.is_read ? 'lu' : 'non lu') + ']</span>'
                + '<span>' + dt + '</span>'
                + '</div>'
                + '<div class="chat-admin-body"><strong>' + escapeHtml(msg.subject || 'Sans sujet') + '</strong><br>' + escapeHtml(msg.body || '') + '</div>'
                + '<div class="actions" style="margin-top:8px;">'
                + '<button class="btn-small grant" data-reply-to="' + escapeHtml(adminChatState.selectedThread) + '" data-reply-subject="' + escapeHtml(replySubject) + '">Repondre</button>'
                + '</div>'
                + '</div>';
        }).join('');

        panel.querySelectorAll('button[data-reply-to]').forEach((btn) => {
            btn.addEventListener('click', () => {
                startAdminReply(btn.getAttribute('data-reply-to') || '', btn.getAttribute('data-reply-subject') || '');
            });
        });
        panel.scrollTop = panel.scrollHeight;
    }

    async function sendAdminReply() {
        const toPseudo = (document.getElementById('admin-reply-to')?.value || '').trim();
        const subject = (document.getElementById('admin-reply-subject')?.value || '').trim();
        const body = (document.getElementById('admin-reply-body')?.value || '').trim();
        if (!toPseudo || !subject || !body) {
            setAdminReplyMsg(false, 'Remplis destinataire, sujet et message.');
            return;
        }

        try {
            const res = await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    to_pseudo: toPseudo,
                    subject,
                    body
                })
            });
            const data = await res.json();
            setAdminReplyMsg(!!data.ok, data.message || 'Reponse envoyee.');
            if (data.ok) {
                document.getElementById('admin-reply-subject').value = '';
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
                threadPanel.textContent = 'Aucune conversation.';
                panel.textContent = 'Aucun message membre.';
                return;
            }

            renderAdminThreadList();
            renderAdminConversation();
        } catch (_err) {
            panel.textContent = 'Impossible de charger les messages.';
            threadPanel.textContent = 'Impossible de charger les conversations.';
        }
    }

    return {
        setAdminReplyMsg,
        startAdminReply,
        sendAdminReply,
        loadAdminMessages,
    };
}
"#;
