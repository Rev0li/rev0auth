// Chat overlay module — full-screen messenger style, member ↔ admin
pub const JS_FRIEND_CHAT_MODULE: &str = r#"
function createFriendChatModule(ctx) {
    const { pseudo } = ctx;

    const overlay = document.getElementById('chat-overlay');
    const chatHistory = document.getElementById('chat-history');
    const chatBodyInput = document.getElementById('chat-body');
    const chatMsg = document.getElementById('chat-msg');
    let refreshTimer = null;

    function openChat() {
        overlay.classList.add('open');
        document.body.style.overflow = 'hidden';
        loadChatHistory();
        refreshTimer = setInterval(loadChatHistory, 8000);
    }

    function closeChat() {
        overlay.classList.remove('open');
        document.body.style.overflow = '';
        if (refreshTimer) { clearInterval(refreshTimer); refreshTimer = null; }
    }

    function setChatMsg(ok, message) {
        chatMsg.className = 'chat-overlay-msg ' + (ok ? 'ok' : 'error');
        chatMsg.textContent = message;
        chatMsg.style.display = 'block';
        if (ok) setTimeout(() => { chatMsg.style.display = 'none'; }, 2500);
    }

    async function loadChatHistory() {
        if (!overlay.classList.contains('open')) return;
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
                chatHistory.innerHTML = '<p class="chat-empty">Aucun message pour le moment.</p>';
                return;
            }

            chatHistory.innerHTML = merged.map((m) => {
                const dt = new Date(m.created_at_epoch * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                return '<div class="chat-bubble ' + (m.mine ? 'mine' : 'theirs') + '">'
                    + '<div class="chat-bubble-text">' + escapeHtml(m.body || '') + '</div>'
                    + '<div class="chat-bubble-time">' + dt + '</div>'
                    + '</div>';
            }).join('');
            chatHistory.scrollTop = chatHistory.scrollHeight;
        } catch (err) {
            chatHistory.innerHTML = '<p class="chat-empty">Historique indisponible.</p>';
        }
    }

    async function sendMessage() {
        const body = chatBodyInput.value.trim();
        if (!body) return;

        const sendBtn = document.getElementById('chat-send-btn');
        sendBtn.disabled = true;

        try {
            const res = await fetch('/members/messages/send', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ from_pseudo: pseudo, body })
            });
            const data = await res.json();
            if (data.ok) {
                chatBodyInput.value = '';
                await loadChatHistory();
            } else {
                setChatMsg(false, data.message || 'Erreur envoi.');
            }
        } catch (err) {
            setChatMsg(false, 'Erreur: ' + err.message);
        } finally {
            sendBtn.disabled = false;
        }
    }

    document.getElementById('chat-open-btn').addEventListener('click', openChat);
    document.getElementById('chat-close-btn').addEventListener('click', closeChat);
    overlay.addEventListener('click', (e) => { if (e.target === overlay) closeChat(); });
    document.getElementById('chat-send-btn').addEventListener('click', sendMessage);
    chatBodyInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }
    });

    return { openChat, closeChat, loadChatHistory };
}
"#;

pub const CSS_FRIEND_CHAT_STYLES: &str = r#"
        .chat-fab {
            position: fixed;
            bottom: 28px;
            right: 28px;
            width: 56px;
            height: 56px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border: none;
            cursor: pointer;
            font-size: 1.5rem;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 4px 18px rgba(0,0,0,0.22);
            transition: transform 0.15s, box-shadow 0.15s;
            z-index: 900;
        }
        .chat-fab:hover {
            transform: scale(1.08);
            box-shadow: 0 6px 24px rgba(0,0,0,0.3);
        }
        .chat-overlay {
            position: fixed;
            inset: 0;
            z-index: 950;
            display: flex;
            flex-direction: column;
            background: var(--bg-page, #f5f7fa);
            transform: translateY(100%);
            transition: transform 0.28s cubic-bezier(0.4, 0, 0.2, 1);
        }
        .chat-overlay.open {
            transform: translateY(0);
        }
        .chat-overlay-header {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 14px 18px;
            border-bottom: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            background: var(--bg-card, #fff);
            flex-shrink: 0;
        }
        .chat-overlay-avatar {
            width: 38px;
            height: 38px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 1rem;
            flex-shrink: 0;
        }
        .chat-overlay-title {
            flex: 1;
            font-weight: 700;
            font-size: 1rem;
        }
        .chat-overlay-close {
            background: none;
            border: none;
            cursor: pointer;
            font-size: 1.4rem;
            color: var(--color-muted, #888);
            padding: 4px;
            line-height: 1;
        }
        .chat-overlay-close:hover { color: var(--color-text, #222); }
        .chat-history {
            flex: 1;
            overflow-y: auto;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }
        .chat-empty {
            text-align: center;
            color: var(--color-muted, #888);
            font-size: 0.9rem;
            margin: auto;
        }
        .chat-bubble {
            max-width: 75%;
            padding: 9px 13px;
            border-radius: 18px;
            word-break: break-word;
        }
        .chat-bubble.mine {
            align-self: flex-end;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border-bottom-right-radius: 4px;
        }
        .chat-bubble.theirs {
            align-self: flex-start;
            background: var(--bg-card, #fff);
            border: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            border-bottom-left-radius: 4px;
        }
        .chat-bubble-text { font-size: 0.92rem; line-height: 1.45; white-space: pre-wrap; }
        .chat-bubble-time {
            font-size: 0.72rem;
            opacity: 0.65;
            margin-top: 3px;
            text-align: right;
        }
        .chat-overlay-footer {
            display: flex;
            gap: 10px;
            padding: 12px 16px;
            border-top: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            background: var(--bg-card, #fff);
            flex-shrink: 0;
        }
        .chat-overlay-input {
            flex: 1;
            border: 1px solid var(--color-panel-border, rgba(0,0,0,0.15));
            border-radius: 22px;
            padding: 9px 16px;
            font: inherit;
            font-size: 0.92rem;
            background: var(--bg-page, #f5f7fa);
            resize: none;
            max-height: 100px;
            line-height: 1.4;
        }
        .chat-overlay-input:focus { outline: none; border-color: var(--color-accent, #4a9eff); }
        .chat-overlay-send {
            width: 42px;
            height: 42px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border: none;
            cursor: pointer;
            font-size: 1.1rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            align-self: flex-end;
            transition: opacity 0.15s;
        }
        .chat-overlay-send:disabled { opacity: 0.5; }
        .chat-overlay-msg {
            margin: 0 16px 6px;
            padding: 7px 12px;
            border-radius: 8px;
            font-size: 0.83rem;
            display: none;
        }
        .chat-overlay-msg.ok { background: var(--color-success-bg, #d4edda); color: var(--color-success, #155724); }
        .chat-overlay-msg.error { background: var(--color-danger-bg, #f8d7da); color: var(--color-danger, #721c24); }
"#;
