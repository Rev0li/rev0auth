// Community wall module — post / delete short messages
pub const JS_FRIEND_WALL_MODULE: &str = r#"
function createFriendWallModule(ctx) {
    const { pseudo } = ctx;

    const wallList = document.getElementById('wall-list');
    const wallInput = document.getElementById('wall-input');
    const wallSendBtn = document.getElementById('wall-send-btn');
    const wallCharCount = document.getElementById('wall-char-count');

    if (!wallList || !wallInput || !wallSendBtn) return { loadWall: async () => {} };

    const MAX_CHARS = 140;

    wallInput.addEventListener('input', () => {
        const remaining = MAX_CHARS - wallInput.value.length;
        if (wallCharCount) {
            wallCharCount.textContent = String(remaining);
            wallCharCount.className = 'wall-char-count' + (remaining < 20 ? ' warn' : '');
        }
    });

    async function loadWall() {
        try {
            const res = await fetch('/members/wall', { cache: 'no-store' });
            const posts = await res.json();
            if (!Array.isArray(posts) || posts.length === 0) {
                wallList.innerHTML = '<p class="wall-empty">Soyez le premier à laisser un message.</p>';
                return;
            }
            wallList.innerHTML = posts.map((p) => {
                const d = new Date(p.created_at_epoch * 1000);
                const dt = d.toLocaleDateString([], { day: '2-digit', month: '2-digit' })
                    + ' ' + d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
                const isOwn = p.pseudo.toLowerCase() === pseudo.toLowerCase();
                return '<div class="wall-post" id="wall-post-' + p.id + '">'
                    + '<div class="wall-post-header">'
                    + '<span class="wall-post-pseudo">' + escapeHtml(p.pseudo) + '</span>'
                    + '<span class="wall-post-time">' + dt + '</span>'
                    + (isOwn
                        ? '<button class="wall-delete-btn" data-id="' + p.id + '" title="Supprimer">✕</button>'
                        : '')
                    + '</div>'
                    + '<p class="wall-post-body">' + escapeHtml(p.body) + '</p>'
                    + '</div>';
            }).join('');
            wallList.querySelectorAll('.wall-delete-btn').forEach((btn) => {
                btn.addEventListener('click', () => deletePost(parseInt(btn.getAttribute('data-id'), 10)));
            });
        } catch (_) {
            wallList.innerHTML = '<p class="wall-empty">Erreur de chargement.</p>';
        }
    }

    async function postMessage() {
        const body = wallInput.value.trim();
        if (!body || body.length > MAX_CHARS) return;
        wallSendBtn.disabled = true;
        try {
            const res = await fetch('/members/wall', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo, body })
            });
            const data = await res.json();
            if (data.ok) {
                wallInput.value = '';
                if (wallCharCount) { wallCharCount.textContent = String(MAX_CHARS); wallCharCount.className = 'wall-char-count'; }
                await loadWall();
            }
        } catch (_) {}
        wallSendBtn.disabled = false;
    }

    async function deletePost(id) {
        try {
            await fetch('/members/wall/' + id, {
                method: 'DELETE',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo })
            });
            await loadWall();
        } catch (_) {}
    }

    wallSendBtn.addEventListener('click', postMessage);
    wallInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); postMessage(); }
    });

    loadWall();

    return { loadWall };
}
"#;

pub const CSS_FRIEND_WALL_STYLES: &str = r#"
        .wall-list {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            min-height: 72px;
            max-height: 260px;
            overflow-y: auto;
            padding: 8px;
            display: flex;
            flex-direction: column;
            gap: 6px;
            margin-bottom: 10px;
        }
        .wall-post {
            display: flex;
            gap: 10px;
            align-items: flex-start;
            padding: 8px 10px;
            border-radius: var(--radius-lg);
            border: 1px solid var(--border);
            background: var(--muted);
            font-size: 0.875rem;
        }
        .wall-post-header {
            display: flex;
            align-items: center;
            gap: 6px;
            margin-bottom: 3px;
        }
        .wall-post-pseudo {
            font-weight: 700;
            font-size: 0.8125rem;
            color: var(--foreground);
        }
        .wall-post-time {
            font-size: 0.75rem;
            color: var(--muted-foreground);
            flex: 1;
        }
        .wall-delete-btn {
            background: none;
            border: none;
            cursor: pointer;
            color: var(--muted-foreground);
            font-size: 0.8125rem;
            padding: 2px 5px;
            border-radius: var(--radius-sm);
            transition: color 0.1s, background 0.1s;
            line-height: 1;
        }
        .wall-delete-btn:hover { color: var(--destructive); background: var(--destructive-bg); }
        .wall-post-body {
            margin: 0;
            font-size: 0.875rem;
            line-height: 1.5;
            white-space: pre-wrap;
            word-break: break-word;
            flex: 1;
        }
        .wall-empty {
            text-align: center;
            color: var(--muted-foreground);
            font-size: 0.875rem;
            padding: 18px 0;
            margin: 0;
        }
        .wall-compose {
            border: 1px solid var(--border);
            border-radius: var(--radius-xl);
            background: var(--card);
            padding: 10px 12px;
            box-shadow: var(--shadow-soft);
        }
        .wall-input {
            width: 100%;
            border: none;
            background: none;
            font: inherit;
            font-size: 0.9rem;
            color: var(--foreground);
            resize: none;
            outline: none;
            line-height: 1.5;
        }
        .wall-compose-footer {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-top: 6px;
        }
        .wall-char-count {
            font-size: 0.75rem;
            color: var(--muted-foreground);
            font-variant-numeric: tabular-nums;
        }
        .wall-char-count.warn { color: var(--destructive); font-weight: 600; }
"#;
