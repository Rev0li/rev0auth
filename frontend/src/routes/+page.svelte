<script lang="ts">
    import { goto } from '$app/navigation';
    import { slide, fade } from 'svelte/transition';

    type Step = 'pseudo' | 'password' | 'admin';
    type Tab  = 'login'  | 'register';

    let tab  = $state<Tab>('login');
    let step = $state<Step>('pseudo');

    // Login state
    let pseudo       = $state('');
    let password     = $state('');
    let seed         = $state('');
    let otp          = $state('');
    let challenge    = $state('');
    let totpEnabled  = $state(false);
    let loading      = $state(false);
    let error        = $state('');

    // Register state
    let regPseudo    = $state('');
    let regReferral  = $state('');
    let regLoading   = $state(false);
    let regMsg       = $state('');
    let regOk        = $state(false);

    const EMOJIS = [
        { id: 'spark',       label: '✨ Spark'  },
        { id: 'rocket',      label: '🚀 Rocket' },
        { id: 'secure-lock', label: '🔒 Verrou' },
        { id: 'sun',         label: '☀ Soleil'  },
        { id: 'moon',        label: '🌙 Lune'   },
        { id: 'star',        label: '⭐ Étoile' },
    ];

    async function checkPseudo() {
        error = '';
        const key = pseudo.trim().toLowerCase();
        if (!key) { error = 'Entre ton pseudo.'; return; }
        loading = true;
        try {
            const r = await fetch('/portal/login', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: key }),
            });
            const data = await r.json();
            if (!data.ok) { error = data.message; return; }
            if (data.state === 'admin') { totpEnabled = data.totpEnabled; step = 'admin'; }
            else step = 'password';
        } catch { error = 'Erreur réseau.'; }
        finally { loading = false; }
    }

    async function loginMember() {
        error = '';
        loading = true;
        try {
            const r = await fetch('/auth/password-check', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: pseudo.trim().toLowerCase(), password }),
            });
            const data = await r.json();
            if (!data.ok) { error = data.message; return; }
            if (data.state === 'onboarding') goto('/home/friend?onboarding=1');
            else goto('/home/friend');
        } catch { error = 'Erreur réseau.'; }
        finally { loading = false; }
    }

    async function loginAdmin() {
        error = '';
        if (!challenge) { error = 'Sélectionne le bon emoji.'; return; }
        loading = true;
        try {
            const r = await fetch('/japprends/login', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo: pseudo.trim().toLowerCase(),
                    seed,
                    password,
                    challenge_choice: challenge,
                    otp: otp || undefined,
                    trap_value: '',
                }),
            });
            const data = await r.json();
            if (!data.ok) { error = data.message; return; }
            goto('/japprends/tdd');
        } catch { error = 'Erreur réseau.'; }
        finally { loading = false; }
    }

    async function register() {
        regMsg = ''; regOk = false;
        const key = regPseudo.trim().toLowerCase();
        if (!key) { regMsg = 'Pseudo requis.'; return; }
        regLoading = true;
        try {
            const r = await fetch('/portal', {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({ pseudo: key, referral: regReferral.trim() || null }),
            });
            const data = await r.json();
            regOk = data.ok;
            regMsg = data.message;
        } catch { regMsg = 'Erreur réseau.'; }
        finally { regLoading = false; }
    }

    function onEnter(e: KeyboardEvent, fn: () => void) {
        if (e.key === 'Enter') fn();
    }
</script>

