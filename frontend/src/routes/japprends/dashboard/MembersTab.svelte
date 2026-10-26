<script lang="ts">
    type User = {
        pseudo: string; role: string; active: boolean; approved: boolean;
        createdAt: number;
    };

    let { users: initial }: { users: User[] } = $props();

    let members = $state<User[]>(initial);
    let busy = $state<Record<string, boolean>>({});
    let expanded = $state<string | null>(null);

    // Create form
    let createOpen = $state(false);
    let newPseudo = $state('');
    let newPassword = $state('');
    let createErr = $state('');
    let createLoading = $state(false);

    // Reset password (per member)
    let resetPseudo = $state<string | null>(null);
    let newPwd = $state('');
    let resetLoading = $state(false);
    let resetMsg = $state('');

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' });
    }

    function toggle(pseudo: string) {
        expanded = expanded === pseudo ? null : pseudo;
        resetPseudo = null; resetMsg = ''; newPwd = '';
    }

    async function patch(pseudo: string, updates: Record<string, unknown>) {
        busy[pseudo] = true;
        try {
            const r = await fetch(`/japprends/users/${pseudo}`, {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(updates),
            });
            if (r.ok) members = members.map(u => u.pseudo === pseudo ? { ...u, ...updates } : u);
        } finally { busy[pseudo] = false; }
    }

    async function deleteMember(pseudo: string) {
        if (!confirm(`Supprimer ${pseudo} ? Cette action est irréversible.`)) return;
        busy[pseudo] = true;
        try {
            await fetch(`/japprends/users/${pseudo}`, { method: 'DELETE' });
            members = members.filter(u => u.pseudo !== pseudo);
            if (expanded === pseudo) expanded = null;
        } finally { busy[pseudo] = false; }
    }

    async function resetPassword() {
        if (!resetPseudo || !newPwd) return;
        resetLoading = true; resetMsg = '';
        try {
            const r = await fetch(`/japprends/users/${resetPseudo}/password`, {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ password: newPwd }),
            });
            const d = await r.json();
            resetMsg = d.ok ? '✓ Mot de passe modifié' : (d.message ?? 'Erreur');
            if (d.ok) { newPwd = ''; resetPseudo = null; }
        } finally { resetLoading = false; }
    }

    async function createMember() {
        createErr = '';
        const key = newPseudo.trim().toLowerCase();
        if (!key || !newPassword) { createErr = 'Pseudo et mot de passe requis.'; return; }
        createLoading = true;
        try {
            const r = await fetch('/japprends/users', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: key, password: newPassword, role: 'member' }),
            });
            const data = await r.json();
            if (!data.ok) { createErr = data.message ?? 'Erreur.'; return; }
            members = [...members, {
                pseudo: key, role: 'member', active: true, approved: false,
                createdAt: Math.floor(Date.now() / 1000),
            }];
            newPseudo = ''; newPassword = ''; createOpen = false;
        } finally { createLoading = false; }
    }
</script>

