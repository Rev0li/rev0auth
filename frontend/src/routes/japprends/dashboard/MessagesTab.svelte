<script lang="ts">
    type Msg = { id: number; fromPseudo: string; toPseudo: string; body: string; createdAt: number; isRead: boolean; };
    type Thread = { key: string; other: string; messages: Msg[]; lastAt: number; unread: number; };

    let allMessages = $state<Msg[]>([]);
    let loading = $state(true);
    let activeThread = $state<string | null>(null);
    let replyBody = $state('');
    let replying = $state(false);

    const EMOJIS = ['😊','👍','❤️','🎵','🔥','😂','🙏','✨','🎉','😎','💪','🤔'];

    function threadKey(a: string, b: string) { return [a, b].sort().join('↔'); }

    const threads = $derived.by<Thread[]>(() => {
        const map = new Map<string, Thread>();
        for (const m of allMessages) {
            const key = threadKey(m.fromPseudo, m.toPseudo);
            if (!map.has(key)) {
                const other = m.fromPseudo === 'admin' ? m.toPseudo : m.fromPseudo;
                map.set(key, { key, other, messages: [], lastAt: 0, unread: 0 });
            }
            const t = map.get(key)!;
            t.messages.push(m);
            if (m.createdAt > t.lastAt) t.lastAt = m.createdAt;
            if (!m.isRead && m.fromPseudo !== 'admin') t.unread++;
        }
        return [...map.values()].sort((a, b) => b.lastAt - a.lastAt);
    });

    const active = $derived(threads.find(t => t.key === activeThread) ?? null);

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR', {
            day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit'
        });
    }

    async function load(silent = false) {
        if (!silent) loading = true;
        try {
            const r = await fetch('/japprends/messages');
            allMessages = await r.json();
        } finally { if (!silent) loading = false; }
    }

    async function selectThread(key: string, other: string) {
        activeThread = key;
        // Mark thread as read
        const thread = threads.find(t => t.key === key);
        if (thread && thread.unread > 0) {
            await fetch('/japprends/messages', {
                method: 'PATCH',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: other }),
            });
            allMessages = allMessages.map(m =>
                (m.fromPseudo === other || m.toPseudo === other) ? { ...m, isRead: true } : m
            );
        }
    }

    async function deleteThread(other: string) {
        if (!confirm(`Supprimer toute la conversation avec ${other} ?`)) return;
        await fetch('/japprends/messages', {
            method: 'DELETE',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ pseudo: other }),
        });
        allMessages = allMessages.filter(m => m.fromPseudo !== other && m.toPseudo !== other);
        activeThread = null;
    }

    async function reply() {
        if (!active || !replyBody.trim()) return;
        replying = true;
        try {
            await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to: active.other, body: replyBody.trim() }),
            });
            replyBody = '';
            await load();
            activeThread = active.key;
        } finally { replying = false; }
    }

    function insertEmoji(emoji: string) {
        replyBody += emoji;
    }

    $effect(() => {
        load();
        const id = setInterval(() => load(true), 8_000);
        return () => clearInterval(id);
    });
</script>

