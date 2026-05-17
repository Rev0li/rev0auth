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
        localStorage.setItem('chat_popup_open', '1');
        if (refreshTimer) clearInterval(refreshTimer);
        refreshTimer = setInterval(loadChatHistory, 8000);
        loadChatHistory();
    }

    function closeChat() {
        isOpen = false;
        popup.classList.remove('open');
        localStorage.removeItem('chat_popup_open');
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

    // Emoji picker
    const EMOJIS = [
        '😀','😃','😄','😁','😆','😅','😂','🤣','😊','😇',
        '🙂','😉','😍','🥰','😘','😋','😜','🤪','😝','🤑',
        '🤗','🤭','🤫','🤔','🤐','😐','😑','😶','😏','😒',
        '🙄','😬','😔','😪','😴','😷','🤒','🤢','🤮','🥵',
        '😵','🤯','🥳','😎','🤓','🧐','😕','😟','😮','😯',
        '😲','🥺','😦','😧','😨','😢','😭','😱','😤','😡',
        '😠','🤬','😈','👿',
        '👍','👎','👌','🤌','✌️','🤞','🤙','👋','✋','💪',
        '🙏','👏','🤝','🫡','🫶',
        '❤️','🧡','💛','💚','💙','💜','🖤','🤍','💔','💕',
        '❤️‍🔥','💯',
        '🌸','🌟','✨','🔥','💫','🌈','⭐','🌙','❄️','🌊',
        '🎉','🎊','🎁','🎵','🎶','💬','👀','💤','💥','🚀',
    ];
    const emojiBtn = document.getElementById('chat-emoji-btn');
    const emojiPanel = document.getElementById('chat-emoji-panel');
    if (emojiBtn && emojiPanel) {
        emojiPanel.innerHTML = EMOJIS.map((e) => '<button class="emoji-pick" type="button">' + e + '</button>').join('');
        emojiBtn.addEventListener('click', (ev) => {
            ev.stopPropagation();
            emojiPanel.classList.toggle('open');
        });
        emojiPanel.addEventListener('click', (ev) => {
            const btn = ev.target.closest('.emoji-pick');
            if (!btn) return;
            chatBodyInput.value += btn.textContent;
            chatBodyInput.focus();
            emojiPanel.classList.remove('open');
        });
        document.addEventListener('click', () => emojiPanel.classList.remove('open'));
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

    // Restore open state across page navigations
    if (localStorage.getItem('chat_popup_open') === '1') {
        openChat();
    }

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
        }
        .chat-fab {
            width: 52px;
            height: 52px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            border: none;
            cursor: pointer;
            font-size: 1.375rem;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: var(--shadow-hover);
            transition: transform 0.15s, opacity 0.15s;
            position: relative;
        }
        .chat-fab:hover { transform: scale(1.06); }
        .chat-fab.has-notif { animation: fabPulse 1.8s ease-in-out infinite; }
        @keyframes fabPulse {
            0%, 100% { box-shadow: var(--shadow-hover); }
            50%       { box-shadow: 0 6px 28px rgba(0,0,0,0.4); }
        }
        .chat-fab-badge {
            position: absolute;
            top: -4px;
            right: -4px;
            min-width: 18px;
            height: 18px;
            border-radius: 9px;
            background: var(--destructive);
            color: #fff;
            font-size: 0.65rem;
            font-weight: 700;
            display: none;
            align-items: center;
            justify-content: center;
            padding: 0 4px;
            border: 2px solid var(--background);
            pointer-events: none;
        }

        /* ===== Chat popup ===== */
        .chat-popup {
            position: fixed;
            bottom: 92px;
            right: 28px;
            width: min(400px, 92vw);
            height: 50vh;
            min-height: 320px;
            border-radius: var(--radius-xl);
            background: var(--background);
            box-shadow: 0 12px 40px rgba(0,0,0,0.18);
            border: 1px solid var(--border);
            z-index: 950;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            transform: translateY(16px) scale(0.97);
            opacity: 0;
            pointer-events: none;
            transition: transform 0.2s cubic-bezier(0.4,0,0.2,1), opacity 0.2s ease;
        }
        .chat-popup.open { transform: translateY(0) scale(1); opacity: 1; pointer-events: auto; }
        .chat-popup-header {
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 11px 14px;
            border-bottom: 1px solid var(--border);
            background: var(--card);
            flex-shrink: 0;
        }
        .chat-popup-avatar {
            width: 30px;
            height: 30px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            font-size: 0.8rem;
            flex-shrink: 0;
        }
        .chat-popup-title { flex: 1; font-weight: 700; font-size: 0.875rem; }
        .chat-popup-close {
            background: none;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            color: var(--muted-foreground);
            padding: 4px;
            line-height: 1;
            border-radius: var(--radius-sm);
            transition: color 0.1s;
        }
        .chat-popup-close:hover { color: var(--foreground); }
        .chat-history {
            flex: 1;
            overflow-y: auto;
            padding: 12px;
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
        .chat-empty {
            text-align: center;
            color: var(--muted-foreground);
            font-size: 0.875rem;
            margin: auto;
        }
        .chat-bubble {
            max-width: 78%;
            padding: 7px 11px;
            border-radius: var(--radius-xl);
            word-break: break-word;
        }
        .chat-bubble.mine {
            align-self: flex-end;
            background: var(--foreground);
            color: var(--background);
            border-bottom-right-radius: var(--radius-sm);
        }
        .chat-bubble.theirs {
            align-self: flex-start;
            background: var(--card);
            border: 1px solid var(--border);
            border-bottom-left-radius: var(--radius-sm);
        }
        .chat-bubble-text { font-size: 0.875rem; line-height: 1.45; white-space: pre-wrap; }
        .chat-bubble-time { font-size: 0.68rem; opacity: 0.55; margin-top: 3px; text-align: right; }
        .chat-popup-footer {
            display: flex;
            gap: 8px;
            padding: 9px 12px;
            border-top: 1px solid var(--border);
            background: var(--card);
            flex-shrink: 0;
            align-items: flex-end;
        }
        .chat-emoji-wrap { position: relative; flex-shrink: 0; }
        .chat-emoji-btn {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--muted);
            cursor: pointer;
            font-size: 0.9rem;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: background 0.1s;
        }
        .chat-emoji-btn:hover { background: var(--border); }
        .chat-emoji-panel {
            position: absolute;
            bottom: 38px;
            left: 0;
            width: 272px;
            max-height: 200px;
            overflow-y: auto;
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: var(--radius-lg);
            padding: 8px;
            display: none;
            grid-template-columns: repeat(6, 1fr);
            gap: 2px;
            box-shadow: var(--shadow-hover);
            z-index: 100;
        }
        .chat-emoji-panel.open { display: grid; }
        .emoji-pick {
            border: none;
            background: none;
            cursor: pointer;
            font-size: 1.15rem;
            padding: 4px;
            border-radius: var(--radius-sm);
            transition: background 0.1s;
            line-height: 1;
        }
        .emoji-pick:hover { background: var(--muted); }
        .chat-overlay-input {
            flex: 1;
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            padding: 7px 13px;
            font: inherit;
            font-size: 1rem;
            background: var(--muted);
            color: var(--foreground);
            resize: none;
            max-height: 80px;
            line-height: 1.4;
            outline: none;
            transition: border-color 0.15s;
        }
        .chat-overlay-input:focus { border-color: var(--foreground); background: var(--card); }
        .chat-overlay-send {
            width: 36px;
            height: 36px;
            border-radius: 50%;
            background: var(--foreground);
            color: var(--background);
            border: none;
            cursor: pointer;
            font-size: 0.9rem;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            align-self: flex-end;
            transition: opacity 0.15s;
        }
        .chat-overlay-send:disabled { opacity: 0.4; }
        .chat-popup-msg {
            margin: 0 12px 6px;
            padding: 5px 10px;
            border-radius: var(--radius-md);
            font-size: 0.8rem;
            display: none;
            flex-shrink: 0;
        }
        .chat-popup-msg.ok    { background: var(--success-bg); color: var(--success); display: block; }
        .chat-popup-msg.error { background: var(--destructive-bg);  color: var(--destructive);  display: block; }

        @media (max-width: 600px) {
            .chat-popup    { right: 12px; bottom: 80px; width: calc(100vw - 24px); }
            .chat-fab-wrap { right: 16px; bottom: 20px; }
        }
"#;
