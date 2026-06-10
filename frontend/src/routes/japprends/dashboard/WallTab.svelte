<script lang="ts">
    type Post = { id: number; pseudo: string; body: string; createdAt: number; };

    let posts = $state<Post[]>([]);
    let loading = $state(true);
    let body = $state('');
    let posting = $state(false);

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleString('fr-FR', {
            day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit'
        });
    }

    async function load() {
        loading = true;
        try {
            const r = await fetch('/japprends/wall');
            posts = await r.json();
        } finally { loading = false; }
    }

    async function post() {
        if (!body.trim()) return;
        posting = true;
        try {
            const r = await fetch('/japprends/wall', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ body: body.trim() }),
            });
            if (r.ok) { body = ''; await load(); }
        } finally { posting = false; }
    }

    async function remove(id: number) {
        await fetch(`/japprends/wall/${id}`, { method: 'DELETE' });
        posts = posts.filter(p => p.id !== id);
    }

    $effect(() => {
        load();
        const id = setInterval(load, 20_000);
        return () => clearInterval(id);
    });
</script>

<div class="wall-tab">
    <div class="tab-header">
        <h2>Mur communautaire</h2>
    </div>

    <div class="compose">
        <textarea
            bind:value={body}
            placeholder="Écrire un post (140 car. max)…"
            maxlength="140"
            rows="2"
            onkeydown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); post(); } }}
        ></textarea>
        <div class="compose-footer">
            <span class="char-count" class:warn={body.length > 120}>{body.length}/140</span>
            <button class="btn-post" onclick={post} disabled={posting || !body.trim()}>
                {posting ? '…' : 'Poster'}
            </button>
        </div>
    </div>

    {#if loading}
        <p class="loading">Chargement…</p>
    {:else if posts.length === 0}
        <p class="empty">Aucun post pour l'instant.</p>
    {:else}
        <div class="post-list">
            {#each posts as p (p.id)}
                <div class="post-row">
                    <div class="post-header">
                        <span class="post-pseudo">{p.pseudo}</span>
                        <span class="post-date">{fmt(p.createdAt)}</span>
                        <button class="btn-del" onclick={() => remove(p.id)}>✕</button>
                    </div>
                    <p class="post-body">{p.body}</p>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .wall-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }

    .compose {
        display: flex; flex-direction: column; gap: 0.5rem;
        padding: 0.75rem; background: var(--muted);
        border: 1px solid var(--border); border-radius: var(--radius-md);
    }
    .compose textarea { resize: vertical; min-height: 60px; }
    .compose-footer { display: flex; align-items: center; justify-content: space-between; }
    .char-count { font-size: 0.75rem; color: var(--muted-foreground); }
    .char-count.warn { color: var(--destructive); }
    .btn-post {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 6px 16px;
        font: 500 0.875rem/1 var(--font-sans); cursor: pointer;
    }
    .btn-post:disabled { opacity: 0.4; cursor: not-allowed; }

    .loading, .empty { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .post-list { display: flex; flex-direction: column; gap: 0.5rem; }
    .post-row {
        padding: 0.75rem; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
    }
    .post-header { display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.375rem; }
    .post-pseudo { font-weight: 600; font-size: 0.875rem; }
    .post-date { font-size: 0.75rem; color: var(--muted-foreground); flex: 1; }
    .post-body { margin: 0; font-size: 0.9rem; line-height: 1.5; white-space: pre-wrap; }
    .btn-del {
        background: none; border: none; color: var(--muted-foreground);
        cursor: pointer; font-size: 0.75rem; padding: 2px 6px;
        border-radius: var(--radius-sm);
    }
    .btn-del:hover { background: var(--destructive-bg); color: var(--destructive); }
</style>
