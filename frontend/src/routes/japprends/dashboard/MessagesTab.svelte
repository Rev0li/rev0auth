<script lang="ts">
    type Msg = { id: number; fromPseudo: string; toPseudo: string; body: string; createdAt: number; isRead: boolean; };
    type Thread = { key: string; participants: [string, string]; messages: Msg[]; lastAt: number; unread: number; };

    let allMessages = $state<Msg[]>([]);
    let loading = $state(true);
    let activeThread = $state<string | null>(null);
    let replyBody = $state('');
    let replying = $state(false);

    function threadKey(a: string, b: string) {
        return [a, b].sort().join('↔');
    }

    const threads = $derived.by<Thread[]>(() => {
        const map = new Map<string, Thread>();
        for (const m of allMessages) {
            const key = threadKey(m.fromPseudo, m.toPseudo);
            if (!map.has(key)) {
                const [a, b] = key.split('↔') as [string, string];
                map.set(key, { key, participants: [a, b], messages: [], lastAt: 0, unread: 0 });
            }
            const t = map.get(key)!;
            t.messages.push(m);
            if (m.createdAt > t.lastAt) t.lastAt = m.createdAt;
            if (!m.isRead) t.unread++;
        }
        return [...map.values()].sort((a, b) => b.lastAt - a.lastAt);
    });

    const active = $derived(threads.find(t => t.key === activeThread) ?? null);

    function otherParticipant(t: Thread) {
        return t.participants.find(p => p !== 'admin') ?? t.participants[0];
    }

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR', {
            day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit'
        });
    }

    async function load() {
        loading = true;
        try {
            const r = await fetch('/japprends/messages');
            allMessages = await r.json();
        } finally { loading = false; }
    }

    async function reply() {
        if (!active || !replyBody.trim()) return;
        const to = otherParticipant(active);
        replying = true;
        try {
            await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to, body: replyBody.trim() }),
            });
            replyBody = '';
            await load();
        } finally { replying = false; }
    }

    $effect(() => { load(); });
</script>

<div class="messages-tab">
    <div class="tab-header">
        <h2>Messages privés</h2>
        <button class="btn-refresh" onclick={load}>↺</button>
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
                        onclick={() => { activeThread = t.key; }}
                    >
                        <span class="thread-name">{otherParticipant(t)}</span>
                        {#if t.unread > 0}
                            <span class="unread-dot">{t.unread}</span>
                        {/if}
                        <span class="thread-date">{fmt(t.lastAt)}</span>
                    </button>
                {/each}
            </aside>

            <div class="conversation">
                {#if !active}
                    <p class="select-hint">Sélectionne une conversation</p>
                {:else}
                    <div class="msg-list">
                        {#each active.messages.sort((a, b) => a.createdAt - b.createdAt) as m (m.id)}
                            <div class="msg-bubble" class:from-admin={m.fromPseudo === 'admin'}>
                                <span class="msg-from">{m.fromPseudo}</span>
                                <p class="msg-body">{m.body}</p>
                                <span class="msg-date">{fmt(m.createdAt)}</span>
                            </div>
                        {/each}
                    </div>

                    <div class="reply-form">
                        <textarea
                            bind:value={replyBody}
                            placeholder="Répondre à {otherParticipant(active)}…"
                            rows="2"
                            onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); reply(); } }}
                        ></textarea>
                        <button class="btn-send" onclick={reply} disabled={replying || !replyBody.trim()}>
                            {replying ? '…' : '➤'}
                        </button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .messages-tab { display: flex; flex-direction: column; gap: 1rem; height: 100%; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .btn-refresh {
        background: var(--muted); border: 1px solid var(--border);
        border-radius: var(--radius-sm); padding: 4px 10px;
        font-size: 1rem; cursor: pointer; color: var(--muted-foreground);
    }

    .loading, .empty { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .msg-layout {
        display: grid; grid-template-columns: 220px 1fr;
        gap: 0; border: 1px solid var(--border); border-radius: var(--radius-md);
        overflow: hidden; min-height: 420px;
    }

    .thread-list {
        border-right: 1px solid var(--border); background: var(--muted);
        overflow-y: auto; display: flex; flex-direction: column;
    }
    .thread-item {
        display: flex; flex-direction: column; gap: 2px; align-items: flex-start;
        padding: 0.625rem 0.875rem; border: none; background: none;
        cursor: pointer; text-align: left; border-bottom: 1px solid var(--border);
        transition: background 0.1s;
    }
    .thread-item:hover { background: var(--card); }
    .thread-item.active { background: var(--card); box-shadow: inset 2px 0 0 var(--primary); }
    .thread-name { font-weight: 600; font-size: 0.875rem; }
    .thread-date { font-size: 0.6875rem; color: var(--muted-foreground); }
    .unread-dot {
        font-size: 0.6875rem; font-weight: 700;
        background: var(--primary); color: var(--primary-foreground);
        border-radius: 99px; padding: 1px 6px;
    }

    .conversation {
        display: flex; flex-direction: column; background: var(--card);
    }
    .select-hint {
        flex: 1; display: flex; align-items: center; justify-content: center;
        color: var(--muted-foreground); font-size: 0.875rem; margin: 0;
    }

    .msg-list {
        flex: 1; overflow-y: auto; padding: 1rem;
        display: flex; flex-direction: column; gap: 0.75rem;
    }
    .msg-bubble {
        display: flex; flex-direction: column; gap: 2px;
        max-width: 70%; padding: 0.5rem 0.75rem;
        border-radius: var(--radius-md); background: var(--muted);
        border: 1px solid var(--border); align-self: flex-start;
    }
    .msg-bubble.from-admin { align-self: flex-end; background: var(--primary); color: var(--primary-foreground); border-color: transparent; }
    .msg-from { font-size: 0.6875rem; font-weight: 600; opacity: 0.7; }
    .msg-body { margin: 0; font-size: 0.875rem; line-height: 1.5; white-space: pre-wrap; }
    .msg-date { font-size: 0.625rem; opacity: 0.6; align-self: flex-end; }

    .reply-form {
        display: flex; gap: 0.5rem; padding: 0.75rem;
        border-top: 1px solid var(--border);
    }
    .reply-form textarea { flex: 1; resize: none; }
    .btn-send {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 8px 14px;
        font-size: 1rem; cursor: pointer; align-self: flex-end;
    }
    .btn-send:disabled { opacity: 0.4; cursor: not-allowed; }

    @media (max-width: 600px) {
        .msg-layout { grid-template-columns: 1fr; }
        .thread-list { border-right: none; border-bottom: 1px solid var(--border); max-height: 200px; }
    }
</style>
