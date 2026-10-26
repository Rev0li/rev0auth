<script lang="ts">
    import { goto } from '$app/navigation';
    import { Button } from '$lib/components/ui/button/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Card } from '$lib/components/ui/card/index.js';

    let { data } = $props();

    let pseudo   = $state('');
    let password = $state('');
    let loading  = $state(false);
    let error    = $state('');
    let showInviteInfo = $state(false);

    async function login() {
        error = '';
        const key = pseudo.trim().toLowerCase();
        const pwd = password.trim();
        if (!key || !pwd) { error = 'Pseudo et mot de passe requis.'; return; }

        loading = true;
        try {
            const r = await fetch('/auth/password-check', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: key, password: pwd }),
            });
            // `res`, pas `data` : ce nom masquerait les données de la page (returnTo)
            const res = await r.json();
            if (!res.ok) { error = res.message; return; }

            localStorage.setItem('logged_pseudo', res.pseudo ?? key);
            // Retour au service qui a demandé la connexion (adresse vérifiée
            // côté serveur), sinon l'espace membre.
            if (data.returnTo) window.location.assign(data.returnTo);
            else goto('/home/friend');
        } catch { error = 'Erreur réseau.'; }
        finally { loading = false; }
    }

    function onKeydown(e: KeyboardEvent) {
        if (e.key === 'Enter') login();
    }
</script>

<main class="portal-bg">
    <Card class="portal-card">

        <div class="portal-logo">
            <span class="portal-brand">rev0auth</span>
            <span class="portal-tagline">Espace privé</span>
        </div>

        <div class="field">
            <label for="pseudo">Pseudo</label>
            <Input
                id="pseudo"
                bind:value={pseudo}
                placeholder="ton_pseudo"
                autocomplete="username"
                oninput={() => { error = ''; }}
                onkeydown={onKeydown}
            />
        </div>

        <div class="field">
            <label for="password">Mot de passe</label>
            <Input
                id="password"
                type="password"
                bind:value={password}
                placeholder="••••••••"
                autocomplete="current-password"
                oninput={() => { error = ''; }}
                onkeydown={onKeydown}
            />
        </div>

        {#if error}<p class="chip-error">{error}</p>{/if}

        <Button fullWidth type="button" disabled={loading} onclick={login}>
            {loading ? '…' : 'Se connecter'}
        </Button>

        {#if showInviteInfo}
            <div class="invite-info">
                <span class="invite-info-icon">🔒</span>
                <span>Inscription sur invitation uniquement — pas de lien ? Contacte un admin.</span>
            </div>
        {:else}
            <button
                class="invite-hint"
                type="button"
                onclick={() => { showInviteInfo = true; }}
            >
                Pas encore de compte ?
            </button>
        {/if}

    </Card>
</main>

<style>
    .portal-bg {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
    }

    :global(.portal-card) {
        width: 100%;
        max-width: 420px;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .portal-logo {
        text-align: center;
        margin-bottom: 0.75rem;
    }
    .portal-brand {
        display: block;
        font-size: 1.625rem;
        font-weight: 700;
        letter-spacing: -0.02em;
        color: var(--foreground);
    }
    .portal-tagline {
        font-size: 0.8125rem;
        color: var(--muted-foreground);
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }
    .field label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--foreground);
    }

    .invite-hint {
        display: block;
        width: 100%;
        padding: 0;
        background: none;
        border: none;
        font: 500 0.8125rem/1 var(--font-sans);
        color: var(--muted-foreground);
        text-align: center;
        cursor: pointer;
        transition: color 0.15s;
    }
    .invite-hint:hover { color: var(--foreground); }

    .invite-info {
        display: flex;
        align-items: flex-start;
        gap: 8px;
        padding: 10px 12px;
        background: var(--muted);
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        font-size: 0.875rem;
        color: var(--muted-foreground);
        line-height: 1.5;
    }
    .invite-info-icon {
        flex-shrink: 0;
        font-size: 1rem;
        line-height: 1.5;
    }
</style>
