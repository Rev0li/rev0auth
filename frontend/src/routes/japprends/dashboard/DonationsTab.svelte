<script lang="ts">
    type Donation = { id: number; pseudo: string; method: string; code: string; reviewed: boolean; approved: boolean; createdAt: number; };

    let donations = $state<Donation[]>([]);
    let loading = $state(true);
    let busy = $state<Record<number, boolean>>({});

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleDateString('fr-FR', {
            day: '2-digit', month: 'short', year: 'numeric'
        });
    }

    async function load() {
        loading = true;
        try {
            const r = await fetch('/japprends/donations');
            donations = await r.json();
        } finally { loading = false; }
    }

    async function review(id: number, approved: boolean) {
        busy[id] = true;
        try {
            await fetch(`/japprends/donations/${id}/review`, {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ approved }),
            });
            donations = donations.map(d => d.id === id ? { ...d, reviewed: true, approved } : d);
        } finally { busy[id] = false; }
    }

    $effect(() => {
        load();
        const id = setInterval(load, 30_000);
        return () => clearInterval(id);
    });
</script>

<div class="donations-tab">
    <div class="tab-header">
        <h2>Donations</h2>
        {#if donations.filter(d => !d.reviewed).length > 0}
            <span class="badge-alert">{donations.filter(d => !d.reviewed).length} en attente</span>
        {/if}
    </div>

    {#if loading}
        <p class="loading">Chargement…</p>
    {:else if donations.length === 0}
        <p class="empty">Aucune donation enregistrée.</p>
    {:else}
        <div class="donation-list">
            {#each donations as d (d.id)}
                <div class="donation-row" class:pending={!d.reviewed}>
                    <div class="donation-info">
                        <span class="pseudo">{d.pseudo}</span>
                        <span class="method">{d.method}</span>
                        {#if d.code}<span class="code">{d.code}</span>{/if}
                        <span class="date">{fmt(d.createdAt)}</span>
                    </div>
                    <div class="donation-status">
                        {#if d.reviewed}
                            <span class="chip {d.approved ? 'approved' : 'rejected'}">
                                {d.approved ? '✓ Approuvé' : '✕ Refusé'}
                            </span>
                        {:else}
                            <button
                                class="btn-action grant"
                                disabled={busy[d.id]}
                                onclick={() => review(d.id, true)}
                            >Approuver</button>
                            <button
                                class="btn-action danger"
                                disabled={busy[d.id]}
                                onclick={() => review(d.id, false)}
                            >Refuser</button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .donations-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .badge-alert {
        font-size: 0.75rem; font-weight: 600;
        background: var(--destructive-bg); border: 1px solid var(--destructive-border);
        color: var(--destructive); border-radius: 99px; padding: 2px 10px;
    }

    .loading, .empty { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .donation-list { display: flex; flex-direction: column; gap: 0.5rem; }
    .donation-row {
        display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 0.5rem;
        padding: 0.75rem 1rem; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
    }
    .donation-row.pending { border-color: rgba(255,180,50,0.4); background: rgba(255,180,50,0.04); }

    .donation-info { display: flex; align-items: center; gap: 0.625rem; flex-wrap: wrap; }
    .pseudo { font-weight: 600; font-size: 0.9rem; }
    .method {
        font-size: 0.75rem; font-weight: 600; text-transform: uppercase;
        background: var(--muted); border-radius: 99px; padding: 2px 8px;
        color: var(--muted-foreground);
    }
    .code { font-family: monospace; font-size: 0.8125rem; color: var(--muted-foreground); }
    .date { font-size: 0.75rem; color: var(--muted-foreground); }

    .donation-status { display: flex; align-items: center; gap: 0.4rem; }
    .chip { font-size: 0.75rem; font-weight: 600; border-radius: 99px; padding: 3px 10px; }
    .chip.approved { background: var(--success-bg); color: #3a9e6a; border: 1px solid var(--success-border); }
    .chip.rejected { background: var(--destructive-bg); color: var(--destructive); border: 1px solid var(--destructive-border); }

    .btn-action {
        font: 500 0.8125rem/1 var(--font-sans); border: 1px solid transparent;
        border-radius: var(--radius-sm); padding: 4px 10px; cursor: pointer;
    }
    .btn-action:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn-action.grant  { background: var(--success-bg); border-color: var(--success-border); color: #3a9e6a; }
    .btn-action.danger { background: var(--destructive-bg); border-color: var(--destructive-border); color: var(--destructive); }
</style>
