<script lang="ts">
    import { goto } from '$app/navigation';

    // Login admin par mot de passe (POST /japprends/login).
    // Le mode YubiKey (proxy WebAuthn vers crates/web) a été retiré avec le
    // Rust web — les passkeys reviendront via le plugin BetterAuth.
    let pseudo    = $state('');
    let seed      = $state('');
    let password  = $state('');
    let otp       = $state('');
    let challenge = $state('');   // doit valoir 'secure-lock'
    let trap      = $state('');   // honeypot — doit rester vide
    let loading   = $state(false);
    let error     = $state('');

    const CHALLENGE_ICONS = [
        { value: 'master-key',  icon: '🔑' },
        { value: 'secure-lock', icon: '🔒' },
        { value: 'iron-shield', icon: '🛡️' },
    ];

    async function login(e: SubmitEvent) {
        e.preventDefault();
        error = '';
        if (!pseudo.trim() || !seed.trim() || !password) {
            error = 'Tous les champs sont requis.';
            return;
        }
        if (!challenge) {
            error = 'Choisis un symbole.';
            return;
        }

        loading = true;
        try {
            const res = await fetch('/japprends/login', {
                method:  'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo: pseudo.trim(),
                    seed:   seed.trim(),
                    password,
                    otp:    otp.trim() || undefined,
                    challenge_choice: challenge,
                    trap_value: trap,
                }),
            });
            const data = await res.json() as { ok: boolean; message?: string };
            if (!data.ok) {
                error = data.message ?? 'Connexion refusée.';
                return;
            }
            goto('/japprends/dashboard');
        } catch {
            error = 'Erreur réseau.';
        } finally {
            loading = false;
        }
    }
</script>

<main class="admin-bg">
    <form class="card" onsubmit={login}>
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>

        <h1>Admin Access</h1>

        <div class="fields">
            <input class="field" type="text" placeholder="Pseudo" autocomplete="username" bind:value={pseudo} />
            <input class="field" type="password" placeholder="Seed" autocomplete="off" bind:value={seed} />
            <input class="field" type="password" placeholder="Mot de passe" autocomplete="current-password" bind:value={password} />
            <input class="field" type="text" inputmode="numeric" placeholder="Code 2FA (si activé)" autocomplete="one-time-code" bind:value={otp} />
        </div>

        <!-- Honeypot : invisible pour un humain, rempli par les bots -->
        <input class="trap" type="text" name="website" tabindex="-1" autocomplete="off" bind:value={trap} />

        <div class="challenge-row" role="radiogroup" aria-label="Vérification">
            {#each CHALLENGE_ICONS as c (c.value)}
                <button
                    type="button"
                    class="challenge-btn"
                    class:selected={challenge === c.value}
                    onclick={() => { challenge = c.value; }}
                    aria-pressed={challenge === c.value}
                >{c.icon}</button>
            {/each}
        </div>

        {#if error}
            <p class="result err">{error}</p>
        {/if}

        <button class="btn" type="submit" disabled={loading}>
            {loading ? 'Connexion…' : 'Se connecter'}
        </button>
    </form>
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

    .trap {
        position: absolute;
        left: -9999px;
        width: 1px;
        height: 1px;
        opacity: 0;
    }

    .challenge-row {
        display: flex;
        justify-content: center;
        gap: 0.75rem;
    }

    .challenge-btn {
        font-size: 1.25rem;
        padding: 0.5rem 0.875rem;
        background: var(--muted);
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        cursor: pointer;
        transition: border-color 0.15s;
    }
    .challenge-btn:hover    { border-color: var(--primary); }
    .challenge-btn.selected { border-color: var(--primary); background: var(--background); }

    .result {
        font-size: 0.875rem;
        padding: 8px 12px;
        border-radius: var(--radius-sm);
        margin: 0;
    }
    .result.err { color: hsl(0 72% 51%); background: hsl(0 72% 51% / 0.1); }

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
</style>
