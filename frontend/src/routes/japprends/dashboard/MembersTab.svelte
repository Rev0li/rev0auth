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

    // ── Rôle (AUTH-27) : Modérateur n'a pas encore de droits propres (futur
    // tableau blanc). Admin demande une confirmation : effet immédiat dans les
    // services derrière la porte, console seulement après une clé enregistrée.
    const ROLE_CHOICES = [
        { id: 'member', label: 'Membre' },
        { id: 'mod',    label: 'Modérateur' },
        { id: 'admin',  label: 'Admin' },
    ] as const;
    let confirmAdmin = $state(false);
    let roleBusy = $state(false);
    let roleMsg = $state('');

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
        confirmAdmin = false; roleMsg = '';
        dialog?.showModal();
    }

    function pickRole(role: string) {
        if (!selected || role === selected.role || roleBusy) return;
        roleMsg = '';
        if (role === 'admin') { confirmAdmin = true; return; }
        setRole(role);
    }

    async function setRole(role: string) {
        if (!selected) return;
        roleBusy = true; roleMsg = '';
        try {
            const r = await fetch(`/japprends/users/${encodeURIComponent(selected.pseudo)}`, {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ role }),
            });
            const d = await r.json();
            if (!d.ok) { roleMsg = d.message ?? 'Erreur.'; return; }
            selected = { ...selected, role };
            confirmAdmin = false;
            const label = ROLE_CHOICES.find(c => c.id === role)?.label.toLowerCase() ?? role;
            roleMsg = `✓ ${selected.pseudo} est maintenant ${label}.`;
            await invalidateAll();
        } finally { roleBusy = false; }
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
            <h4>Rôle</h4>
            {#if selected.pseudo === me}
                <p class="meta">C’est toi : ton rôle ne se change pas d’ici.</p>
            {:else}
                <div class="segmented" role="radiogroup" aria-label="Rôle de {selected.pseudo}">
                    {#each ROLE_CHOICES as c (c.id)}
                        {@const on = (confirmAdmin ? 'admin' : selected.role) === c.id}
                        <button
                            type="button" role="radio" aria-checked={on}
                            class:on disabled={roleBusy}
                            onclick={() => pickRole(c.id)}
                        >{c.label}</button>
                    {/each}
                </div>
                {#if confirmAdmin}
                    <div class="confirm-box">
                        <p>Donner le rôle <strong>admin</strong> à {selected.pseudo} ?</p>
                        <ul>
                            <li>Tout de suite : pouvoirs admin dans SongSurf (pas de quota, voit les téléchargements de tous).</li>
                            <li>La console lui reste fermée tant que tu ne lui as pas enregistré une clé de sécurité (code à générer sur le serveur).</li>
                        </ul>
                        <div class="row">
                            <button class="btn-primary" disabled={roleBusy} onclick={() => setRole('admin')}>
                                {roleBusy ? '…' : `Oui, ${selected.pseudo} devient admin`}
                            </button>
                            <button class="btn" onclick={() => { confirmAdmin = false; }}>Annuler</button>
                        </div>
                    </div>
                {/if}
                {#if selected.role === 'mod' && !confirmAdmin}
                    <p class="meta">Modérateur : mêmes accès qu’un membre pour l’instant (droits à venir avec le tableau blanc).</p>
                {/if}
            {/if}
            {#if roleMsg}<p class="feedback" role="status">{roleMsg}</p>{/if}
        </section>

        <section class="spaced">
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
    .sub { font-size: 0.875rem; color: var(--rv-muted); }
    .count {
        display: inline-flex; align-items: center; justify-content: center;
        background: var(--rv-panel); border-radius: 99px;
        font-size: 0.75rem; font-weight: 500; padding: 1px 7px;
        color: var(--rv-muted);
    }
    .tab-header .btn-primary { margin-left: auto; }

    .btn-primary {
        background: var(--rv-turq); color: var(--rv-on-accent);
        border: none; border-radius: var(--rv-radius-sm); padding: 8px 16px;
        font: 500 0.875rem/1 var(--rv-font-sans); cursor: pointer; white-space: nowrap;
    }
    .btn-primary:hover { box-shadow: var(--rv-shadow-gel); }
    .btn-primary:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn {
        font: 500 0.8125rem/1 var(--rv-font-sans); border: 1px solid var(--rv-line);
        background: var(--rv-surface); border-radius: var(--rv-radius-sm);
        padding: 7px 12px; cursor: pointer; color: var(--rv-ink); white-space: nowrap;
    }
    .btn:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn.danger { background: var(--rv-danger-soft); border-color: var(--rv-danger); color: var(--rv-danger-ink); }
    .btn-link {
        background: none; border: none; padding: 2px 4px; cursor: pointer;
        font: 400 0.8125rem/1.2 var(--rv-font-sans); color: var(--rv-muted);
        text-decoration: underline; text-underline-offset: 2px;
    }
    .btn-link:hover { color: var(--rv-ink); }
    .btn-link.danger { color: var(--rv-danger-ink); }
    .btn-link:disabled { opacity: 0.4; cursor: not-allowed; }

    .invite-form {
        display: flex; flex-direction: column; gap: 0.5rem;
        padding: 1rem; background: var(--rv-panel);
        border: 1px solid var(--rv-line); border-radius: var(--rv-radius-sm);
    }
    .invite-form label { font-size: 0.875rem; font-weight: 500; }
    .hint { font-weight: 400; color: var(--rv-muted); }
    .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; }
    .row input { flex: 1; min-width: 160px; }

    .feedback { margin: 0; font-size: 0.875rem; color: var(--rv-muted); }
    .err { margin: 0; font-size: 0.8125rem; color: var(--rv-danger-ink); }

    /* ── Cartes ── */
    .grid {
        display: grid; gap: 1rem;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    }
    .card {
        display: flex; flex-direction: column; align-items: center; gap: 0.35rem;
        padding: 1.25rem 0.75rem 1rem;
        border: 1px solid var(--rv-line); border-radius: var(--rv-radius-sm);
        background: var(--rv-surface); color: var(--rv-ink);
        font: inherit; text-align: center;
    }
    .card.member { cursor: pointer; transition: border-color 0.15s, transform 0.15s; }
    .card.member:hover, .card.member:focus-visible { border-color: var(--rv-turq-ink); transform: translateY(-2px); }
    .card.invite { border-style: dashed; background: transparent; }

    .avatar {
        width: 64px; height: 64px; border-radius: 50%;
        background: var(--rv-panel); margin-bottom: 0.4rem; object-fit: cover;
    }
    .avatar.big { width: 72px; height: 72px; margin: 0; }
    .envelope {
        display: flex; align-items: center; justify-content: center;
        font-size: 1.6rem; color: var(--rv-muted);
    }
    .pseudo {
        font-weight: 600; font-size: 0.9375rem;
        max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
    }
    .role {
        font-size: 0.6875rem; font-weight: 600; border-radius: 99px; padding: 2px 8px;
        background: var(--rv-panel); color: var(--rv-muted);
    }
    .role.admin { background: var(--rv-turq); color: var(--rv-on-accent); }
    .role.pending { background: transparent; border: 1px dashed var(--rv-line); }
    .meta { font-size: 0.75rem; color: var(--rv-muted); }

    .btn-copy {
        margin-top: 0.5rem; width: 100%;
        background: var(--rv-turq); color: var(--rv-on-accent);
        border: none; border-radius: var(--rv-radius-sm); padding: 7px 10px;
        font: 500 0.8125rem/1 var(--rv-font-sans); cursor: pointer;
    }
    .btn-copy:hover { box-shadow: var(--rv-shadow-gel); }

    .expired { margin: 0; font-size: 0.8125rem; color: var(--rv-muted); text-align: center; }

    /* ── Fiche ── */
    .sheet {
        width: min(420px, calc(100vw - 2rem));
        border: 1px solid var(--rv-line); border-radius: var(--rv-radius-sm);
        background: var(--rv-surface); color: var(--rv-ink);
        padding: 1.5rem; position: relative;
    }
    .sheet::backdrop { background: color-mix(in oklab, var(--rv-band) 45%, transparent); }
    .close {
        position: absolute; top: 0.75rem; right: 0.75rem;
        background: none; border: none; cursor: pointer;
        color: var(--rv-muted); font-size: 0.875rem; padding: 4px 8px;
    }
    .sheet-head { display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem; }
    .sheet-head h3 { margin: 0; font-size: 1.125rem; }
    .sheet-head .meta { margin: 0.25rem 0 0; font-size: 0.8125rem; }
    .sheet section { display: flex; flex-direction: column; gap: 0.6rem; align-items: flex-start; }
    .sheet h4 {
        margin: 0; font-size: 0.75rem; font-weight: 600; text-transform: uppercase;
        letter-spacing: 0.05em; color: var(--rv-muted);
    }
    .sheet .row { width: 100%; }
    .sheet section.spaced { margin-top: 1.5rem; }

    /* Choix du rôle : trois boutons collés, celui en cours plein */
    .segmented {
        display: inline-flex; border: 1px solid var(--rv-line);
        border-radius: var(--rv-radius-sm); overflow: hidden;
    }
    .segmented button {
        font: 500 0.875rem/1 var(--rv-font-sans); padding: 9px 16px;
        background: var(--rv-surface); color: var(--rv-ink);
        border: none; border-right: 1px solid var(--rv-line); cursor: pointer;
    }
    .segmented button:last-child { border-right: none; }
    .segmented button:hover:not(.on) { background: var(--rv-panel); }
    .segmented button.on { background: var(--rv-turq); color: var(--rv-on-accent); }
    .segmented button:disabled { cursor: wait; }

    .confirm-box {
        width: 100%; padding: 0.875rem 1rem;
        border: 1px solid var(--rv-turq-ink); border-radius: var(--rv-radius-sm);
        background: var(--rv-panel);
        display: flex; flex-direction: column; gap: 0.5rem;
    }
    .confirm-box p { margin: 0; font-size: 0.875rem; }
    .confirm-box ul { margin: 0; padding-left: 1.1rem; font-size: 0.8125rem; color: var(--rv-muted); line-height: 1.45; }

    .danger-zone { margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--rv-line); }
    .danger-zone p { margin: 0; font-size: 0.875rem; }
</style>
