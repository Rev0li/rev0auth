<script lang="ts">
    import type { PageData } from './$types.js';
    import { goto, invalidateAll } from '$app/navigation';
    import { slide, fade } from 'svelte/transition';
    import { AVATARS } from '$lib/avatars.js';

    let { data }: { data: PageData } = $props();

    type Tab = 'profil' | 'donations' | 'compte';
    let tab = $state<Tab>('profil');

    // ── Profil ────────────────────────────────────────────────────────
    let bio         = $state(data.user.bio ?? '');
    let saveLoading = $state(false);
    let saveMsg     = $state('');
    let saveOk      = $state(false);

    async function saveProfile() {
        saveLoading = true; saveMsg = '';
        try {
            const r = await fetch('/members/profile/data', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ bio }),
            });
            saveOk = r.ok;
            saveMsg = r.ok ? 'Profil mis à jour.' : 'Erreur lors de la sauvegarde.';
        } finally { saveLoading = false; }
    }

    // ── Avatar ────────────────────────────────────────────────────────
    let selectedAvatar = $state('');
    let avatarLoading  = $state(false);
    let avatarMsg      = $state('');

    async function saveAvatar() {
        if (!selectedAvatar) return;
        avatarLoading = true; avatarMsg = '';
        try {
            const r = await fetch('/members/avatar', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ avatar_id: selectedAvatar }),
            });
            avatarMsg = r.ok ? 'Avatar mis à jour.' : 'Erreur lors de la mise à jour.';
            if (r.ok) { selectedAvatar = ''; await invalidateAll(); }
        } finally { avatarLoading = false; }
    }

    async function deleteAvatar() {
        await fetch('/members/avatar', { method: 'DELETE' });
        selectedAvatar = '';
        invalidateAll();
    }

    // ── Mot de passe ─────────────────────────────────────────────────
    let currentPwd = $state('');
    let newPwd     = $state('');
    let confirmPwd = $state('');
    let pwdLoading = $state(false);
    let pwdMsg     = $state('');
    let pwdOk      = $state(false);

    async function changePassword() {
        pwdMsg = '';
        if (newPwd !== confirmPwd) { pwdMsg = 'Les mots de passe ne correspondent pas.'; return; }
        if (newPwd.length < 8) { pwdMsg = 'Minimum 8 caractères.'; return; }
        pwdLoading = true;
        try {
            const r = await fetch('/members/password', {
                method: 'PUT',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ currentPassword: currentPwd, newPassword: newPwd }),
            });
            const d = await r.json();
            pwdOk = d.ok;
            pwdMsg = d.message;
            if (d.ok) { currentPwd = ''; newPwd = ''; confirmPwd = ''; }
        } finally { pwdLoading = false; }
    }

    // ── Donations ────────────────────────────────────────────────────
    let donaMethod  = $state<'pcs' | 'crypto'>('pcs');
    let donaCode    = $state('');
    let donaLoading = $state(false);
    let donaMsg     = $state('');
    let donaOk      = $state(false);

    async function submitDona() {
        if (!donaCode.trim()) return;
        donaLoading = true; donaMsg = '';
        try {
            const r = await fetch('/members/donations', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ method: donaMethod, code: donaCode }),
            });
            donaOk = r.ok;
            donaMsg = r.ok ? 'Code envoyé, en attente de validation.' : 'Erreur.';
            if (r.ok) { donaCode = ''; invalidateAll(); }
        } finally { donaLoading = false; }
    }

    async function logout() {
        await fetch('/auth/logout', { method: 'POST' });
        goto('/');
    }

    function timeAgo(date: Date | string | number) {
        const d = typeof date === 'number' ? new Date(date * 1000) : new Date(date);
        const sec = Math.floor((Date.now() - d.getTime()) / 1000);
        if (sec < 60) return 'à l\'instant';
        if (sec < 3600) return `il y a ${Math.floor(sec / 60)} min`;
        if (sec < 86400) return `il y a ${Math.floor(sec / 3600)} h`;
        return d.toLocaleDateString('fr-FR');
    }
</script>

<!-- Navbar -->
<nav class="navbar">
    <a class="navbar-back" href="/home/friend">← Home</a>
    <span class="navbar-title">{data.user.pseudo}</span>
    <button class="nav-btn nav-btn-logout" onclick={logout}>Déconnexion</button>
</nav>

<!-- Tabs -->
<div class="tabbar">
    {#each [
        { id: 'profil',    label: 'Profil'    },
        { id: 'donations', label: 'Donations' },
        { id: 'compte',    label: 'Compte'    },
    ] as t}
        <button class="tabbar-btn" class:active={tab === t.id} onclick={() => tab = t.id as Tab}>{t.label}</button>
    {/each}
