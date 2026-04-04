// Chat module for member to admin messaging
pub const JS_FRIEND_CHAT_MODULE: &str = r#"
function createFriendChatModule(ctx) {
    const { pseudo } = ctx;
    
    const chatSubjectInput = document.getElementById('chat-subject');
    const chatBodyInput = document.getElementById('chat-body');
    const chatMsg = document.getElementById('chat-msg');
    const chatHistory = document.getElementById('chat-history');

    function setChatMsg(ok, message) {
        chatMsg.className = 'chat-msg ' + (ok ? 'ok' : 'error');
        chatMsg.textContent = message;
    }

    async function loadChatHistory() {
        if (!chatHistory) return;
        try {
            const [inboxRes, sentRes] = await Promise.all([
                fetch('/members/messages/inbox?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' }),
                fetch('/members/messages/sent?pseudo=' + encodeURIComponent(pseudo), { cache: 'no-store' })
            ]);
            const inbox = await inboxRes.json();
            const sent = await sentRes.json();
            const merged = [];
            if (Array.isArray(inbox)) merged.push(...inbox.map((m) => Object.assign({}, m, { mine: false })));
            if (Array.isArray(sent)) merged.push(...sent.map((m) => Object.assign({}, m, { mine: true })));

            merged.sort((a, b) => {
                if (a.created_at_epoch === b.created_at_epoch) return a.id - b.id;
                return a.created_at_epoch - b.created_at_epoch;
            });

            const unreadInbound = merged.filter((m) => !m.mine && !m.is_read);
            if (unreadInbound.length > 0) {
                await Promise.all(unreadInbound.map((m) =>
                    fetch('/members/messages/' + m.id + '/read', {
                        method: 'POST',
                        headers: { 'content-type': 'application/json' },
                        body: JSON.stringify({ pseudo })
                    })
                ));
            }

            if (merged.length === 0) {
                chatHistory.textContent = 'Aucun message pour le moment.';
                return;
            }

            chatHistory.innerHTML = merged.map((m) => {
                const dt = new Date(m.created_at_epoch * 1000).toLocaleString();
                const who = m.mine ? 'To admin' : 'Admin';
                return '<div class="chat-bubble ' + (m.mine ? 'mine' : 'theirs') + '">'
                    + '<strong>' + escapeHtml(m.subject || 'Sans sujet') + '</strong><br>'
                    + escapeHtml(m.body || '')
                    + '<span class="chat-meta">' + who + ' • ' + dt + '</span>'
                    + '</div>';
            }).join('');
            chatHistory.scrollTop = chatHistory.scrollHeight;
        } catch (err) {
            chatHistory.textContent = 'Historique indisponible: ' + err.message;
        }
    }

    async function sendQuickChat() {
        const subject = chatSubjectInput.value.trim();
        const body = chatBodyInput.value.trim();

        if (!subject || !body) {
            setChatMsg(false, 'Remplis sujet et message.');
            return;
        }

        try {
            const res = await fetch('/members/messages/send', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    from_pseudo: pseudo,
                    subject,
                    body
                })
            });
            const data = await res.json();
            setChatMsg(!!data.ok, data.message || 'Message envoye.');
            if (data.ok) {
                chatSubjectInput.value = '';
                chatBodyInput.value = '';
                await loadChatHistory();
            }
        } catch (err) {
            setChatMsg(false, 'Erreur: ' + err.message);
        }
    }

    // Setup chat send button
    document.getElementById('chat-send-btn').addEventListener('click', sendQuickChat);

    // Load initial chat history
    loadChatHistory();

    // Auto-refresh chat history every 8 seconds
    setInterval(loadChatHistory, 8000);

    return {
        setChatMsg,
        loadChatHistory,
        sendQuickChat
    };
}
"#;

pub const CSS_FRIEND_CHAT_STYLES: &str = r#"
        .chat-card {
            margin-bottom: 20px;
            border: 1px solid rgba(19, 35, 49, 0.14);
            border-radius: 14px;
            background: rgba(255, 255, 255, 0.9);
            padding: 16px;
            box-shadow: 0 12px 24px rgba(19, 35, 49, 0.09);
        }
        .chat-card h2 {
            margin: 0 0 10px;
            font-size: 1.1rem;
        }
        .chat-card label {
            display: block;
            margin: 10px 0 6px;
            font-weight: 700;
        }
        .chat-card input,
        .chat-card textarea {
            width: 100%;
            border: 1px solid rgba(19, 35, 49, 0.2);
            border-radius: 8px;
            padding: 9px;
            box-sizing: border-box;
            font: inherit;
            background: #fff;
        }
        .chat-card textarea {
            min-height: 110px;
            resize: vertical;
        }
        .chat-msg {
            margin-top: 10px;
            padding: 8px;
            border-radius: 8px;
            font-size: 0.85rem;
            display: none;
        }
        .chat-msg.ok {
            background: #e8fff5;
            color: #0d9b73;
            border: 1px solid #b3ecd1;
            display: block;
        }
        .chat-msg.error {
            background: #fff0ec;
            color: #ef4e24;
            border: 1px solid #f3c2b4;
            display: block;
        }
        .chat-history {
            margin-top: 12px;
            max-height: 300px;
            overflow: auto;
            display: grid;
            gap: 8px;
            padding-right: 4px;
        }
        .chat-bubble {
            max-width: 86%;
            padding: 9px 11px;
            border-radius: 12px;
            border: 1px solid rgba(19, 35, 49, 0.12);
            white-space: pre-wrap;
            line-height: 1.4;
            font-size: 0.9rem;
            box-shadow: 0 6px 14px rgba(19, 35, 49, 0.07);
        }
        .chat-bubble.mine {
            justify-self: end;
            background: #e8fff5;
            border-color: #b3ecd1;
        }
        .chat-bubble.theirs {
            justify-self: start;
            background: #fff;
        }
        .chat-meta {
            display: block;
            margin-top: 4px;
            font-size: 0.78rem;
            opacity: 0.72;
        }
"#;
