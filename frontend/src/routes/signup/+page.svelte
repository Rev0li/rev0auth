<script lang="ts">
    import { goto } from '$app/navigation';
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();


    let selectedAvatar = $state('');
    let pseudo    = $state('');
    let password  = $state('');
    let confirm   = $state('');
    let loading   = $state(false);
    let error     = $state('');

    // Variantes DiceBear initial-face, seedées par le pseudo en cours de frappe
    const PSEUDO_OK = /^[a-zA-Z0-9_-]{3,20}$/;
    let avatarSeeds = $derived.by(() => {
        const p = pseudo.trim().toLowerCase();
        if (!PSEUDO_OK.test(p)) return [];
        return Array.from({ length: 8 }, (_, i) => (i === 0 ? p : `${p}-${i + 1}`));
    });

    async function submit() {
        error = '';
        if (!pseudo.trim()) { error = 'Pseudo requis.'; return; }
        if (password.length < 8) { error = 'Mot de passe trop court (8 caractères minimum).'; return; }
        if (password !== confirm) { error = 'Les mots de passe ne correspondent pas.'; return; }

        loading = true;
        try {
            const r = await fetch('/signup', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo: pseudo.trim(),
                    password,
                    invite_code: data.inviteCode,
                    avatar_seed: selectedAvatar || null,
                }),
            });
            const d = await r.json();
            if (!d.ok) { error = d.message; return; }
            setTimeout(() => goto('/'), 1200);
            error = '';
            pseudo = '✓ Compte créé ! Redirection...';
        } catch { error = 'Erreur réseau.'; }
        finally { loading = false; }
    }
</script>

<main class="portal-bg">
    <div class="portal-card">

        <div class="portal-logo">
            <span class="portal-brand">rev0auth</span>
            <span class="portal-tagline">Créer un compte</span>
        </div>

        {#if data.invalid}
            <div class="invite-error">
                <p>Ce lien d'inscription n'est plus valide.</p>
                <p>Contacte un admin pour en obtenir un nouveau.</p>
                <a href="/" class="back-link">← Retour à la connexion</a>
            </div>
        {:else}
            <p class="warn-pseudo">⚠ Ton pseudo est définitif — il ne pourra jamais être modifié.</p>

            <div class="field">
                <label for="pseudo">Pseudo *</label>
                <input
                    id="pseudo"
                    type="text"
                    bind:value={pseudo}
                    placeholder="ton_pseudo"
                    autocomplete="username"
                    oninput={() => { error = ''; selectedAvatar = ''; }}
                />
            </div>

            <div class="field">
                <label for="password">Mot de passe *</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    placeholder="8 caractères minimum"
                    autocomplete="new-password"
                    oninput={() => { error = ''; }}
                />
            </div>

            <div class="field">
                <label for="confirm">Confirmer le mot de passe *</label>
                <input
                    id="confirm"
                    type="password"
                    bind:value={confirm}
                    placeholder="répète le mot de passe"
                    autocomplete="new-password"
                    oninput={() => { error = ''; }}
                />
            </div>

            <div class="avatar-section">
                <p class="avatar-label">Choisis ton avatar <span class="optional">(optionnel)</span></p>
                {#if avatarSeeds.length > 0}
                    <div class="avatar-grid">
                        {#each avatarSeeds as seed (seed)}
                            <button
                                type="button"
                                class="avatar-btn"
                                class:selected={selectedAvatar === seed}
                                onclick={() => { selectedAvatar = selectedAvatar === seed ? '' : seed; }}
                                title="Variante {seed}"
                            >
                                <img src="/avatars/{seed}" alt="Variante" loading="lazy" />
                            </button>
                        {/each}
                    </div>
                {:else}
                    <p class="optional">Renseigne d'abord ton pseudo pour voir tes avatars.</p>
                {/if}
            </div>

            {#if error}<p class="chip-error">{error}</p>{/if}

            <button class="btn-primary" type="button" disabled={loading} onclick={submit}>
                {loading ? '…' : 'Créer mon compte'}
            </button>

            <a href="/" class="back-link">← Déjà un compte ? Se connecter</a>
        {/if}

    </div>
</main>

<style>
    .portal-bg {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
    }

    .portal-card {
        width: 100%;
        max-width: 480px;
        background: var(--card);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .portal-logo { text-align: center; margin-bottom: 0.25rem; }
    .portal-brand {
        display: block;
        font-size: 1.5rem;
        font-weight: 700;
        letter-spacing: -0.02em;
        color: var(--foreground);
    }
    .portal-tagline {
        display: block;
        font-size: 0.85rem;
        color: var(--muted-foreground);
        margin-top: 2px;
    }

    .warn-pseudo {
        font-size: 0.82rem;
        color: var(--muted-foreground);
        background: var(--muted);
        padding: 8px 12px;
        border-radius: var(--radius-sm);
        margin: 0;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
    }
    .field label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--foreground);
    }
    .field input {
        width: 100%;
        padding: 0.5rem 0.75rem;
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        background: var(--background);
        color: var(--foreground);
        font-size: 0.875rem;
        outline: none;
        box-sizing: border-box;
    }
    .field input:focus { border-color: var(--ring); }

    .avatar-section { display: flex; flex-direction: column; gap: 0.5rem; }
    .avatar-label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--foreground);
        margin: 0;
    }
    .optional { color: var(--muted-foreground); font-weight: 400; }

    .avatar-grid {
        display: flex;
        gap: 0.75rem;
        flex-wrap: wrap;
    }

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
    .avatar-btn img { width: 48px; height: 48px; border-radius: 50%; }
    .avatar-btn:hover { border-color: var(--ring); }
    .avatar-btn.selected {
        border-color: var(--primary);
        box-shadow: 0 0 0 2px var(--primary);
    }

    .chip-error {
        font-size: 0.83rem;
        color: hsl(0 72% 51%);
        background: hsl(0 72% 51% / 0.1);
        padding: 6px 10px;
        border-radius: var(--radius-sm);
        margin: 0;
    }

    .btn-primary {
        width: 100%;
        padding: 0.6rem 1rem;
        background: var(--primary);
        color: var(--primary-foreground);
        border: none;
        border-radius: var(--radius-sm);
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.15s;
    }
    .btn-primary:disabled { opacity: 0.6; cursor: default; }
    .btn-primary:not(:disabled):hover { opacity: 0.9; }

    .back-link {
        text-align: center;
        font-size: 0.83rem;
        color: var(--muted-foreground);
        text-decoration: none;
    }
    .back-link:hover { color: var(--foreground); }

    .invite-error {
        text-align: center;
        color: var(--muted-foreground);
        font-size: 0.9rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    .invite-error p { margin: 0; }
</style>