</div>

<div class="profile-content">

    <!-- ── PROFIL ── -->
    {#if tab === 'profil'}
        <div transition:fade={{ duration: 120 }}>
            <!-- Avatar -->
            <div class="section-card card">
                <h2 class="section-title">Avatar</h2>
                <div class="avatar-area">
                    <img
                        src={`/members/avatar/${data.user.pseudo}`}
                        alt="Avatar"
                        class="profile-avatar"
                        onerror={(e) => (e.currentTarget as HTMLImageElement).src = ''}
                    />
                    <div class="avatar-grid">
                        {#each AVATARS as av}
                            <button
                                type="button"
                                class="avatar-btn"
                                class:selected={selectedAvatar === av.id}
                                onclick={() => { selectedAvatar = selectedAvatar === av.id ? '' : av.id; }}
                                title={av.name}
                            >
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html av.svg}
                                <span>{av.name}</span>
                            </button>
                        {/each}
                    </div>
                    <div class="avatar-actions">
                        {#if selectedAvatar}
                            <button class="btn-primary" onclick={saveAvatar} disabled={avatarLoading}>
                                {avatarLoading ? '…' : 'Sauvegarder'}
                            </button>
                        {/if}
                        <button class="btn-secondary revoke" onclick={deleteAvatar}>Supprimer</button>
                    </div>
                    {#if avatarMsg}<p class="meta">{avatarMsg}</p>{/if}
                </div>
            </div>

            <!-- Bio -->
            <div class="section-card card">
                <h2 class="section-title">Informations</h2>
                <div class="field">
                    <label for="bio">Bio</label>
                    <textarea id="bio" bind:value={bio} placeholder="Quelques mots sur toi…" rows={3}></textarea>
                </div>
                {#if saveMsg}
                    <p class={saveOk ? 'chip-ok' : 'chip-error'}>{saveMsg}</p>
                {/if}
                <button class="btn-primary" onclick={saveProfile} disabled={saveLoading}>
                    {saveLoading ? '…' : 'Sauvegarder'}
                </button>
            </div>
        </div>
    {/if}

    <!-- ── DONATIONS ── -->
    {#if tab === 'donations'}
        <div transition:fade={{ duration: 120 }}>
            <div class="section-card card">
                <h2 class="section-title">Soumettre une preuve</h2>
                <div class="folder-tabs" style="margin-bottom:1rem">
                    <button class="folder-btn" class:active={donaMethod === 'pcs'}    onclick={() => donaMethod = 'pcs'}>PCS</button>
                    <button class="folder-btn" class:active={donaMethod === 'crypto'} onclick={() => donaMethod = 'crypto'}>Crypto</button>
                </div>
                <div class="field">
                    <label for="dona-code">Code {donaMethod === 'pcs' ? 'coupon PCS' : 'transaction'}</label>
                    <input id="dona-code" bind:value={donaCode} placeholder={donaMethod === 'pcs' ? '14 chiffres' : 'hash de transaction'} />
                </div>
                {#if donaMsg}<p class={donaOk ? 'chip-ok' : 'chip-error'}>{donaMsg}</p>{/if}
                <button class="btn-primary" onclick={submitDona} disabled={donaLoading || !donaCode.trim()}>
                    {donaLoading ? '…' : 'Envoyer'}
                </button>
            </div>

            <h3 class="section-heading">Historique</h3>
            {#each data.donations as d (d.id)}
                <div class="dona-row">
                    <span class="meta">{d.method.toUpperCase()}</span>
                    <code class="dona-code">{d.code}</code>
                    <span class="meta">{timeAgo(d.createdAt)}</span>
                    {#if !d.reviewed}
                        <span class="meta">En attente…</span>
                    {:else if d.approved}
                        <span class="chip-ok" style="display:inline-block;padding:2px 10px">✓ Validé</span>
                    {:else}
                        <span class="chip-error" style="display:inline-block;padding:2px 10px">✗ Refusé</span>
                    {/if}
                </div>
            {:else}
                <p class="empty-state">Aucune donation.</p>
            {/each}
        </div>
    {/if}

    <!-- ── COMPTE ── -->
    {#if tab === 'compte'}
        <div transition:fade={{ duration: 120 }}>
            <div class="section-card card">
                <h2 class="section-title">Changer le mot de passe</h2>
                <div class="field">
                    <label for="cur-pwd">Mot de passe actuel</label>
                    <input id="cur-pwd" type="password" bind:value={currentPwd} placeholder="••••••••" />
                </div>
                <div class="field">
                    <label for="new-pwd">Nouveau mot de passe</label>
                    <input id="new-pwd" type="password" bind:value={newPwd} placeholder="••••••••" />
                </div>
                <div class="field">
                    <label for="confirm-pwd">Confirmer</label>
                    <input id="confirm-pwd" type="password" bind:value={confirmPwd} placeholder="••••••••" />
                </div>
                {#if pwdMsg}<p class={pwdOk ? 'chip-ok' : 'chip-error'}>{pwdMsg}</p>{/if}
                <button class="btn-primary" onclick={changePassword} disabled={pwdLoading}>
                    {pwdLoading ? '…' : 'Mettre à jour'}
                </button>
            </div>

            <div class="section-card card" style="border-color:var(--destructive-border)">
                <h2 class="section-title" style="color:var(--destructive)">Zone dangereuse</h2>
                <p class="meta">La suppression du compte est définitive. Contacte un admin si tu veux supprimer ton compte.</p>
            </div>
        </div>
    {/if}

</div>

<style>
    .navbar {
        position: sticky; top: 0; z-index: 100;
        display: flex; align-items: center; justify-content: space-between;
        padding: 0 1.5rem; height: 52px;
        background: var(--card); border-bottom: 1px solid var(--border);
    }
    .navbar-back { text-decoration: none; color: var(--muted-foreground); font-size: 0.875rem; transition: color 0.15s; }
    .navbar-back:hover { color: var(--foreground); }
    .navbar-title { font-weight: 700; }
    .nav-btn {
        height: 30px; padding: 0 12px; border: 1px solid var(--border);
        border-radius: var(--radius-full); background: transparent;
        font: 500 0.8125rem/1 var(--font-sans); color: var(--foreground); cursor: pointer;
    }
    .nav-btn-logout { color: var(--destructive); border-color: var(--destructive-border); }

    .tabbar {
        display: flex; gap: 2px; padding: 0 1.5rem;
        border-bottom: 1px solid var(--border); background: var(--card); overflow-x: auto;
    }
    .tabbar-btn {
        height: 44px; padding: 0 14px; background: none; border: none;
        font: 500 0.875rem/1 var(--font-sans); color: var(--muted-foreground);
        cursor: pointer; border-bottom: 2px solid transparent; white-space: nowrap;
        transition: color 0.15s, border-color 0.15s;
    }
    .tabbar-btn.active { color: var(--foreground); border-bottom-color: var(--primary); }

    .profile-content { max-width: 720px; margin: 0 auto; padding: 1.5rem; display: flex; flex-direction: column; gap: 1rem; }

    .section-card { padding: 1.5rem; }
    .section-title { margin: 0 0 1rem; font-size: 1rem; font-weight: 700; }
    .section-heading { font-size: 0.8125rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--muted-foreground); margin: 0.5rem 0 0.75rem; }
    .two-col { display: grid; grid-template-columns: 1fr 1fr; gap: 10px; }

    .avatar-area { display: flex; align-items: center; gap: 1.25rem; flex-wrap: wrap; }
    .profile-avatar { width: 80px; height: 80px; border-radius: 50%; object-fit: cover; border: 2px solid var(--border); }
    .avatar-actions { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }
    .avatar-grid { display: flex; gap: 0.75rem; flex-wrap: wrap; }
    .avatar-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
        background: none;
        border: 2px solid var(--border);
        border-radius: var(--radius);
        padding: 6px;
        cursor: pointer;
        transition: border-color 0.15s, box-shadow 0.15s;
        width: 72px;
    }
    .avatar-btn :global(svg) { width: 48px; height: 48px; border-radius: 50%; }
    .avatar-btn span { font-size: 0.7rem; color: var(--muted-foreground); }
    .avatar-btn:hover { border-color: var(--ring); }
    .avatar-btn.selected {
        border-color: var(--primary);
        box-shadow: 0 0 0 2px var(--primary);
    }

    .folder-tabs { display: flex; gap: 4px; }
    .folder-btn {
        height: 32px; padding: 0 14px; border: 1px solid var(--border);
        border-radius: var(--radius-full); background: var(--muted);
        font: 500 0.8125rem/1 var(--font-sans); cursor: pointer; color: var(--muted-foreground);
        transition: background 0.12s, color 0.12s;
    }
    .folder-btn.active { background: var(--card); color: var(--foreground); border-color: var(--primary); }

    .dona-row {
        display: flex; align-items: center; gap: 12px; flex-wrap: wrap;
        padding: 10px 14px; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card); margin-bottom: 6px;
    }
    .dona-code { font-family: var(--font-mono); font-size: 0.8125rem; background: var(--muted); padding: 2px 6px; border-radius: 4px; }
    .empty-state { color: var(--muted-foreground); text-align: center; padding: 2rem 0; }
</style>
