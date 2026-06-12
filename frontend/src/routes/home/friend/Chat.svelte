<script lang="ts">
    import { slide } from 'svelte/transition';
    import { tick } from 'svelte';

    type Thread  = { peer: string; lastBody: string; lastAt: number; lastFromMe: boolean; unread: number };
    type Message = { id: number; fromPseudo: string; toPseudo: string; body: string; isRead: boolean; createdAt: number };

    let { myPseudo, adminPseudo, members, initialUnread }: {
        myPseudo: string;
        adminPseudo: string;
        members: { pseudo: string }[];
        initialUnread: number;
    } = $props();

    let open       = $state(false);
    let view       = $state<'list' | 'thread'>('list');
    let threads    = $state<Thread[]>([]);
    let activePeer = $state('');
    let msgs       = $state<Message[]>([]);
    let draft      = $state('');
    let search     = $state('');
    let sending    = $state(false);
    let unread     = $state(initialUnread);
    let msgsEl     = $state<HTMLElement | null>(null);

    const isAdmin = (p: string) => p.toLowerCase() === adminPseudo.toLowerCase();
    const isMine  = (m: Message) => m.fromPseudo.toLowerCase() === myPseudo.toLowerCase();

    const EMOJIS = ['😄', '😂', '❤️', '👍', '🎉', '🔥', '💡', '🎬', '🍿', '🎵'];

    // Fil Admin toujours en tête, même vide
    let adminThread = $derived(
        threads.find(t => isAdmin(t.peer))
        ?? { peer: adminPseudo, lastBody: 'Une question ? Écris-moi.', lastAt: 0, lastFromMe: false, unread: 0 }
    );
    let memberThreads = $derived(threads.filter(t => !isAdmin(t.peer)));

    // Recherche : membres actifs sans conversation existante (et pas moi)
    let searchResults = $derived.by(() => {
        const q = search.trim().toLowerCase();
        if (!q) return [];
        const known = new Set(threads.map(t => t.peer.toLowerCase()));
        return members
            .filter(m => m.pseudo.toLowerCase() !== myPseudo.toLowerCase()
                && !known.has(m.pseudo.toLowerCase())
                && m.pseudo.toLowerCase().includes(q))
            .slice(0, 6);
    });

    async function loadThreads() {
        const r = await fetch('/members/messages');
        if (!r.ok) return;
        const d = await r.json() as { threads: Thread[] };
        threads = d.threads;
        unread = threads.reduce((n, t) => n + t.unread, 0);
    }

    async function scrollToBottom() {
        await tick();
        if (msgsEl) msgsEl.scrollTop = msgsEl.scrollHeight;
    }

    async function openThread(peer: string) {
        activePeer = peer;
        view = 'thread';
        msgs = [];
        search = '';
        const r = await fetch(`/members/messages?with=${encodeURIComponent(peer)}`);
        if (r.ok) {
            const d = await r.json() as { messages: Message[] };
            msgs = d.messages;
        }
        scrollToBottom();
        // Marque la conversation comme lue
        await fetch('/members/messages', {
            method: 'PATCH',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ with: peer }),
        });
        threads = threads.map(t => t.peer === peer ? { ...t, unread: 0 } : t);
        unread = threads.reduce((n, t) => n + t.unread, 0);
    }

    async function send() {
        const body = draft.trim();
        if (!body || sending) return;
        sending = true;
        try {
            const r = await fetch('/members/messages', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to: activePeer, body }),
            });
            if (r.ok) {
                draft = '';
                const fresh = await fetch(`/members/messages?with=${encodeURIComponent(activePeer)}`);
                if (fresh.ok) msgs = (await fresh.json() as { messages: Message[] }).messages;
                scrollToBottom();
                loadThreads();
            }
        } finally { sending = false; }
    }

    function toggle() {
        open = !open;
        if (open) { view = 'list'; loadThreads(); }
    }

    function timeAgo(epoch: number) {
        const sec = Math.floor(Date.now() / 1000 - epoch);
        if (sec < 60) return 'à l\'instant';
        if (sec < 3600) return `${Math.floor(sec / 60)} min`;
        if (sec < 86400) return `${Math.floor(sec / 3600)} h`;
        return `${Math.floor(sec / 86400)} j`;
    }
</script>

