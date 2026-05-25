<script lang="ts">
    const ROLES = ['guest', 'member', 'mod', 'admin'];

    type User = {
        pseudo: string; role: string; active: boolean; approved: boolean;
        status: string; createdAt: number;
        accessGithub: boolean; accessJellyfin: boolean; accessSongsurf: boolean;
        requestGithub: boolean; requestJellyfin: boolean; requestSongsurf: boolean;
    };

    let { users: initial }: { users: User[] } = $props();

    let members = $state<User[]>(initial);
    let busy = $state<Record<string, boolean>>({});
    let createOpen = $state(false);
    let newPseudo = $state('');
    let newPassword = $state('');
    let newRole = $state('member');
    let createErr = $state('');
    let createLoading = $state(false);

    function fmt(epoch: number) {
        return new Date(epoch * 1000).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' });
    }

    async function patch(pseudo: string, updates: Record<string, unknown>) {
        busy[pseudo] = true;
        try {
            const r = await fetch(`/japprends/users/${pseudo}`, {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(updates),
            });
            if (r.ok) {
                members = members.map(u => u.pseudo === pseudo ? { ...u, ...updates } : u);
            }
        } finally { busy[pseudo] = false; }
    }

    async function deleteMember(pseudo: string) {
        if (!confirm(`Supprimer ${pseudo} ? Cette action est irréversible.`)) return;
        busy[pseudo] = true;
        try {
            await fetch(`/japprends/users/${pseudo}`, { method: 'DELETE' });
            members = members.filter(u => u.pseudo !== pseudo);
        } finally { busy[pseudo] = false; }
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
                body: JSON.stringify({ pseudo: key, password: newPassword, role: newRole }),
            });
            const data = await r.json();
            if (!data.ok) { createErr = data.message ?? 'Erreur.'; return; }
            members = [...members, {
                pseudo: key, role: newRole, active: true, approved: false,
                status: 'offline', createdAt: Math.floor(Date.now() / 1000),
                accessGithub: false, accessJellyfin: false, accessSongsurf: false,
                requestGithub: false, requestJellyfin: false, requestSongsurf: false,
            }];
            newPseudo = ''; newPassword = ''; newRole = 'member'; createOpen = false;
        } finally { createLoading = false; }
    }

    const pendingRequests = $derived(members.filter(u =>
        u.requestGithub || u.requestJellyfin || u.requestSongsurf
    ));
</script>

