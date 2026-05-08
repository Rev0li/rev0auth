// Chat popup module — 50% screen popup, notifications badge
pub const JS_FRIEND_CHAT_MODULE: &str = r#"
function createFriendChatModule(ctx) {
    const { pseudo } = ctx;

    const popup = document.getElementById('chat-popup');
    const chatHistory = document.getElementById('chat-history');
    const chatBodyInput = document.getElementById('chat-body');
    const chatMsg = document.getElementById('chat-msg');
    const fabBadge = document.getElementById('chat-fab-badge');
    let refreshTimer = null;
    let bgPollTimer = null;
    let lastKnownCount = 0;
    let isOpen = false;

    function updateBadge(unreadCount) {
        if (!fabBadge) return;
        if (unreadCount > 0) {
            fabBadge.textContent = unreadCount > 9 ? '9+' : String(unreadCount);
            fabBadge.style.display = 'flex';
            document.getElementById('chat-open-btn').classList.add('has-notif');
        } else {
            fabBadge.style.display = 'none';
            document.getElementById('chat-open-btn').classList.remove('has-notif');
        }
    }

    function openChat() {
        isOpen = true;
        popup.classList.add('open');
        if (refreshTimer) clearInterval(refreshTimer);
        refreshTimer = setInterval(loadChatHistory, 8000);
        loadChatHistory();
    }

    function closeChat() {
        isOpen = false;
        popup.classList.remove('open');
        if (refreshTimer) { clearInterval(refreshTimer); refreshTimer = null; }
    }

    function setChatMsg(ok, message) {
        chatMsg.className = 'chat-popup-msg ' + (ok ? 'ok' : 'error');
        chatMsg.textContent = message;
        chatMsg.style.display = 'block';
        if (ok) setTimeout(() => { chatMsg.style.display = 'none'; }, 2500);
    }

    async function fetchMessages() {
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
        return merged;
    }

    // Background poll — only updates the badge, doesn't touch the DOM chat
    async function bgPoll() {
        try {
            const merged = await fetchMessages();
            const unread = merged.filter((m) => !m.mine && !m.is_read).length;
            updateBadge(unread);
        } catch (_) {}
    }

    async function loadChatHistory() {
        try {
            const merged = await fetchMessages();

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
            updateBadge(0);

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

    document.getElementById('chat-open-btn').addEventListener('click', () => {
        isOpen ? closeChat() : openChat();
    });
    document.getElementById('chat-close-btn').addEventListener('click', closeChat);
    document.getElementById('chat-send-btn').addEventListener('click', sendMessage);
    chatBodyInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }
    });

    // Start background polling for notifications (every 15s)
    bgPoll();
    bgPollTimer = setInterval(bgPoll, 15000);

    return { openChat, closeChat, loadChatHistory };
}
"#;

pub const CSS_FRIEND_CHAT_STYLES: &str = r#"
        /* ===== Chat FAB ===== */
        .chat-fab-wrap {
            position: fixed;
            bottom: 28px;
            right: 28px;
            z-index: 900;
            display: flex;
            flex-direction: column;
            align-items: flex-end;
            gap: 0;
        }
        .chat-fab {
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
            position: relative;
        }
        .chat-fab:hover { transform: scale(1.08); box-shadow: 0 6px 24px rgba(0,0,0,0.3); }
        .chat-fab.has-notif { animation: fabPulse 1.6s ease-in-out infinite; }
        @keyframes fabPulse {
            0%, 100% { box-shadow: 0 4px 18px rgba(74,158,255,0.4); }
            50% { box-shadow: 0 4px 28px rgba(74,158,255,0.85); }
        }
        .chat-fab-badge {
            position: absolute;
            top: -4px;
            right: -4px;
            min-width: 20px;
            height: 20px;
            border-radius: 10px;
            background: #e53e3e;
            color: #fff;
            font-size: 0.7rem;
            font-weight: 700;
            display: none;
            align-items: center;
            justify-content: center;
            padding: 0 5px;
            border: 2px solid #fff;
            pointer-events: none;
        }

        /* ===== Chat popup ===== */
        .chat-popup {
            position: fixed;
            bottom: 96px;
            right: 28px;
            width: min(420px, 92vw);
            height: 50vh;
            min-height: 320px;
            border-radius: 16px;
            background: var(--bg-page, #f5f7fa);
            box-shadow: 0 12px 40px rgba(0,0,0,0.22);
            border: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            z-index: 950;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            transform: translateY(20px) scale(0.96);
            opacity: 0;
            pointer-events: none;
            transition: transform 0.22s cubic-bezier(0.4,0,0.2,1), opacity 0.22s ease;
        }
        .chat-popup.open {
            transform: translateY(0) scale(1);
            opacity: 1;
            pointer-events: auto;
        }
        .chat-popup-header {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 12px 14px;
            border-bottom: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            background: var(--bg-card, #fff);
            flex-shrink: 0;
        }
        .chat-popup-avatar {
            width: 34px;
            height: 34px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 0.9rem;
            flex-shrink: 0;
        }
        .chat-popup-title { flex: 1; font-weight: 700; font-size: 0.9rem; }
        .chat-popup-close {
            background: none;
            border: none;
            cursor: pointer;
            font-size: 1.1rem;
            color: var(--color-muted, #888);
            padding: 4px;
            line-height: 1;
        }
        .chat-popup-close:hover { color: var(--color-text, #222); }
        .chat-history {
            flex: 1;
            overflow-y: auto;
            padding: 12px;
            display: flex;
            flex-direction: column;
            gap: 7px;
        }
        .chat-empty {
            text-align: center;
            color: var(--color-muted, #888);
            font-size: 0.875rem;
            margin: auto;
        }
        .chat-bubble {
            max-width: 78%;
            padding: 8px 12px;
            border-radius: 16px;
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
        .chat-bubble-text { font-size: 0.875rem; line-height: 1.45; white-space: pre-wrap; }
        .chat-bubble-time { font-size: 0.7rem; opacity: 0.6; margin-top: 3px; text-align: right; }
        .chat-popup-footer {
            display: flex;
            gap: 8px;
            padding: 10px 12px;
            border-top: 1px solid var(--color-panel-border, rgba(0,0,0,0.1));
            background: var(--bg-card, #fff);
            flex-shrink: 0;
        }
        .chat-overlay-input {
            flex: 1;
            border: 1px solid var(--color-panel-border, rgba(0,0,0,0.15));
            border-radius: 20px;
            padding: 8px 14px;
            font: inherit;
            font-size: 0.875rem;
            background: var(--bg-page, #f5f7fa);
            resize: none;
            max-height: 80px;
            line-height: 1.4;
            outline: none;
            transition: border-color 0.15s;
        }
        .chat-overlay-input:focus { border-color: var(--color-accent, #4a9eff); }
        .chat-overlay-send {
            width: 38px;
            height: 38px;
            border-radius: 50%;
            background: var(--color-accent, #4a9eff);
            color: #fff;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            align-self: flex-end;
            transition: opacity 0.15s;
        }
        .chat-overlay-send:disabled { opacity: 0.45; }
        .chat-popup-msg {
            margin: 0 12px 6px;
            padding: 6px 10px;
            border-radius: 8px;
            font-size: 0.8rem;
            display: none;
            flex-shrink: 0;
        }
        .chat-popup-msg.ok { background: var(--color-success-bg, #d4edda); color: var(--color-success, #155724); display: block; }
        .chat-popup-msg.error { background: var(--color-danger-bg, #f8d7da); color: var(--color-danger, #721c24); display: block; }
"#;