<div class="members-tab">

    <div class="tab-header">
        <h2>Membres <span class="count">{members.length}</span></h2>
        <button class="btn-create" onclick={() => { createOpen = !createOpen; }}>
            {createOpen ? '✕ Annuler' : '+ Nouveau membre'}
        </button>
    </div>

    {#if createOpen}
        <div class="create-form">
            <input bind:value={newPseudo} placeholder="pseudo" autocomplete="off" />
            <input bind:value={newPassword} type="password" placeholder="mot de passe" />
            <button class="btn-action grant" onclick={createMember} disabled={createLoading}>
                {createLoading ? '…' : 'Créer'}
            </button>
            {#if createErr}<span class="err">{createErr}</span>{/if}
        </div>
    {/if}

    <div class="member-list">
        {#each members as u (u.pseudo)}
            {@const isExpanded = expanded === u.pseudo}
            <div class="member-card" class:open={isExpanded}>

                <!-- ── Row cliquable ── -->
                <button class="member-row" onclick={() => toggle(u.pseudo)}>
                    <div class="member-identity">
                        <span class="pseudo">{u.pseudo}</span>
                    </div>
                    <span class="meta-date">{fmt(u.createdAt)}</span>
                    <span class="chevron">{isExpanded ? '▲' : '▼'}</span>
                </button>

                <!-- ── Détail expandable ── -->
                {#if isExpanded}
                    <div class="member-detail">

                        <!-- Reset mot de passe -->
                        <div class="detail-section">
                            <span class="detail-label">Mot de passe</span>
                            {#if resetPseudo === u.pseudo}
                                <div class="inline-form">
                                    <input type="password" bind:value={newPwd} placeholder="Nouveau mot de passe" />
                                    <button class="btn-action grant" onclick={resetPassword} disabled={resetLoading || !newPwd}>
                                        {resetLoading ? '…' : 'Valider'}
                                    </button>
                                    <button class="btn-action" onclick={() => { resetPseudo = null; newPwd = ''; }}>Annuler</button>
                                </div>
                                {#if resetMsg}<span class="feedback">{resetMsg}</span>{/if}
                            {:else}
                                <button class="btn-action" onclick={() => { resetPseudo = u.pseudo; resetMsg = ''; }}>
                                    Réinitialiser
                                </button>
                            {/if}
                        </div>

                        <!-- Danger zone -->
                        <div class="detail-section danger-zone">
                            <button
                                class="btn-action danger"
                                disabled={busy[u.pseudo]}
                                onclick={() => deleteMember(u.pseudo)}
                            >
                                Supprimer le compte
                            </button>
                        </div>

                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .members-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header { display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .count {
        display: inline-flex; align-items: center; justify-content: center;
        background: var(--muted); border-radius: 99px;
        font-size: 0.75rem; font-weight: 500; padding: 1px 7px;
        color: var(--muted-foreground);
    }
    .btn-create {
        margin-left: auto; background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 6px 14px;
        font: 500 0.8125rem/1 var(--font-sans); cursor: pointer;
    }
    .btn-create:hover { background: var(--primary-hover); }

    .create-form {
        display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: flex-end;
        padding: 1rem; background: var(--muted); border-radius: var(--radius-md);
        border: 1px solid var(--border);
    }
    .create-form input { flex: 1; min-width: 140px; }
    .err { font-size: 0.8125rem; color: var(--destructive); }

    .member-list { display: flex; flex-direction: column; gap: 0.5rem; }

    .member-card {
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--card); overflow: hidden;
        transition: border-color 0.15s;
    }
    .member-card.open { border-color: var(--primary); }

    .member-row {
        width: 100%; display: flex; align-items: center; gap: 0.75rem;
        padding: 0.75rem 1rem; background: none; border: none;
        cursor: pointer; text-align: left;
    }
    .member-row:hover { background: var(--muted); }

    .member-identity { display: flex; align-items: center; gap: 0.5rem; flex: 1; }
    .pseudo { font-weight: 600; font-size: 0.9375rem; }
    .meta-date { font-size: 0.8125rem; color: var(--muted-foreground); margin-left: auto; }
    .chevron { font-size: 0.6875rem; color: var(--muted-foreground); flex-shrink: 0; }

    /* ── Detail panel ── */
    .member-detail {
        border-top: 1px solid var(--border);
        padding: 1rem; display: flex; flex-direction: column; gap: 1rem;
        background: var(--muted);
    }

    .detail-section { display: flex; flex-direction: column; gap: 0.5rem; }
    .detail-label { font-size: 0.75rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.05em; color: var(--muted-foreground); }




    .inline-form { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: flex-end; }
    .inline-form input { flex: 1; min-width: 180px; }

    .btn-action {
        font: 500 0.8125rem/1 var(--font-sans); border: 1px solid var(--border);
        background: var(--card); border-radius: var(--radius-sm);
        padding: 5px 12px; cursor: pointer; color: var(--foreground);
        white-space: nowrap;
    }
    .btn-action:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn-action.grant  { background: var(--success-bg); border-color: var(--success-border); color: #3a9e6a; }
    .btn-action.danger { background: var(--destructive-bg); border-color: var(--destructive-border); color: var(--destructive); }

    .feedback { font-size: 0.8125rem; color: var(--muted-foreground); }

    .danger-zone { padding-top: 0.5rem; border-top: 1px solid var(--border); }
</style>