<div class="members-tab">

    <div class="tab-header">
        <h2>Membres <span class="count">{members.length}</span></h2>
        {#if pendingRequests.length > 0}
            <span class="badge-alert">{pendingRequests.length} demande{pendingRequests.length > 1 ? 's' : ''} d'accès</span>
        {/if}
        <button class="btn-create" onclick={() => { createOpen = !createOpen; }}>
            {createOpen ? '✕ Annuler' : '+ Nouveau membre'}
        </button>
    </div>

    {#if createOpen}
        <div class="create-form">
            <input bind:value={newPseudo} placeholder="pseudo" autocomplete="off" />
            <input bind:value={newPassword} type="password" placeholder="mot de passe" />
            <select bind:value={newRole}>
                {#each ROLES as r}<option value={r}>{r}</option>{/each}
            </select>
            <button class="btn-action grant" onclick={createMember} disabled={createLoading}>
                {createLoading ? '…' : 'Créer'}
            </button>
            {#if createErr}<span class="err">{createErr}</span>{/if}
        </div>
    {/if}

    <div class="member-list">
        {#each members as u (u.pseudo)}
            <div class="member-row" class:inactive={!u.active}>
                <div class="member-identity">
                    <span class="pseudo">{u.pseudo}</span>
                    <span class="role-badge role-{u.role}">{u.role}</span>
                    {#if u.requestGithub || u.requestJellyfin || u.requestSongsurf}
                        <span class="chip-request">demande accès</span>
                    {/if}
                </div>

                <div class="member-meta">
                    <span class="meta-date">{fmt(u.createdAt)}</span>
                </div>

                <div class="member-actions">
                    <select
                        value={u.role}
                        disabled={busy[u.pseudo]}
                        onchange={(e) => patch(u.pseudo, { role: (e.target as HTMLSelectElement).value })}
                    >
                        {#each ROLES as r}<option value={r}>{r}</option>{/each}
                    </select>

                    <button
                        class="btn-action {u.active ? 'revoke' : 'grant'}"
                        disabled={busy[u.pseudo]}
                        onclick={() => patch(u.pseudo, { active: !u.active })}
                    >
                        {u.active ? 'Désactiver' : 'Réactiver'}
                    </button>

                    <button
                        class="btn-action danger"
                        disabled={busy[u.pseudo]}
                        onclick={() => deleteMember(u.pseudo)}
                    >
                        ✕
                    </button>
                </div>

                {#if u.requestGithub || u.requestJellyfin || u.requestSongsurf}
                    <div class="access-requests">
                        {#if u.requestGithub}
                            <button
                                class="btn-access {u.accessGithub ? 'granted' : ''}"
                                onclick={() => patch(u.pseudo, { accessGithub: !u.accessGithub, requestGithub: false })}
                            >
                                GitHub {u.accessGithub ? '✓' : '?'}
                            </button>
                        {/if}
                        {#if u.requestJellyfin}
                            <button
                                class="btn-access {u.accessJellyfin ? 'granted' : ''}"
                                onclick={() => patch(u.pseudo, { accessJellyfin: !u.accessJellyfin, requestJellyfin: false })}
                            >
                                Jellyfin {u.accessJellyfin ? '✓' : '?'}
                            </button>
                        {/if}
                        {#if u.requestSongsurf}
                            <button
                                class="btn-access {u.accessSongsurf ? 'granted' : ''}"
                                onclick={() => patch(u.pseudo, { accessSongsurf: !u.accessSongsurf, requestSongsurf: false })}
                            >
                                SongSurf {u.accessSongsurf ? '✓' : '?'}
                            </button>
                        {/if}
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .members-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header {
        display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap;
    }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .count {
        display: inline-flex; align-items: center; justify-content: center;
        background: var(--muted); border-radius: 99px;
        font-size: 0.75rem; font-weight: 500; padding: 1px 7px;
        color: var(--muted-foreground);
    }
    .badge-alert {
        font-size: 0.75rem; font-weight: 600;
        background: var(--destructive-bg); border: 1px solid var(--destructive-border);
        color: var(--destructive); border-radius: 99px; padding: 2px 10px;
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
    .create-form input, .create-form select { flex: 1; min-width: 120px; }
    .err { font-size: 0.8125rem; color: var(--destructive); }

    .member-list { display: flex; flex-direction: column; gap: 0.5rem; }

    .member-row {
        display: flex; flex-direction: column; gap: 0.5rem;
        padding: 0.75rem 1rem; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
        transition: opacity 0.15s;
    }
    .member-row.inactive { opacity: 0.5; }

    .member-identity { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; }
    .pseudo { font-weight: 600; font-size: 0.9375rem; }

    .role-badge {
        font-size: 0.6875rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.04em; border-radius: 99px; padding: 2px 8px;
    }
    .role-admin  { background: rgba(160,120,255,0.15); color: #a078ff; }
    .role-mod    { background: rgba(90,180,255,0.15);  color: #5ab4ff; }
    .role-member { background: var(--muted); color: var(--muted-foreground); }
    .role-guest  { background: var(--muted); color: var(--muted-foreground); opacity: 0.6; }

    .chip-warn {
        font-size: 0.6875rem; padding: 2px 8px; border-radius: 99px;
        background: rgba(255,180,50,0.15); color: #c88a00; border: 1px solid rgba(255,180,50,0.3);
    }
    .chip-request {
        font-size: 0.6875rem; padding: 2px 8px; border-radius: 99px;
        background: var(--destructive-bg); color: var(--destructive);
        border: 1px solid var(--destructive-border);
    }

    .member-meta { display: flex; align-items: center; gap: 0.5rem; }
    .meta-date { font-size: 0.8125rem; color: var(--muted-foreground); }

    .member-actions { display: flex; gap: 0.4rem; flex-wrap: wrap; align-items: center; }
    .member-actions select {
        width: auto; font-size: 0.8125rem; padding: 4px 8px;
    }

    .btn-action {
        font: 500 0.8125rem/1 var(--font-sans); border: 1px solid transparent;
        border-radius: var(--radius-sm); padding: 4px 10px; cursor: pointer;
        transition: opacity 0.15s;
    }
    .btn-action:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn-action.grant   { background: var(--success-bg); border-color: var(--success-border); color: #3a9e6a; }
    .btn-action.revoke  { background: var(--muted); border-color: var(--border); color: var(--muted-foreground); }
    .btn-action.danger  { background: var(--destructive-bg); border-color: var(--destructive-border); color: var(--destructive); }

    .access-requests {
        display: flex; gap: 0.4rem; flex-wrap: wrap;
        padding-top: 0.5rem; border-top: 1px solid var(--border);
    }
    .btn-access {
        font: 500 0.8125rem/1 var(--font-sans); border: 1px solid var(--destructive-border);
        background: var(--destructive-bg); color: var(--destructive);
        border-radius: var(--radius-sm); padding: 4px 10px; cursor: pointer;
    }
    .btn-access.granted { background: var(--success-bg); border-color: var(--success-border); color: #3a9e6a; }
</style>
