<script lang="ts">
    import { untrack } from 'svelte';
    import { goto } from '$app/navigation';
    import { startAuthentication, startRegistration } from '@simplewebauthn/browser';

    // Console admin : on est déjà connecté comme membre, la clé de sécurité
    // ouvre la console. Sans clé enregistrée (première fois ou clé perdue) :
    // code d'installation généré sur le serveur, puis enregistrement de la clé.
    let { data } = $props();

    // Mode de départ seulement : ensuite, c'est l'utilisateur qui bascule
    let mode    = $state<'key' | 'setup'>(untrack(() => data.hasKeys) ? 'key' : 'setup');
    let code    = $state('');
    let keyName = $state('');
    let loading = $state(false);
    let error   = $state('');

    async function post(url: string, body: unknown = {}) {
        const res = await fetch(url, {
            method:  'POST',
            headers: { 'content-type': 'application/json' },
            body:    JSON.stringify(body),
        });
        const json = await res.json().catch(() => ({}));
        if (!res.ok || json.ok === false) throw new Error(json.message ?? 'Erreur.');
        return json;
    }

    // NotAllowedError : opération annulée ou clé non touchée à temps
    function describe(e: unknown): string {
        if (e instanceof Error && e.name === 'NotAllowedError') return 'Opération annulée ou clé non détectée.';
        return e instanceof Error ? e.message : 'Erreur.';
    }

    async function useKey() {
        error = '';
        loading = true;
        try {
            const { options } = await post('/japprends/passkey/auth-options');
            const response = await startAuthentication({ optionsJSON: options });
            await post('/japprends/passkey/auth-verify', { response });
            await goto('/japprends/dashboard');
        } catch (e) {
            error = describe(e);
        } finally {
            loading = false;
        }
    }

    async function registerKey(event: SubmitEvent) {
        event.preventDefault();
        error = '';
        if (!code.trim()) {
            error = "Entre le code d'installation.";
            return;
        }
        loading = true;
        try {
            const { options } = await post('/japprends/passkey/register-options', { code });
            const response = await startRegistration({ optionsJSON: options });
            await post('/japprends/passkey/register-verify', { response, name: keyName });
            await goto('/japprends/dashboard');
        } catch (e) {
            error = describe(e);
        } finally {
            loading = false;
        }
    }

    async function logout() {
        await fetch('/auth/logout', { method: 'POST' });
        // Navigation complète : sous le nom de la console, `/` renvoie au portail
        location.href = '/';
    }
</script>

<svelte:head>
    <title>Console admin — rev0auth</title>
</svelte:head>

<main class="admin-bg">
    <div class="card">
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>

        <h1>Console admin</h1>
        <p class="hint">Connecté en tant que <strong>{data.pseudo}</strong></p>

        {#if mode === 'key'}
            <p class="hint">Branche ta clé de sécurité, puis touche-la quand elle clignote.</p>
            <button class="btn" type="button" onclick={useKey} disabled={loading}>
                {loading ? 'En attente de la clé…' : 'Utiliser ma clé'}
            </button>
            <button class="text-btn" type="button" onclick={() => { mode = 'setup'; error = ''; }}>
                Clé perdue ? Utiliser un code d'installation
            </button>
        {:else}
            <form class="fields" onsubmit={registerKey}>
                <p class="hint">Entre le code d'installation généré sur le serveur, puis touche ta nouvelle clé.</p>
                <input class="field" type="text" placeholder="Code d'installation" autocomplete="one-time-code"
                       aria-label="Code d'installation" bind:value={code} />
                <input class="field" type="text" placeholder="Nom de la clé (ex. YubiKey 5)" maxlength="60"
                       aria-label="Nom de la clé" bind:value={keyName} />
                <button class="btn" type="submit" disabled={loading}>
                    {loading ? 'En attente de la clé…' : 'Enregistrer ma clé'}
                </button>
            </form>
            {#if data.hasKeys}
                <button class="text-btn" type="button" onclick={() => { mode = 'key'; error = ''; }}>
                    J'ai ma clé
                </button>
            {/if}
        {/if}

        {#if error}
            <p class="chip-error" role="alert">{error}</p>
        {/if}

        <button class="text-btn" type="button" onclick={logout}>Se déconnecter</button>
    </div>
</main>

<style>
    .admin-bg {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 1.5rem;
    }

    .card {
        width: 100%;
        max-width: 360px;
        background: var(--card);
        border: 1px solid var(--border);
        border-radius: var(--radius-lg);
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
        text-align: center;
    }

    .brand-badge {
        display: inline-block;
        font-size: 0.75rem;
        font-weight: 600;
        letter-spacing: 0.06em;
        text-transform: uppercase;
        background: var(--muted);
        color: var(--muted-foreground);
        padding: 3px 10px;
        border-radius: 999px;
    }

    h1 {
        font-size: 1.25rem;
        font-weight: 700;
        margin: 0;
        color: var(--foreground);
    }

    .fields {
        display: flex;
        flex-direction: column;
        gap: 0.625rem;
    }

    .field {
        width: 100%;
        padding: 0.6rem 0.75rem;
        background: var(--background);
        color: var(--foreground);
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        font-size: 0.875rem;
    }
    .field:focus { outline: none; border-color: var(--primary); }

    .btn {
        width: 100%;
        padding: 0.6rem 1rem;
        background: var(--primary);
        color: var(--primary-foreground);
        border: none;
        border-radius: var(--radius-sm);
        font-size: 0.875rem;
        font-weight: 600;
        cursor: pointer;
    }
    .btn:hover:not(:disabled) { opacity: 0.9; }
    .btn:disabled { opacity: 0.6; cursor: default; }

    .hint {
        margin: 0;
        font-size: 0.875rem;
        color: var(--muted-foreground);
    }

    .text-btn {
        background: none;
        border: none;
        padding: 0;
        font-size: 0.8125rem;
        color: var(--muted-foreground);
        text-decoration: underline;
        text-underline-offset: 3px;
        cursor: pointer;
    }
    .text-btn:hover { color: var(--foreground); }
</style>
