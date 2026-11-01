<script lang="ts">
    // Onglet Membres (AUTH-12) : les membres et les invitations en attente, en
    // cartes. Une invitation est un membre pas encore arrivé : elle vit ici, en
    // pointillés, jusqu'à ce qu'elle soit utilisée. Seule façon d'ajouter
    // quelqu'un : « Inviter quelqu'un » → un lien à envoyer.
    import { invalidateAll } from '$app/navigation';

    type User = { pseudo: string; role: string; createdAt: number };
    type Invite = { id: number; code: string; note: string; expiresAt: number };

    let { users, invites, me, signupUrl, avatarBase }: {
        users: User[]; invites: Invite[]; me: string; signupUrl: string; avatarBase: string;
    } = $props();

    const ROLE_LABEL: Record<string, string> = { admin: 'admin', mod: 'modo', member: 'membre', guest: 'invité' };
    const DAY = 86400;

    let now = $state(Math.floor(Date.now() / 1000));
    let pending = $derived(invites.filter(i => i.expiresAt > now));
    let expired = $derived(invites.filter(i => i.expiresAt <= now));

    // ── Inviter ──
    let inviteOpen = $state(false);
    let inviteNote = $state('');
    let inviteBusy = $state(false);
    let inviteMsg = $state('');

    // ── Cartes invitation ──
    let copied = $state<number | null>(null);
    let busyInvite = $state<Record<number, boolean>>({});

    // ── Fiche membre ──
    let dialog = $state<HTMLDialogElement>();
    let selected = $state<User | null>(null);
    let newPwd = $state('');
    let pwdOpen = $state(false);
    let pwdBusy = $state(false);
    let pwdMsg = $state('');
    let confirmDelete = $state(false);
    let deleteBusy = $state(false);
    let deleteErr = $state('');

    const avatar = (pseudo: string) => `${avatarBase}${encodeURIComponent(pseudo)}`;
    const linkOf = (inv: Invite) => `${signupUrl}?invite=${inv.code}`;

    function since(epoch: number) {
        return new Date(epoch * 1000).toLocaleDateString('fr-FR', { month: 'short', year: 'numeric' });
    }
    function left(inv: Invite) {
        const d = Math.ceil((inv.expiresAt - now) / DAY);
        return d <= 1 ? 'expire aujourd’hui' : `expire dans ${d} jours`;
    }

    async function copyLink(inv: Invite): Promise<boolean> {
        try {
            await navigator.clipboard.writeText(linkOf(inv));
            copied = inv.id;
            setTimeout(() => { if (copied === inv.id) copied = null; }, 2500);
            return true;
        } catch { return false; }
    }

    async function createInvite() {
        inviteBusy = true; inviteMsg = '';
        try {
            const r = await fetch('/japprends/invites', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ note: inviteNote.trim() }),
            });
            const d = await r.json();
            if (!d.ok) { inviteMsg = 'Impossible de créer le lien.'; return; }
            const who = inviteNote.trim();
            await invalidateAll();
            now = Math.floor(Date.now() / 1000);
            const ok = await copyLink({ id: d.id, code: d.code, note: who, expiresAt: 0 });
            inviteMsg = ok
                ? `✓ Lien copié, envoie-le${who ? ` à ${who}` : ''}. Il marche 7 jours.`
                : '✓ Lien créé : clique sur « Copier le lien » sur sa carte.';
            inviteNote = ''; inviteOpen = false;
        } finally { inviteBusy = false; }
    }

    async function removeInvite(inv: Invite) {
        busyInvite[inv.id] = true;
        try {
            await fetch(`/japprends/invites/${inv.id}`, { method: 'DELETE' });
            await invalidateAll();
        } finally { busyInvite[inv.id] = false; }
    }

    async function cleanExpired() {
        for (const inv of expired) await fetch(`/japprends/invites/${inv.id}`, { method: 'DELETE' });
        await invalidateAll();
    }

    function openMember(u: User) {
        selected = u;
        pwdOpen = false; newPwd = ''; pwdMsg = '';
        confirmDelete = false; deleteErr = '';
        dialog?.showModal();
    }

    async function resetPassword() {
        if (!selected || !newPwd) return;
        pwdBusy = true; pwdMsg = '';
        try {
            const r = await fetch(`/japprends/users/${encodeURIComponent(selected.pseudo)}/password`, {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ password: newPwd }),
            });
            const d = await r.json();
            pwdMsg = d.ok ? '✓ Mot de passe modifié, ses sessions sont fermées.' : (d.message ?? 'Erreur.');
            if (d.ok) { newPwd = ''; pwdOpen = false; }
        } finally { pwdBusy = false; }
    }

    async function deleteMember() {
        if (!selected) return;
        deleteBusy = true; deleteErr = '';
        try {
            const r = await fetch(`/japprends/users/${encodeURIComponent(selected.pseudo)}`, { method: 'DELETE' });
            const d = await r.json();
            if (!d.ok) { deleteErr = d.message ?? 'Erreur.'; return; }
            dialog?.close();
            await invalidateAll();
        } finally { deleteBusy = false; }
    }
