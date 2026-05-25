<script lang="ts">
    import type { PageData } from './$types.js';
    import { goto, invalidateAll } from '$app/navigation';
    import { slide, fade } from 'svelte/transition';

    let { data }: { data: PageData } = $props();

    type Tab = 'membres' | 'demandes' | 'messages' | 'donations' | 'tests' | 'statut' | 'audit';
    let tab = $state<Tab>('membres');

    // ── Membres ──────────────────────────────────────────────────────
    let users       = $state([...data.users]);
    let search      = $state('');
    let showCreate  = $state(false);
    let newPseudo   = $state('');
    let newPassword = $state('');
    let newRole     = $state<'member' | 'mod' | 'admin'>('member');
    let createError = $state('');
    let createLoading = $state(false);

    let filtered = $derived(
        search ? users.filter(u => u.pseudo.includes(search.toLowerCase())) : users
    );

    async function createUser() {
        createError = '';
        if (!newPseudo.trim() || !newPassword.trim()) { createError = 'Champs requis.'; return; }
        createLoading = true;
        try {
            const r = await fetch('/japprends/users', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: newPseudo.trim(), password: newPassword, role: newRole }),
            });
            const d = await r.json();
            if (!d.ok) { createError = d.message; return; }
            showCreate = false; newPseudo = ''; newPassword = ''; newRole = 'member';
            await invalidateAll();
            users = [...data.users];
        } finally { createLoading = false; }
    }

    async function toggleActive(pseudo: string, active: boolean) {
        await fetch(`/japprends/users/${pseudo}`, {
            method: 'PUT',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ active }),
        });
        users = users.map(u => u.pseudo === pseudo ? { ...u, active } : u);
    }

    async function changeRole(pseudo: string, role: string) {
        await fetch(`/japprends/users/${pseudo}`, {
            method: 'PUT',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ role }),
        });
        users = users.map(u => u.pseudo === pseudo ? { ...u, role: role as typeof u.role } : u);
    }

    async function deleteUser(pseudo: string) {
        if (!confirm(`Supprimer ${pseudo} ? Cette action est irréversible.`)) return;
        await fetch(`/japprends/users/${pseudo}`, { method: 'DELETE' });
        users = users.filter(u => u.pseudo !== pseudo);
    }

    async function resetPassword(pseudo: string) {
        const pwd = prompt(`Nouveau mot de passe pour ${pseudo} :`);
        if (!pwd) return;
        await fetch(`/japprends/users/${pseudo}/password`, {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ password: pwd }),
        });
    }

    // ── Invitations ──────────────────────────────────────────────────
    let pending = $state([...data.pending]);

    async function createInvite() {
        const note = prompt('Note pour cette invitation (optionnel) :') ?? '';
        const r = await fetch('/japprends/signup-requests', {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ note, ttlDays: 7 }),
        });
        if (r.ok) await invalidateAll();
    }

    async function revokeInvite(id: number) {
        await fetch(`/japprends/signup-requests/${id}/reject`, { method: 'POST' });
        pending = pending.filter(p => p.id !== id);
    }

    // ── Messages ─────────────────────────────────────────────────────
    let msgs        = $state([...data.messages]);
    let replyTo     = $state('');
    let replyBody   = $state('');
    let replyLoading = $state(false);

    async function sendReply() {
        if (!replyTo || !replyBody.trim()) return;
        replyLoading = true;
        try {
            await fetch('/japprends/messages/reply', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ to: replyTo, body: replyBody }),
            });
            replyBody = '';
        } finally { replyLoading = false; }
    }

    // Thread view: group by conversation pair
    let threads = $derived(
        Object.values(
            msgs.reduce((acc, m) => {
                const key = [m.fromPseudo, m.toPseudo].sort().join('↔');
                if (!acc[key]) acc[key] = { key, participants: [m.fromPseudo, m.toPseudo].sort(), msgs: [] };
                acc[key].msgs.push(m);
                return acc;
            }, {} as Record<string, { key: string; participants: string[]; msgs: typeof msgs }>)
        ).sort((a, b) => {
            const aLast = a.msgs[0]?.createdAt ?? 0;
            const bLast = b.msgs[0]?.createdAt ?? 0;
            return (bLast as number) - (aLast as number);
        })
    );

    let activeThread = $state<string | null>(null);
    let threadMsgs   = $derived(activeThread ? threads.find(t => t.key === activeThread)?.msgs ?? [] : []);

    // ── Donations ────────────────────────────────────────────────────
    let donas     = $state([...data.donations]);

    async function reviewDona(id: number, approved: boolean) {
        await fetch(`/japprends/donations/${id}/review`, {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify({ approved }),
        });
        donas = donas.map(d => d.id === id ? { ...d, reviewed: true, approved } : d);
    }

    // ── Tests ────────────────────────────────────────────────────────
    let runs       = $state([...data.testRuns]);
    let testLoading = $state(false);
    let latestRun  = $state<typeof runs[0] | null>(runs[0] ?? null);

    async function launchTests() {
        testLoading = true;
        try {
            const r = await fetch('/japprends/tests/launch', { method: 'POST' });
            if (r.ok) {
                const d = await r.json();
                const run = { ...d, executedAt: new Date() };
                latestRun = run;
                runs = [run, ...runs];
            }
        } finally { testLoading = false; }
    }

    // ── Logout ───────────────────────────────────────────────────────
    async function logout() {
        await fetch('/japprends/logout', { method: 'POST' });
        goto('/');
    }

    function timeAgo(date: Date | string | number | null) {
        if (!date) return '–';
        const d = typeof date === 'number' ? new Date(date * 1000) : new Date(date);
        const sec = Math.floor((Date.now() - d.getTime()) / 1000);
        if (sec < 60) return 'à l\'instant';
        if (sec < 3600) return `il y a ${Math.floor(sec / 60)} min`;
        if (sec < 86400) return `il y a ${Math.floor(sec / 3600)} h`;
        return d.toLocaleDateString('fr-FR');
    }

    const ROLE_COLORS: Record<string, string> = {
        admin: '#D96B6B', mod: '#D9A84C', member: '#65B48A', guest: '#8A8A8A'
    };
