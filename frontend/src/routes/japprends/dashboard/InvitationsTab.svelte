<script lang="ts">
    type Invite = { id: number; code: string; note: string; createdAt: number; expiresAt: number; usedBy: string | null; usedAt: number | null; };

    let invites = $state<Invite[]>([]);
    let loading = $state(true);
    let note = $state('');
    let generating = $state(false);
    let copied = $state<number | null>(null);
    let busy = $state<Record<number, boolean>>({});

    const now = Math.floor(Date.now() / 1000);

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleDateString('fr-FR', {
            day: '2-digit', month: 'short', year: 'numeric'
        });
    }

    function isExpired(inv: Invite) { return inv.expiresAt < now; }
    function isUsed(inv: Invite) { return !!inv.usedBy; }

    async function load() {
        loading = true;
        try {
            const r = await fetch('/japprends/invites');
            invites = await r.json();
        } finally { loading = false; }
    }

    async function generate() {
        generating = true;
        try {
            await fetch('/japprends/invites', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ note: note.trim() }),
            });
            note = '';
            await load();
        } finally { generating = false; }
    }

    async function copy(inv: Invite) {
        const url = `${location.origin}/signup?invite=${inv.code}`;
        await navigator.clipboard.writeText(url);
        copied = inv.id;
        setTimeout(() => { if (copied === inv.id) copied = null; }, 2000);
    }

    async function remove(id: number) {
        busy[id] = true;
        try {
            await fetch(`/japprends/invites/${id}`, { method: 'DELETE' });
            invites = invites.filter(i => i.id !== id);
        } finally { busy[id] = false; }
    }

    $effect(() => { load(); });
</script>

<div class="invites-tab">
    <div class="tab-header">
        <h2>Invitations</h2>
        {#if invites.filter(i => !isUsed(i) && !isExpired(i)).length > 0}
            <span class="chip-active">{invites.filter(i => !isUsed(i) && !isExpired(i)).length} active{invites.filter(i => !isUsed(i) && !isExpired(i)).length > 1 ? 's' : ''}</span>
        {/if}
    </div>

    <div class="generate-form">
        <input bind:value={note} placeholder="Note (ex: pour Alice)" maxlength="80" />
        <button class="btn-generate" onclick={generate} disabled={generating}>
            {generating ? '…' : '+ Générer (7 jours)'}
        </button>
    </div>

    {#if loading}
        <p class="loading">Chargement…</p>
    {:else if invites.length === 0}
        <p class="empty">Aucune invitation générée.</p>
    {:else}
        <div class="invite-list">
            {#each invites as inv (inv.id)}
                {@const used = isUsed(inv)}
                {@const expired = !used && isExpired(inv)}
                <div class="invite-row" class:used class:expired>
                    <div class="invite-main">
                        <code class="invite-code">{inv.code}</code>
                        {#if inv.note}<span class="invite-note">{inv.note}</span>{/if}
                        <span class="invite-status">
                            {#if used}
                                <span class="chip used">Utilisé par {inv.usedBy}</span>
                            {:else if expired}
                                <span class="chip expired">Expiré</span>
                            {:else}
                                <span class="chip active">Expire le {fmt(inv.expiresAt)}</span>
                            {/if}
                        </span>
                    </div>
                    <div class="invite-actions">
                        {#if !used && !expired}
                            <button class="btn-copy" onclick={() => copy(inv)}>
                                {copied === inv.id ? '✓ Copié' : 'Copier le lien'}
                            </button>
                        {/if}
                        <button
                            class="btn-del"
                            disabled={busy[inv.id]}
                            onclick={() => remove(inv.id)}
                        >✕</button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .invites-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .chip-active {
        font-size: 0.75rem; font-weight: 600;
        background: var(--success-bg); border: 1px solid var(--success-border);
        color: #3a9e6a; border-radius: 99px; padding: 2px 10px;
    }

    .generate-form {
        display: flex; gap: 0.5rem; flex-wrap: wrap;
        padding: 0.75rem; background: var(--muted);
        border: 1px solid var(--border); border-radius: var(--radius-md);
    }
    .generate-form input { flex: 1; min-width: 160px; }
    .btn-generate {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 8px 16px;
        font: 500 0.875rem/1 var(--font-sans); cursor: pointer; white-space: nowrap;
    }
    .btn-generate:disabled { opacity: 0.4; cursor: not-allowed; }

    .loading, .empty { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .invite-list { display: flex; flex-direction: column; gap: 0.5rem; }
    .invite-row {
        display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 0.5rem;
        padding: 0.75rem 1rem; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
    }
    .invite-row.used { opacity: 0.5; }
    .invite-row.expired { opacity: 0.4; }

    .invite-main { display: flex; align-items: center; gap: 0.625rem; flex-wrap: wrap; }
    .invite-code { font-family: monospace; font-size: 0.875rem; letter-spacing: 0.03em; }
    .invite-note { font-size: 0.8125rem; color: var(--muted-foreground); font-style: italic; }

    .chip { font-size: 0.6875rem; font-weight: 600; border-radius: 99px; padding: 2px 8px; }
    .chip.active  { background: var(--success-bg); color: #3a9e6a; border: 1px solid var(--success-border); }
    .chip.expired { background: var(--muted); color: var(--muted-foreground); }
    .chip.used    { background: var(--muted); color: var(--muted-foreground); }

    .invite-actions { display: flex; align-items: center; gap: 0.4rem; }
    .btn-copy {
        font: 500 0.8125rem/1 var(--font-sans);
        background: var(--muted); border: 1px solid var(--border);
        border-radius: var(--radius-sm); padding: 4px 10px; cursor: pointer;
        color: var(--foreground);
    }
    .btn-copy:hover { border-color: var(--primary); }
    .btn-del {
        background: none; border: none; color: var(--muted-foreground);
        cursor: pointer; font-size: 0.75rem; padding: 4px 8px;
        border-radius: var(--radius-sm);
    }
    .btn-del:hover { background: var(--destructive-bg); color: var(--destructive); }
    .btn-del:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