<main class="portal-bg">
    <div class="card portal-card">

        <!-- Logo -->
        <div class="portal-logo">
            <span class="portal-brand">rev0auth</span>
            <span class="portal-tagline">Espace privé</span>
        </div>

        <!-- Tabs -->
        <div class="tabs" role="tablist">
            <button class="tab" class:active={tab === 'login'}    role="tab" onclick={() => { tab = 'login';    step = 'pseudo'; error = ''; }}>Connexion</button>
            <button class="tab" class:active={tab === 'register'} role="tab" onclick={() => { tab = 'register'; error = ''; }}>S'inscrire</button>
        </div>

        <!-- ── LOGIN ── -->
        {#if tab === 'login'}
            <div transition:fade={{ duration: 150 }}>

                {#if step === 'pseudo'}
                    <div transition:slide={{ duration: 200 }}>
                        <div class="field">
                            <label for="pseudo">Pseudo</label>
                            <input id="pseudo" bind:value={pseudo} placeholder="ton_pseudo"
                                onkeydown={(e) => onEnter(e, checkPseudo)} />
                        </div>
                        {#if error}<p class="chip-error">{error}</p>{/if}
                        <button class="btn-primary btn-full" onclick={checkPseudo} disabled={loading}>
                            {loading ? '…' : 'Suivant →'}
                        </button>
                        <a class="link" href="/portal">Pas encore de compte ? S'inscrire</a>
                    </div>

                {:else if step === 'password'}
                    <div transition:slide={{ duration: 200 }}>
                        <button class="back-btn" onclick={() => { step = 'pseudo'; error = ''; password = ''; }}>← {pseudo}</button>
                        <div class="field">
                            <label for="password">Mot de passe</label>
                            <input id="password" type="password" bind:value={password} placeholder="••••••••"
                                onkeydown={(e) => onEnter(e, loginMember)} />
                        </div>
                        {#if error}<p class="chip-error">{error}</p>{/if}
                        <button class="btn-primary btn-full" onclick={loginMember} disabled={loading}>
                            {loading ? '…' : 'Se connecter'}
                        </button>
                    </div>

                {:else if step === 'admin'}
                    <div transition:slide={{ duration: 200 }}>
                        <button class="back-btn" onclick={() => { step = 'pseudo'; error = ''; }}>← {pseudo}</button>
                        <p class="admin-hint">Accès admin</p>

                        <div class="field">
                            <label for="seed">Seed</label>
                            <input id="seed" bind:value={seed} placeholder="seed secret" />
                        </div>
                        <div class="field">
                            <label for="adm-pwd">Mot de passe</label>
                            <input id="adm-pwd" type="password" bind:value={password} placeholder="••••••••" />
                        </div>

                        <div class="field">
                            <p class="challenge-label">Challenge — Clique uniquement sur 🔒</p>
                            <div class="emoji-grid" role="group" aria-label="Challenge de sécurité">
                                {#each EMOJIS as e}
                                    <button
                                        class="emoji-btn"
                                        class:selected={challenge === e.id}
                                        onclick={() => challenge = e.id}
                                        type="button"
                                    >{e.label}</button>
                                {/each}
                            </div>
                        </div>

                        {#if totpEnabled}
                            <div class="field">
                                <label for="otp">Code 2FA</label>
                                <input id="otp" bind:value={otp} placeholder="123456" maxlength={6}
                                    onkeydown={(e) => onEnter(e, loginAdmin)} />
                            </div>
                        {/if}

                        {#if error}<p class="chip-error">{error}</p>{/if}
                        <button class="btn-primary btn-full admin-btn" onclick={loginAdmin} disabled={loading}>
                            {loading ? '…' : 'Connexion admin 🔑'}
                        </button>

                        <!-- honeypot — invisible -->
                        <input style="display:none" tabindex="-1" aria-hidden="true" autocomplete="off" name="trap" />
                    </div>
                {/if}

            </div>
        {/if}

        <!-- ── REGISTER ── -->
        {#if tab === 'register'}
            <div transition:fade={{ duration: 150 }}>
                {#if regOk}
                    <div class="chip-ok register-ok">
                        ✓ Demande envoyée ! On revient vers toi dès que c'est approuvé.
                    </div>
                {:else}
                    <div class="field">
                        <label for="reg-pseudo">Pseudo souhaité</label>
                        <input id="reg-pseudo" bind:value={regPseudo} placeholder="ton_pseudo" />
                    </div>
                    <div class="field">
                        <label for="reg-ref">Référent <span class="meta">(optionnel)</span></label>
                        <input id="reg-ref" bind:value={regReferral} placeholder="pseudo de quelqu'un qui te connaît" />
                    </div>
                    {#if regMsg}<p class="chip-error">{regMsg}</p>{/if}
                    <button class="btn-primary btn-full" onclick={register} disabled={regLoading}>
                        {regLoading ? '…' : "Demander l'accès"}
                    </button>
                {/if}
            </div>
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
        max-width: 420px;
        padding: 2rem;
    }

    .portal-logo {
        text-align: center;
        margin-bottom: 1.75rem;
    }
    .portal-brand {
        display: block;
        font-size: 1.625rem;
        font-weight: 700;
        letter-spacing: -0.02em;
    }
    .portal-tagline {
        font-size: 0.8125rem;
        color: var(--muted-foreground);
    }

    /* Tabs */
    .tabs {
        display: flex;
        gap: 4px;
        background: var(--muted);
        border-radius: var(--radius-full);
        padding: 4px;
        margin-bottom: 1.5rem;
    }
    .tab {
        flex: 1;
        height: 34px;
        border: none;
        border-radius: var(--radius-full);
        font: 500 0.875rem/1 var(--font-sans);
        cursor: pointer;
        background: transparent;
        color: var(--muted-foreground);
        transition: background 0.15s, color 0.15s;
    }
    .tab.active {
        background: var(--card);
        color: var(--foreground);
        box-shadow: var(--shadow-soft);
    }

    /* Back button */
    .back-btn {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        background: none;
        border: none;
        color: var(--muted-foreground);
        font: 500 0.8125rem/1 var(--font-sans);
        cursor: pointer;
        padding: 0;
        margin-bottom: 1rem;
        transition: color 0.15s;
    }
    .back-btn:hover { color: var(--foreground); }

    /* Admin */
    .admin-hint {
        font-size: 0.8125rem;
        font-weight: 600;
        color: var(--muted-foreground);
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin: 0 0 1rem;
    }
    .challenge-label {
        display: block;
        margin: 0 0 5px;
        font-size: 0.8125rem;
        font-weight: 600;
    }
    .admin-btn { background: var(--accent); color: var(--accent-foreground); }
    .admin-btn:hover { background: color-mix(in srgb, var(--accent) 85%, black); }

    /* Emoji challenge */
    .emoji-grid {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        margin-top: 6px;
    }
    .emoji-btn {
        padding: 6px 10px;
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        background: var(--muted);
        font-size: 0.875rem;
        cursor: pointer;
        transition: border-color 0.15s, background 0.15s;
    }
    .emoji-btn.selected {
        border-color: var(--primary);
        background: var(--card);
        box-shadow: 0 0 0 3px var(--ring);
    }

    .register-ok {
        text-align: center;
        padding: 1.25rem;
        font-size: 0.9375rem;
    }
</style>