</script>

<!-- ── Navbar ── -->
<nav class="navbar">
    <span class="navbar-brand">rev0auth <span class="admin-badge">admin</span></span>
    <div class="navbar-right">
        <span class="meta">session admin</span>
        <button class="nav-btn nav-btn-logout" onclick={logout}>Déconnexion</button>
    </div>
</nav>

<!-- ── Tab bar ── -->
<div class="tabbar">
    {#each [
        { id: 'membres',   label: `Membres (${users.length})` },
        { id: 'demandes',  label: `Demandes ${pending.length > 0 ? `(${pending.length})` : ''}`, badge: pending.length > 0 },
        { id: 'messages',  label: 'Messages' },
        { id: 'donations', label: `Donations ${donas.filter(d => !d.reviewed).length > 0 ? `(${donas.filter(d => !d.reviewed).length})` : ''}` },
        { id: 'tests',     label: 'Tests' },
        { id: 'statut',    label: 'Statut' },
        { id: 'audit',     label: 'Audit' },
    ] as t}
        <button
            class="tabbar-btn"
            class:active={tab === t.id}
            class:badge={t.badge}
            onclick={() => tab = t.id as Tab}
        >{t.label}</button>
    {/each}
</div>

<div class="dash-content">

    <!-- ══ MEMBRES ══ -->
    {#if tab === 'membres'}
        <div transition:fade={{ duration: 120 }}>
            <div class="toolbar">
                <input class="search-input" bind:value={search} placeholder="Rechercher un pseudo…" />
                <button class="btn-primary" onclick={() => showCreate = !showCreate}>+ Créer</button>
            </div>

            {#if showCreate}
                <div class="create-card card" transition:slide={{ duration: 200 }}>
                    <h3>Nouveau membre</h3>
                    <div class="create-grid">
                        <div class="field">
                            <label for="c-pseudo">Pseudo</label>
                            <input id="c-pseudo" bind:value={newPseudo} placeholder="pseudo" />
                        </div>
                        <div class="field">
                            <label for="c-pwd">Mot de passe initial</label>
                            <input id="c-pwd" type="password" bind:value={newPassword} placeholder="••••••••" />
                        </div>
                        <div class="field">
                            <label for="c-role">Rôle</label>
                            <select id="c-role" bind:value={newRole}>
                                <option value="member">member</option>
                                <option value="mod">mod</option>
                                <option value="admin">admin</option>
                            </select>
                        </div>
                    </div>
                    {#if createError}<p class="chip-error">{createError}</p>{/if}
                    <div class="row-btns">
                        <button class="btn-primary" onclick={createUser} disabled={createLoading}>
                            {createLoading ? '…' : 'Créer'}
                        </button>
                        <button class="btn-secondary" onclick={() => showCreate = false}>Annuler</button>
                    </div>
                </div>
            {/if}

            <div class="user-table">
                <div class="table-header">
                    <span>Pseudo</span><span>Rôle</span><span>Statut</span><span>Actif</span><span>Créé</span><span>Actions</span>
                </div>
                {#each filtered as u (u.pseudo)}
                    <div class="table-row" transition:slide={{ duration: 150 }}>
                        <span class="user-pseudo">{u.pseudo}</span>
                        <span>
                            <select
                                class="role-select"
                                style="color:{ROLE_COLORS[u.role]}"
                                value={u.role}
                                onchange={(e) => changeRole(u.pseudo, (e.target as HTMLSelectElement).value)}
                            >
                                {#each ['guest','member','mod','admin'] as r}
                                    <option value={r}>{r}</option>
                                {/each}
                            </select>
                        </span>
                        <span class="meta">{u.status}</span>
                        <span>
                            <button
                                class="toggle-btn"
                                class:active={u.active}
                                onclick={() => toggleActive(u.pseudo, !u.active)}
                                title={u.active ? 'Désactiver' : 'Activer'}
                            >{u.active ? '✓' : '✗'}</button>
                        </span>
                        <span class="meta">{timeAgo(u.createdAt)}</span>
                        <span class="action-row">
                            <button class="btn-secondary" onclick={() => resetPassword(u.pseudo)} title="Réinitialiser le mot de passe">🔑</button>
                            <button class="btn-secondary revoke" onclick={() => deleteUser(u.pseudo)} title="Supprimer">🗑</button>
                        </span>
                    </div>
                {:else}
                    <p class="empty-state">Aucun membre trouvé.</p>
                {/each}
            </div>
        </div>
    {/if}

    <!-- ══ INVITATIONS ══ -->
    {#if tab === 'demandes'}
        <div transition:fade={{ duration: 120 }}>
            <div style="margin-bottom:1rem">
                <button class="btn-secondary" onclick={createInvite}>+ Créer une invitation</button>
            </div>
            {#if pending.length === 0}
                <p class="empty-state">Aucune invitation active.</p>
            {:else}
                <div class="requests-list">
                    {#each pending as inv (inv.id)}
                        <div class="req-card card" transition:slide={{ duration: 200 }}>
                            <div class="req-header">
                                <code style="font-size:0.8rem">{inv.code}</code>
                                {#if inv.note}<span class="meta">{inv.note}</span>{/if}
                                <span class="meta" style="margin-left:auto">expire {timeAgo(inv.expiresAt)}</span>
                            </div>
                            <div class="req-actions">
                                <button class="btn-secondary revoke" onclick={() => revokeInvite(inv.id)}>✗ Révoquer</button>
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    <!-- ══ MESSAGES ══ -->
    {#if tab === 'messages'}
        <div class="msg-layout" transition:fade={{ duration: 120 }}>
            <!-- Thread list -->
            <div class="thread-list">
                {#each threads as t}
                    <button
                        class="thread-item"
                        class:active={activeThread === t.key}
                        onclick={() => { activeThread = t.key; replyTo = t.participants.find(p => p !== 'admin') ?? ''; }}
                    >
                        <span class="thread-names">{t.participants.join(' ↔ ')}</span>
                        <span class="meta">{t.msgs.length} msg</span>
                    </button>
                {:else}
                    <p class="empty-state">Aucun message.</p>
                {/each}
            </div>

            <!-- Thread detail -->
            <div class="thread-detail">
                {#if activeThread}
                    <div class="msg-list">
                        {#each [...threadMsgs].reverse() as m (m.id)}
                            <div class="msg-bubble" class:mine={m.fromPseudo === 'admin'}>
                                <span class="msg-author">{m.fromPseudo}</span>
                                <p class="msg-body">{m.body}</p>
                                <span class="msg-time meta">{timeAgo(m.createdAt)}</span>
                            </div>
                        {/each}
                    </div>
                    <div class="reply-box">
                        <textarea bind:value={replyBody} placeholder="Répondre à {replyTo}…" rows={2}></textarea>
                        <button class="btn-primary" onclick={sendReply} disabled={replyLoading}>
                            {replyLoading ? '…' : 'Envoyer'}
                        </button>
                    </div>
                {:else}
                    <p class="empty-state" style="padding:3rem">Sélectionne une conversation.</p>
                {/if}
            </div>
        </div>
    {/if}

    <!-- ══ DONATIONS ══ -->
    {#if tab === 'donations'}
        <div transition:fade={{ duration: 120 }}>
            {#each donas as d (d.id)}
                <div class="dona-row" transition:slide={{ duration: 150 }}>
                    <div class="dona-info">
                        <strong>{d.pseudo}</strong>
                        <span class="meta">{d.method.toUpperCase()}</span>
                        <code class="dona-code">{d.code}</code>
                        <span class="meta">{timeAgo(d.createdAt)}</span>
                    </div>
                    {#if d.reviewed}
                        <span class={d.approved ? 'chip-ok' : 'chip-error'}>{d.approved ? '✓ Approuvé' : '✗ Rejeté'}</span>
                    {:else}
                        <div class="dona-actions">
                            <button class="btn-secondary grant"  onclick={() => reviewDona(d.id, true)}>✓ Valider</button>
                            <button class="btn-secondary revoke" onclick={() => reviewDona(d.id, false)}>✗ Rejeter</button>
                        </div>
                    {/if}
                </div>
            {:else}
                <p class="empty-state">Aucune donation.</p>
            {/each}
        </div>
    {/if}

    <!-- ══ TESTS ══ -->
    {#if tab === 'tests'}
        <div transition:fade={{ duration: 120 }}>
            <div class="toolbar">
                <button class="btn-primary" onclick={launchTests} disabled={testLoading}>
                    {testLoading ? '⏳ Tests en cours…' : '▶ Lancer les tests'}
                </button>
                {#if latestRun}
                    <span class="test-summary" class:pass={latestRun.passed === latestRun.total}>
                        {latestRun.passed}/{latestRun.total} ✓
                    </span>
                {/if}
            </div>

            {#if latestRun}
                <div class="test-cases">
                    {#each latestRun.cases as c}
                        <div class="test-case" class:ok={c.ok} class:fail={!c.ok}>
                            <span>{c.ok ? '✓' : '✗'}</span>
                            <span class="case-name">{c.name}</span>
                        </div>
                    {:else}
                        <p class="meta">Aucun cas de test.</p>
                    {/each}
                </div>
            {/if}

            {#if runs.length > 1}
                <h3 class="section-heading" style="margin-top:2rem">Historique</h3>
                <div class="run-history">
                    {#each runs.slice(1) as r (r.runId)}
                        <div class="run-row" class:pass={r.passed === r.total}>
                            <span>{r.passed}/{r.total}</span>
                            <span class="meta">{timeAgo(r.executedAt)}</span>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    {/if}

    <!-- ══ STATUT ══ -->
    {#if tab === 'statut'}
        <div class="statut-grid" transition:fade={{ duration: 120 }}>
            <div class="card stat-card">
                <span class="stat-label">Base de données</span>
                <span class={data.dbOk ? 'chip-ok' : 'chip-error'}>{data.dbOk ? '✓ OK' : '✗ Down'}</span>
            </div>
            <div class="card stat-card">
                <span class="stat-label">Membres</span>
                <span class="stat-value">{data.users.length}</span>
            </div>
            <div class="card stat-card">
                <span class="stat-label">Demandes en attente</span>
                <span class="stat-value">{data.pending.length}</span>
            </div>
            <div class="card stat-card">
                <span class="stat-label">Messages</span>
                <span class="stat-value">{data.messages.length}</span>
            </div>
            <div class="card stat-card">
                <span class="stat-label">Donations non reviewées</span>
                <span class="stat-value">{data.donations.filter(d => !d.reviewed).length}</span>
            </div>
            <div class="card stat-card">
                <span class="stat-label">Dernier test</span>
                <span class="stat-value">
                    {data.testRuns[0] ? `${data.testRuns[0].passed}/${data.testRuns[0].total}` : '–'}
                </span>
            </div>
        </div>
    {/if}

    <!-- ══ AUDIT ══ -->
    {#if tab === 'audit'}
        <div transition:fade={{ duration: 120 }}>
            <div class="audit-list">
                <p class="empty-state">Audit log non disponible.</p>
            </div>
        </div>
    {/if}

</div>

<style>
    /* ── Navbar ── */
    .navbar {
        position: sticky; top: 0; z-index: 100;
        display: flex; align-items: center; justify-content: space-between;
        padding: 0 1.5rem; height: 52px;
        background: var(--card); border-bottom: 1px solid var(--border);
        box-shadow: var(--shadow-soft);
    }
    .navbar-brand { font-weight: 700; font-size: 1rem; display: flex; align-items: center; gap: 8px; }
    .admin-badge {
        font-size: 0.6875rem; font-weight: 700;
        background: var(--accent); color: var(--accent-foreground);
        border-radius: var(--radius-full); padding: 2px 8px;
        text-transform: uppercase; letter-spacing: 0.05em;
    }
    .navbar-right { display: flex; align-items: center; gap: 10px; }
    .nav-btn {
        height: 30px; padding: 0 12px;
        border: 1px solid var(--border); border-radius: var(--radius-full);
        background: transparent; font: 500 0.8125rem/1 var(--font-sans);
        color: var(--foreground); cursor: pointer; transition: background 0.15s;
    }
    .nav-btn:hover { background: var(--muted); }
    .nav-btn-logout { color: var(--destructive); border-color: var(--destructive-border); }

    /* ── Tab bar ── */
    .tabbar {
        display: flex; gap: 2px;
        padding: 0 1.5rem;
        border-bottom: 1px solid var(--border);
        background: var(--card);
        overflow-x: auto;
    }
    .tabbar-btn {
        position: relative;
        height: 44px; padding: 0 14px;
        background: none; border: none;
        font: 500 0.875rem/1 var(--font-sans);
        color: var(--muted-foreground);
        cursor: pointer; white-space: nowrap;
        border-bottom: 2px solid transparent;
        transition: color 0.15s, border-color 0.15s;
    }
    .tabbar-btn.active { color: var(--foreground); border-bottom-color: var(--primary); }
    .tabbar-btn.badge::after {
        content: ''; position: absolute; top: 10px; right: 6px;
        width: 6px; height: 6px; border-radius: 50%;
        background: var(--destructive);
    }

    /* ── Content ── */
    .dash-content { padding: 1.5rem; max-width: 1100px; margin: 0 auto; }

    /* ── Toolbar ── */
    .toolbar { display: flex; gap: 10px; align-items: center; margin-bottom: 1rem; }
    .search-input { flex: 1; max-width: 300px; height: 36px; padding: 0 12px; width: auto; }

    /* ── Create card ── */
    .create-card { padding: 1.25rem; margin-bottom: 1rem; }
    .create-card h3 { margin: 0 0 1rem; font-size: 0.9375rem; }
    .create-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 10px; margin-bottom: 10px; }
    .row-btns { display: flex; gap: 8px; }

    /* ── User table ── */
    .user-table { display: flex; flex-direction: column; gap: 2px; }
    .table-header {
        display: grid; grid-template-columns: 160px 130px 80px 60px 100px 1fr;
        gap: 8px; padding: 8px 12px;
        font-size: 0.75rem; font-weight: 700; text-transform: uppercase;
        color: var(--muted-foreground); letter-spacing: 0.04em;
    }
    .table-row {
        display: grid; grid-template-columns: 160px 130px 80px 60px 100px 1fr;
        gap: 8px; padding: 10px 12px; align-items: center;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--card);
    }
    .table-row:hover { background: var(--muted); }
    .user-pseudo { font-weight: 600; font-size: 0.875rem; }
    .role-select {
        border: none; background: none; font: 600 0.8125rem/1 var(--font-sans);
        cursor: pointer; padding: 0; width: auto;
    }
    .toggle-btn {
        width: 28px; height: 28px; border-radius: 50%;
        border: 1px solid var(--border); background: var(--muted);
        cursor: pointer; font-size: 0.875rem;
        transition: background 0.15s, border-color 0.15s;
    }
    .toggle-btn.active { background: var(--success-bg); border-color: var(--success-border); color: var(--success); }
    .action-row { display: flex; gap: 4px; }

    /* ── Requests ── */
    .requests-list { display: flex; flex-direction: column; gap: 10px; }
    .req-card { padding: 1rem 1.25rem; }
    .req-header { display: flex; align-items: center; gap: 10px; margin-bottom: 10px; flex-wrap: wrap; }
    .req-actions { display: flex; gap: 8px; align-items: flex-end; flex-wrap: wrap; }
    .req-actions .field { flex: 1; min-width: 200px; margin: 0; }

    /* ── Messages ── */
    .msg-layout { display: grid; grid-template-columns: 240px 1fr; gap: 1rem; height: calc(100vh - 200px); }
    .thread-list { display: flex; flex-direction: column; gap: 2px; overflow-y: auto; }
    .thread-item {
        display: flex; flex-direction: column; gap: 2px;
        padding: 10px 12px; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
        cursor: pointer; text-align: left; font-size: 0.875rem;
        transition: background 0.12s;
    }
    .thread-item.active { background: color-mix(in srgb, var(--primary) 12%, transparent); border-color: var(--primary); }
    .thread-item:hover:not(.active) { background: var(--muted); }
    .thread-names { font-weight: 600; font-size: 0.8125rem; }
    .thread-detail { display: flex; flex-direction: column; border: 1px solid var(--border); border-radius: var(--radius-lg); overflow: hidden; background: var(--card); }
    .msg-list { flex: 1; overflow-y: auto; padding: 1rem; display: flex; flex-direction: column; gap: 8px; }
    .msg-bubble { max-width: 70%; padding: 8px 12px; border-radius: var(--radius-lg); background: var(--muted); }
    .msg-bubble.mine { align-self: flex-end; background: color-mix(in srgb, var(--primary) 20%, transparent); }
    .msg-author { font-size: 0.75rem; font-weight: 600; display: block; margin-bottom: 2px; }
    .msg-body { margin: 0; font-size: 0.875rem; }
    .msg-time { display: block; text-align: right; margin-top: 4px; font-size: 0.75rem; }
    .reply-box { display: flex; gap: 8px; padding: 12px; border-top: 1px solid var(--border); align-items: flex-end; }
    .reply-box textarea { flex: 1; min-height: 50px; resize: none; }

    /* ── Donations ── */
    .dona-row {
        display: flex; align-items: center; gap: 12px; flex-wrap: wrap;
        padding: 12px 14px; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card); margin-bottom: 6px;
    }
    .dona-info { display: flex; align-items: center; gap: 10px; flex: 1; flex-wrap: wrap; }
    .dona-code { font-family: var(--font-mono); font-size: 0.8125rem; background: var(--muted); padding: 2px 6px; border-radius: 4px; }
    .dona-actions { display: flex; gap: 6px; }

    /* ── Tests ── */
    .test-summary { font-size: 1rem; font-weight: 700; }
    .test-summary.pass { color: var(--success); }
    .test-cases { display: flex; flex-direction: column; gap: 3px; margin-top: 1rem; }
    .test-case {
        display: flex; align-items: center; gap: 10px;
        padding: 6px 12px; border-radius: var(--radius-sm);
        font-size: 0.8125rem;
    }
    .test-case.ok { background: var(--success-bg); color: var(--success); }
    .test-case.fail { background: var(--destructive-bg); color: var(--destructive); }
    .case-name { font-family: var(--font-mono); }
    .run-history { display: flex; flex-direction: column; gap: 4px; }
    .run-row { display: flex; gap: 12px; padding: 6px 12px; border-radius: var(--radius-sm); background: var(--muted); font-size: 0.875rem; }
    .run-row.pass { color: var(--success); }

    /* ── Statut ── */
    .statut-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1rem; }
    .stat-card { padding: 1.25rem; display: flex; flex-direction: column; gap: 8px; }
    .stat-label { font-size: 0.8125rem; font-weight: 600; color: var(--muted-foreground); text-transform: uppercase; letter-spacing: 0.04em; }
    .stat-value { font-size: 1.5rem; font-weight: 700; }

    /* ── Audit ── */
    .audit-list { display: flex; flex-direction: column; gap: 4px; }
    .audit-row {
        display: flex; align-items: center; gap: 12px; flex-wrap: wrap;
        padding: 8px 12px; border-radius: var(--radius-sm);
        font-size: 0.8125rem; background: var(--card);
        border: 1px solid var(--border);
    }
    .audit-action { font-weight: 600; font-family: var(--font-mono); }
    .audit-target { }
    .audit-detail { margin-left: auto; }

    /* ── Shared ── */
    .empty-state { color: var(--muted-foreground); text-align: center; padding: 3rem 0; }
    .section-heading { font-size: 0.8125rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--muted-foreground); }
</style>