<button class="chat-fab" onclick={toggle} aria-label="Messagerie">
    💬
    {#if unread > 0}
        <span class="chat-badge">{unread}</span>
    {/if}
</button>

{#if open}
    <div class="chat-panel card" transition:slide={{ axis: 'y', duration: 250 }}>

        {#if view === 'list'}
            <div class="chat-header">
                <span>Messages</span>
                <button class="chat-close" onclick={() => open = false} aria-label="Fermer">×</button>
            </div>

            <div class="chat-body">
                <!-- Fil admin permanent -->
                <button class="thread-row thread-admin" onclick={() => openThread(adminThread.peer)}>
                    <span class="thread-avatar">🛟</span>
                    <span class="thread-main">
                        <span class="thread-name">Admin <span class="thread-tag">support</span></span>
                        <span class="thread-preview">{adminThread.lastFromMe ? 'Toi : ' : ''}{adminThread.lastBody}</span>
                    </span>
                    {#if adminThread.unread > 0}<span class="thread-unread">{adminThread.unread}</span>{/if}
                </button>

                <!-- Conversations entre membres -->
                {#each memberThreads as t (t.peer)}
                    <button class="thread-row" onclick={() => openThread(t.peer)}>
                        <img class="thread-avatar-img" src="/members/avatar/{t.peer}" alt={t.peer}
                            onerror={(e) => (e.currentTarget as HTMLImageElement).style.visibility = 'hidden'} />
                        <span class="thread-main">
                            <span class="thread-name">{t.peer}</span>
                            <span class="thread-preview">{t.lastFromMe ? 'Toi : ' : ''}{t.lastBody}</span>
                        </span>
                        <span class="thread-time">{timeAgo(t.lastAt)}</span>
                        {#if t.unread > 0}<span class="thread-unread">{t.unread}</span>{/if}
                    </button>
                {/each}

                <!-- Nouvelle conversation -->
                <div class="chat-search">
                    <input
                        type="text"
                        bind:value={search}
                        placeholder="Écrire à un membre…"
                        class="chat-search-input"
                    />
                    {#each searchResults as m (m.pseudo)}
                        <button class="thread-row" onclick={() => openThread(m.pseudo)}>
                            <img class="thread-avatar-img" src="/members/avatar/{m.pseudo}" alt={m.pseudo}
                                onerror={(e) => (e.currentTarget as HTMLImageElement).style.visibility = 'hidden'} />
                            <span class="thread-main">
                                <span class="thread-name">{m.pseudo}</span>
                                <span class="thread-preview">Démarrer une conversation</span>
                            </span>
                        </button>
                    {/each}
                    {#if search.trim() && searchResults.length === 0}
                        <p class="chat-empty">Aucun membre trouvé.</p>
                    {/if}
                </div>
            </div>

        {:else}
            <div class="chat-header">
                <button class="chat-back" onclick={() => { view = 'list'; loadThreads(); }} aria-label="Retour">←</button>
                <span class="chat-peer">
                    {#if isAdmin(activePeer)}🛟 Admin{:else}{activePeer}{/if}
                </span>
                <button class="chat-close" onclick={() => open = false} aria-label="Fermer">×</button>
            </div>

            <div class="chat-msgs" bind:this={msgsEl}>
                {#each msgs as m (m.id)}
                    <div class="bubble" class:mine={isMine(m)}>
                        <p class="bubble-body">{m.body}</p>
                        <span class="bubble-time">{timeAgo(m.createdAt)}</span>
                    </div>
                {:else}
                    <p class="chat-empty">
                        {#if isAdmin(activePeer)}Une question, un souci d'accès ? C'est ici.{:else}Dis bonjour 👋{/if}
                    </p>
                {/each}
            </div>

            <div class="chat-emojis">
                {#each EMOJIS as e (e)}
                    <button class="emoji-btn" onclick={() => { draft += e; }} aria-label="Ajouter {e}">{e}</button>
                {/each}
            </div>
            <div class="chat-compose">
                <textarea
                    bind:value={draft}
                    placeholder="Ton message…"
                    rows={1}
                    class="chat-input"
                    onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send(); } }}
                ></textarea>
                <button class="chat-send" onclick={send} disabled={sending || !draft.trim()} aria-label="Envoyer">➤</button>
            </div>
        {/if}

    </div>
{/if}

<style>
    .chat-fab {
        position: fixed; bottom: 24px; right: 24px; z-index: 200;
        width: 52px; height: 52px; border-radius: 50%;
        border: 1px solid var(--border);
        background: var(--primary);
        font-size: 1.25rem;
        cursor: pointer;
        display: flex; align-items: center; justify-content: center;
        box-shadow: var(--shadow-hover);
        transition: transform 0.15s;
    }
    .chat-fab:hover { transform: scale(1.08); }
    .chat-badge {
        position: absolute; top: -4px; right: -4px;
        background: var(--destructive);
        color: #fff; font-size: 0.6875rem; font-weight: 700;
        border-radius: 99px; padding: 1px 5px;
        min-width: 18px; text-align: center;
    }

    .chat-panel {
        position: fixed; bottom: 88px; right: 24px; z-index: 199;
        width: 340px; height: min(520px, 70vh);
        display: flex; flex-direction: column;
        overflow: hidden;
    }

    .chat-header {
        display: flex; align-items: center; gap: 8px;
        padding: 12px 16px; border-bottom: 1px solid var(--border);
        font-weight: 600; font-size: 0.9375rem;
        flex-shrink: 0;
    }
    .chat-header > span:first-child { flex: 1; }
    .chat-peer { flex: 1; }
    .chat-back, .chat-close {
        background: none; border: none; cursor: pointer;
        font-size: 1.25rem; color: var(--muted-foreground);
        line-height: 1; padding: 0;
    }
    .chat-back:hover, .chat-close:hover { color: var(--foreground); }

    .chat-body { overflow-y: auto; flex: 1; padding: 8px; }

    /* ── Liste des conversations ── */
    .thread-row {
        display: flex; align-items: center; gap: 10px;
        width: 100%; padding: 10px;
        background: none; border: none; cursor: pointer;
        border-radius: var(--radius-md);
        text-align: left;
        transition: background 0.12s;
    }
    .thread-row:hover { background: var(--muted); }
    .thread-admin { background: color-mix(in srgb, var(--primary) 7%, transparent); }
    .thread-avatar {
        width: 34px; height: 34px; border-radius: 50%;
        background: var(--muted);
        display: flex; align-items: center; justify-content: center;
        font-size: 1rem; flex-shrink: 0;
    }
    .thread-avatar-img {
        width: 34px; height: 34px; border-radius: 50%;
        object-fit: cover; flex-shrink: 0;
        border: 1px solid var(--border);
    }
    .thread-main { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 1px; }
    .thread-name { font-size: 0.875rem; font-weight: 600; color: var(--foreground); }
    .thread-tag {
        font-size: 0.625rem; font-weight: 600; text-transform: uppercase;
        background: var(--primary); color: var(--primary-foreground);
        border-radius: 99px; padding: 1px 6px; margin-left: 4px;
        vertical-align: middle;
    }
    .thread-preview {
        font-size: 0.75rem; color: var(--muted-foreground);
        white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
    }
    .thread-time { font-size: 0.6875rem; color: var(--muted-foreground); flex-shrink: 0; }
    .thread-unread {
        background: var(--destructive); color: #fff;
        font-size: 0.6875rem; font-weight: 700;
        border-radius: 99px; padding: 1px 6px; min-width: 18px;
        text-align: center; flex-shrink: 0;
    }

    .chat-search { margin-top: 8px; padding-top: 8px; border-top: 1px solid var(--border); }
    .chat-search-input {
        width: 100%; padding: 8px 10px;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--background); color: var(--foreground);
        font-size: 0.8125rem;
    }
    .chat-search-input:focus { outline: none; border-color: var(--primary); }
    .chat-empty {
        color: var(--muted-foreground); font-size: 0.8125rem;
        text-align: center; padding: 1.25rem 0.5rem; margin: 0;
    }

    /* ── Fil de discussion ── */
    .chat-msgs {
        flex: 1; overflow-y: auto;
        padding: 12px; display: flex; flex-direction: column; gap: 8px;
    }
    .bubble {
        max-width: 80%;
        align-self: flex-start;
        background: var(--muted);
        border-radius: 14px 14px 14px 4px;
        padding: 8px 12px;
    }
    .bubble.mine {
        align-self: flex-end;
        background: color-mix(in srgb, var(--primary) 18%, var(--card));
        border-radius: 14px 14px 4px 14px;
    }
    .bubble-body { margin: 0; font-size: 0.875rem; line-height: 1.45; white-space: pre-wrap; word-break: break-word; }
    .bubble-time { display: block; font-size: 0.625rem; color: var(--muted-foreground); margin-top: 2px; text-align: right; }

    .chat-emojis {
        display: flex; gap: 2px; flex-wrap: nowrap; overflow-x: auto;
        padding: 6px 10px 0; border-top: 1px solid var(--border);
        flex-shrink: 0;
    }
    .emoji-btn {
        background: none; border: none; cursor: pointer;
        font-size: 1rem; padding: 2px 4px; border-radius: var(--radius-sm);
        transition: background 0.12s, transform 0.12s;
    }
    .emoji-btn:hover { background: var(--muted); transform: scale(1.2); }

    .chat-compose {
        display: flex; gap: 6px; align-items: flex-end;
        padding: 10px; flex-shrink: 0;
    }
    .chat-input {
        flex: 1; resize: none;
        padding: 8px 10px; max-height: 90px;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--background); color: var(--foreground);
        font: 400 0.875rem/1.4 var(--font-sans);
    }
    .chat-input:focus { outline: none; border-color: var(--primary); }
    .chat-send {
        width: 36px; height: 36px; border-radius: 50%;
        border: none; background: var(--primary);
        color: var(--primary-foreground); font-size: 0.875rem;
        cursor: pointer; flex-shrink: 0;
    }
    .chat-send:disabled { opacity: 0.5; cursor: default; }
</style>
