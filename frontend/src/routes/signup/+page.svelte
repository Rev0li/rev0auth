<script lang="ts">
    import { goto } from '$app/navigation';
    import type { PageData } from './$types.js';

    let { data }: { data: PageData } = $props();

    const AVATARS = [
        { id: 'fox',    name: 'Renard',  svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#d4500a'/><polygon points='20,55 30,20 42,55' fill='#d4500a'/><polygon points='58,55 70,20 80,55' fill='#d4500a'/><polygon points='23,52 30,27 39,52' fill='#f9b084'/><polygon points='61,52 70,27 77,52' fill='#f9b084'/><circle cx='50' cy='60' r='22' fill='#f9b084'/><ellipse cx='43' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><ellipse cx='57' cy='54' rx='4' ry='4.5' fill='#1a1a1a'/><circle cx='44' cy='53' r='1.2' fill='white'/><circle cx='58' cy='53' r='1.2' fill='white'/><ellipse cx='50' cy='64' rx='3' ry='2' fill='#1a1a1a'/><ellipse cx='50' cy='68' rx='9' ry='5' fill='#fde4cc' opacity='0.7'/></svg>` },
        { id: 'wolf',   name: 'Loup',    svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#4a5568'/><polygon points='18,52 28,15 40,52' fill='#4a5568'/><polygon points='60,52 72,15 82,52' fill='#4a5568'/><polygon points='21,50 28,22 37,50' fill='#9aa5b4'/><polygon points='63,50 72,22 79,50' fill='#9aa5b4'/><ellipse cx='50' cy='62' rx='24' ry='20' fill='#9aa5b4'/><ellipse cx='50' cy='71' rx='13' ry='9' fill='#bec5cf'/><ellipse cx='42' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='4.5' ry='4' fill='#1a1a1a'/><circle cx='43' cy='53' r='1.3' fill='#e8f0fe'/><circle cx='59' cy='53' r='1.3' fill='#e8f0fe'/><ellipse cx='50' cy='65' rx='4' ry='2.5' fill='#2d3748'/></svg>` },
        { id: 'cat',    name: 'Chat',    svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#6b46c1'/><polygon points='22,52 32,18 44,52' fill='#6b46c1'/><polygon points='56,52 68,18 78,52' fill='#6b46c1'/><polygon points='25,50 32,25 41,50' fill='#f9a8d4'/><polygon points='59,50 68,25 75,50' fill='#f9a8d4'/><circle cx='50' cy='60' r='22' fill='#9f7aea'/><ellipse cx='42' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='58' cy='54' rx='5' ry='4' fill='#1a1a1a'/><ellipse cx='42' cy='54' rx='2' ry='3.5' fill='#52b788'/><ellipse cx='58' cy='54' rx='2' ry='3.5' fill='#52b788'/><circle cx='43' cy='53' r='1' fill='white'/><polygon points='50,62 47,65 53,65' fill='#f9a8d4'/><line x1='28' y1='64' x2='43' y2='67' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='28' y1='68' x2='43' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='67' x2='72' y2='64' stroke='white' stroke-width='0.8' opacity='0.7'/><line x1='57' y1='68' x2='72' y2='68' stroke='white' stroke-width='0.8' opacity='0.7'/></svg>` },
        { id: 'eagle',  name: 'Aigle',   svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#1a202c'/><circle cx='50' cy='56' r='24' fill='#744210'/><circle cx='50' cy='48' r='17' fill='#f7fafc'/><circle cx='44' cy='46' r='5' fill='#f6ad55'/><circle cx='44' cy='46' r='3' fill='#1a1a1a'/><circle cx='45' cy='45' r='1' fill='white'/><polygon points='35,52 50,48 37,60' fill='#f6ad55'/><ellipse cx='63' cy='62' rx='12' ry='8' fill='#2d3748'/><ellipse cx='37' cy='63' rx='10' ry='7' fill='#744210'/></svg>` },
        { id: 'dragon', name: 'Dragon',  svg: `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><circle cx='50' cy='50' r='50' fill='#065f46'/><polygon points='38,30 34,10 42,28' fill='#34d399'/><polygon points='62,30 66,10 58,28' fill='#34d399'/><circle cx='50' cy='58' r='24' fill='#059669'/><ellipse cx='50' cy='70' rx='12' ry='9' fill='#34d399'/><circle cx='46' cy='69' r='2' fill='#065f46'/><circle cx='54' cy='69' r='2' fill='#065f46'/><ellipse cx='41' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='59' cy='52' rx='5.5' ry='4' fill='#f59e0b'/><ellipse cx='41' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><ellipse cx='59' cy='52' rx='1.5' ry='4' fill='#1a1a1a'/><path d='M38,64 Q50,58 62,64' fill='none' stroke='#34d399' stroke-width='1.5' opacity='0.6'/></svg>` },
    ];

    let selectedAvatar = $state('');
    let pseudo    = $state('');
    let password  = $state('');
    let confirm   = $state('');
    let loading   = $state(false);
    let error     = $state('');

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
                    avatar_id:   selectedAvatar || null,
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
                    oninput={() => { error = ''; }}
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
    .avatar-btn :global(svg) { width: 48px; height: 48px; border-radius: 50%; }
    .avatar-btn span { font-size: 0.7rem; color: var(--muted-foreground); }
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