</script>

<div class="members-tab">

    <header class="tab-header">
        <h2>Membres <span class="count">{users.length}</span></h2>
        {#if pending.length}
            <span class="sub">· {pending.length} invitation{pending.length > 1 ? 's' : ''} en attente</span>
        {/if}
        <button class="btn-primary" onclick={() => { inviteOpen = !inviteOpen; inviteMsg = ''; }}>
            {inviteOpen ? 'Annuler' : '+ Inviter quelqu’un'}
        </button>
    </header>

    {#if inviteOpen}
        <form class="invite-form" onsubmit={(e) => { e.preventDefault(); createInvite(); }}>
            <label for="invite-note">Pour qui ? <span class="hint">(facultatif, juste pour t’en souvenir)</span></label>
            <div class="row">
                <!-- svelte-ignore a11y_autofocus -->
                <input id="invite-note" bind:value={inviteNote} placeholder="ex : Alice" maxlength="80" autocomplete="off" autofocus />
                <button class="btn-primary" type="submit" disabled={inviteBusy}>
                    {inviteBusy ? '…' : 'Créer le lien'}
                </button>
            </div>
        </form>
    {/if}
    {#if inviteMsg}<p class="feedback" role="status">{inviteMsg}</p>{/if}

    <div class="grid">
        {#each users as u (u.pseudo)}
            <button class="card member" onclick={() => openMember(u)}>
                <img class="avatar" src={avatar(u.pseudo)} alt="" loading="lazy" />
                <span class="pseudo">{u.pseudo}</span>
                <span class="role" class:admin={u.role === 'admin'}>{ROLE_LABEL[u.role] ?? u.role}</span>
                <span class="meta">depuis {since(u.createdAt)}</span>
            </button>
        {/each}

        {#each pending as inv (inv.id)}
            <div class="card invite">
                <span class="avatar envelope" aria-hidden="true">✉</span>
                <span class="pseudo">{inv.note || 'Invitation'}</span>
                <span class="role pending">en attente</span>
                <span class="meta">{left(inv)}</span>
                <button class="btn-copy" onclick={() => copyLink(inv)}>
                    {copied === inv.id ? '✓ Copié' : 'Copier le lien'}
                </button>
                <button class="btn-link" disabled={busyInvite[inv.id]} onclick={() => removeInvite(inv)}>
                    Retirer
                </button>
            </div>
        {/each}
    </div>

    {#if expired.length}
        <p class="expired">
            {expired.length} invitation{expired.length > 1 ? 's' : ''} expirée{expired.length > 1 ? 's' : ''} ·
            <button class="btn-link" onclick={cleanExpired}>Nettoyer</button>
        </p>
    {/if}
</div>

<!-- Fiche d'un membre -->
<dialog bind:this={dialog} class="sheet" onclose={() => { selected = null; }}>
    {#if selected}
        <button class="close" aria-label="Fermer" onclick={() => dialog?.close()}>✕</button>
        <div class="sheet-head">
            <img class="avatar big" src={avatar(selected.pseudo)} alt="" />
            <div>
                <h3>{selected.pseudo}</h3>
                <p class="meta">{ROLE_LABEL[selected.role] ?? selected.role} · depuis {since(selected.createdAt)}</p>
            </div>
        </div>

        <section>
            <h4>Mot de passe</h4>
            {#if pwdOpen}
                <form class="row" onsubmit={(e) => { e.preventDefault(); resetPassword(); }}>
                    <input type="password" bind:value={newPwd} placeholder="Nouveau mot de passe" autocomplete="new-password" />
                    <button class="btn-primary" type="submit" disabled={pwdBusy || !newPwd}>{pwdBusy ? '…' : 'Valider'}</button>
                    <button class="btn" type="button" onclick={() => { pwdOpen = false; newPwd = ''; }}>Annuler</button>
                </form>
            {:else}
                <button class="btn" onclick={() => { pwdOpen = true; pwdMsg = ''; }}>Réinitialiser le mot de passe</button>
            {/if}
            {#if pwdMsg}<p class="feedback" role="status">{pwdMsg}</p>{/if}
        </section>

        {#if selected.pseudo !== me}
            <section class="danger-zone">
                {#if confirmDelete}
                    <p>Supprimer <strong>{selected.pseudo}</strong> et toutes ses données ? C’est définitif.</p>
                    <div class="row">
                        <button class="btn danger" disabled={deleteBusy} onclick={deleteMember}>
                            {deleteBusy ? '…' : `Oui, supprimer ${selected.pseudo}`}
                        </button>
                        <button class="btn" onclick={() => { confirmDelete = false; }}>Non, garder</button>
                    </div>
                {:else}
                    <button class="btn-link danger" onclick={() => { confirmDelete = true; }}>Supprimer le compte</button>
                {/if}
                {#if deleteErr}<p class="err">{deleteErr}</p>{/if}
            </section>
        {/if}
    {/if}
</dialog>

<style>
    .members-tab { display: flex; flex-direction: column; gap: 1.25rem; }

    .tab-header { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; }
    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }
    .sub { font-size: 0.875rem; color: var(--muted-foreground); }
    .count {
        display: inline-flex; align-items: center; justify-content: center;
        background: var(--muted); border-radius: 99px;
        font-size: 0.75rem; font-weight: 500; padding: 1px 7px;
        color: var(--muted-foreground);
    }
    .tab-header .btn-primary { margin-left: auto; }

    .btn-primary {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 8px 16px;
        font: 500 0.875rem/1 var(--font-sans); cursor: pointer; white-space: nowrap;
    }
    .btn-primary:hover { background: var(--primary-hover); }
    .btn-primary:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn {
        font: 500 0.8125rem/1 var(--font-sans); border: 1px solid var(--border);
        background: var(--card); border-radius: var(--radius-sm);
        padding: 7px 12px; cursor: pointer; color: var(--foreground); white-space: nowrap;
    }
    .btn:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn.danger { background: var(--destructive-bg); border-color: var(--destructive-border); color: var(--destructive); }
    .btn-link {
        background: none; border: none; padding: 2px 4px; cursor: pointer;
        font: 400 0.8125rem/1.2 var(--font-sans); color: var(--muted-foreground);
        text-decoration: underline; text-underline-offset: 2px;
    }
    .btn-link:hover { color: var(--foreground); }
    .btn-link.danger { color: var(--destructive); }
    .btn-link:disabled { opacity: 0.4; cursor: not-allowed; }

    .invite-form {
        display: flex; flex-direction: column; gap: 0.5rem;
        padding: 1rem; background: var(--muted);
        border: 1px solid var(--border); border-radius: var(--radius-md);
    }
    .invite-form label { font-size: 0.875rem; font-weight: 500; }
    .hint { font-weight: 400; color: var(--muted-foreground); }
    .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; }
    .row input { flex: 1; min-width: 160px; }

    .feedback { margin: 0; font-size: 0.875rem; color: var(--muted-foreground); }
    .err { margin: 0; font-size: 0.8125rem; color: var(--destructive); }

    /* ── Cartes ── */
    .grid {
        display: grid; gap: 1rem;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    }
    .card {
        display: flex; flex-direction: column; align-items: center; gap: 0.35rem;
        padding: 1.25rem 0.75rem 1rem;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--card); color: var(--foreground);
        font: inherit; text-align: center;
    }
    .card.member { cursor: pointer; transition: border-color 0.15s, transform 0.15s; }
    .card.member:hover, .card.member:focus-visible { border-color: var(--primary); transform: translateY(-2px); }
    .card.invite { border-style: dashed; background: transparent; }

    .avatar {
        width: 64px; height: 64px; border-radius: 50%;
        background: var(--muted); margin-bottom: 0.4rem; object-fit: cover;
    }
    .avatar.big { width: 72px; height: 72px; margin: 0; }
    .envelope {
        display: flex; align-items: center; justify-content: center;
        font-size: 1.6rem; color: var(--muted-foreground);
    }
    .pseudo {
        font-weight: 600; font-size: 0.9375rem;
        max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
    }
    .role {
        font-size: 0.6875rem; font-weight: 600; border-radius: 99px; padding: 2px 8px;
        background: var(--muted); color: var(--muted-foreground);
    }
    .role.admin { background: var(--primary); color: var(--primary-foreground); }
    .role.pending { background: transparent; border: 1px dashed var(--border); }
    .meta { font-size: 0.75rem; color: var(--muted-foreground); }

    .btn-copy {
        margin-top: 0.5rem; width: 100%;
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-sm); padding: 7px 10px;
        font: 500 0.8125rem/1 var(--font-sans); cursor: pointer;
    }
    .btn-copy:hover { background: var(--primary-hover); }

    .expired { margin: 0; font-size: 0.8125rem; color: var(--muted-foreground); text-align: center; }

    /* ── Fiche ── */
    .sheet {
        width: min(420px, calc(100vw - 2rem));
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--card); color: var(--foreground);
        padding: 1.5rem; position: relative;
    }
    .sheet::backdrop { background: rgb(0 0 0 / 0.45); }
    .close {
        position: absolute; top: 0.75rem; right: 0.75rem;
        background: none; border: none; cursor: pointer;
        color: var(--muted-foreground); font-size: 0.875rem; padding: 4px 8px;
    }
    .sheet-head { display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem; }
    .sheet-head h3 { margin: 0; font-size: 1.125rem; }
    .sheet-head .meta { margin: 0.25rem 0 0; font-size: 0.8125rem; }
    .sheet section { display: flex; flex-direction: column; gap: 0.6rem; align-items: flex-start; }
    .sheet h4 {
        margin: 0; font-size: 0.75rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.05em; color: var(--muted-foreground);
    }
    .sheet .row { width: 100%; }
    .danger-zone { margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--border); }
    .danger-zone p { margin: 0; font-size: 0.875rem; }
</style>