<div class="messages-tab">
    <div class="tab-header">
        <h2>Messages privés</h2>
    </div>

    {#if loading}
        <p class="loading">Chargement…</p>
    {:else if threads.length === 0}
        <p class="empty">Aucune conversation.</p>
    {:else}
        <div class="msg-layout">
            <aside class="thread-list">
                {#each threads as t (t.key)}
                    <button
                        class="thread-item"
                        class:active={activeThread === t.key}
                        onclick={() => selectThread(t.key, t.other)}
                    >
                        <span class="thread-name">{t.other}</span>
                        <div class="thread-meta">
                            {#if t.unread > 0}
                                <span class="unread-dot">{t.unread}</span>
                            {/if}
                            <span class="thread-date">{fmt(t.lastAt)}</span>
                        </div>
                    </button>
                {/each}
            </aside>

            <div class="conversation">
                {#if !active}
                    <p class="select-hint">Sélectionne une conversation</p>
                {:else}
                    <div class="conv-header">
                        <span class="conv-title">{active.other}</span>
                        <button class="btn-del-thread" onclick={() => deleteThread(active!.other)} title="Supprimer la conversation">
                            🗑
                        </button>
                    </div>

                    <div class="msg-list">
                        {#each active.messages.toSorted((a, b) => a.createdAt - b.createdAt) as m (m.id)}
                            <div class="msg-bubble" class:from-admin={m.fromPseudo === 'admin'}>
                                <span class="msg-from">{m.fromPseudo}</span>
                                <p class="msg-body">{m.body}</p>
                                <span class="msg-date">{fmt(m.createdAt)}</span>
                            </div>
                        {/each}
                    </div>

                    <div class="reply-area">
                        <div class="emoji-bar">
                            {#each EMOJIS as e}
                                <button class="emoji-btn" onclick={() => insertEmoji(e)}>{e}</button>
                            {/each}
                        </div>
                        <div class="reply-form">
                            <textarea
                                bind:value={replyBody}
                                placeholder="Répondre à {active.other}…"
                                rows="2"
                                onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); reply(); } }}
                            ></textarea>
                            <button class="btn-send" onclick={reply} disabled={replying || !replyBody.trim()}>
                                {replying ? '…' : '➤'}
                            </button>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .messages-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .btn-refresh {
        background: var(--muted); border: 1px solid var(--border);
        border-radius: var(--radius-sm); padding: 4px 10px;
        font-size: 1rem; cursor: pointer; color: var(--muted-foreground);
    }

    .loading, .empty { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .msg-layout {
        display: grid; grid-template-columns: 200px 1fr;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        overflow: hidden; min-height: 460px;
    }

    .thread-list {
        border-right: 1px solid var(--border); background: var(--muted);
        overflow-y: auto; display: flex; flex-direction: column;
    }
    .thread-item {
        display: flex; flex-direction: column; gap: 4px;
        padding: 0.625rem 0.875rem; border: none; background: none;
        cursor: pointer; text-align: left; border-bottom: 1px solid var(--border);
        transition: background 0.1s;
    }
    .thread-item:hover { background: var(--card); }
    .thread-item.active { background: var(--card); box-shadow: inset 2px 0 0 var(--primary); }
    .thread-name { font-weight: 600; font-size: 0.875rem; }
    .thread-meta { display: flex; align-items: center; gap: 0.375rem; }
    .thread-date { font-size: 0.6875rem; color: var(--muted-foreground); }
    .unread-dot {
        font-size: 0.6875rem; font-weight: 700;
        background: var(--primary); color: var(--primary-foreground);
        border-radius: 99px; padding: 1px 6px;
    }

    .conversation { display: flex; flex-direction: column; background: var(--card); overflow: hidden; }
    .select-hint {
        flex: 1; display: flex; align-items: center; justify-content: center;
        color: var(--muted-foreground); font-size: 0.875rem; margin: 0;
    }

    .conv-header {
        display: flex; align-items: center; justify-content: space-between;
        padding: 0.625rem 0.875rem; border-bottom: 1px solid var(--border);
        background: var(--muted);
    }
    .conv-title { font-weight: 600; font-size: 0.9rem; }
    .btn-del-thread {
        background: none; border: none; cursor: pointer; font-size: 1rem;
        padding: 2px 6px; border-radius: var(--radius-sm); opacity: 0.6;
    }
    .btn-del-thread:hover { opacity: 1; background: var(--destructive-bg); }

    .msg-list {
        flex: 1; overflow-y: auto; padding: 0.875rem;
        display: flex; flex-direction: column; gap: 0.625rem;
    }
    .msg-bubble {
        display: flex; flex-direction: column; gap: 2px;
        max-width: 72%; padding: 0.5rem 0.75rem;
        border-radius: var(--radius-md); background: var(--muted);
        border: 1px solid var(--border); align-self: flex-start;
    }
    .msg-bubble.from-admin {
        align-self: flex-end; background: var(--primary);
        color: var(--primary-foreground); border-color: transparent;
    }
    .msg-from { font-size: 0.6875rem; font-weight: 600; opacity: 0.7; }
    .msg-body { margin: 0; font-size: 0.875rem; line-height: 1.5; white-space: pre-wrap; }
    .msg-date { font-size: 0.625rem; opacity: 0.6; align-self: flex-end; }

    .reply-area { border-top: 1px solid var(--border); }
    .emoji-bar {
        display: flex; flex-wrap: wrap; gap: 2px;
        padding: 0.375rem 0.625rem; border-bottom: 1px solid var(--border);
        background: var(--muted);
    }
    .emoji-btn {
        background: none; border: none; font-size: 1.1rem; cursor: pointer;
        padding: 2px 4px; border-radius: var(--radius-sm); line-height: 1;
        transition: background 0.1s;
    }
    .emoji-btn:hover { background: var(--border); }

    .reply-form { display: flex; gap: 0.5rem; padding: 0.625rem; }
    .reply-form textarea { flex: 1; resize: none; }
    .btn-send {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 8px 14px;
        font-size: 1rem; cursor: pointer; align-self: flex-end;
    }
    .btn-send:disabled { opacity: 0.4; cursor: not-allowed; }

    @media (max-width: 600px) {
        .msg-layout { grid-template-columns: 1fr; }
        .thread-list { border-right: none; border-bottom: 1px solid var(--border); max-height: 180px; }
    }
</style>
