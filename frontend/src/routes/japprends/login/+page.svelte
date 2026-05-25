<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';

    // WebAuthn mode (YubiKey) — auto-starts on load.
    // Falls back to seed+password form if WebAuthn fails or user prefers it.

    type Mode = 'webauthn' | 'password';
    let mode     = $state<Mode>('webauthn');
    let waiting  = $state(true);
    let result   = $state<{ ok: boolean; msg: string } | null>(null);

    // ── WebAuthn helpers ────────────────────────────────────────────────
    function b64ToBuffer(b64: string): ArrayBuffer {
        const pad = (b64 + '===').slice(0, b64.length + (4 - b64.length % 4) % 4)
            .replace(/-/g, '+').replace(/_/g, '/');
        const bin = atob(pad);
        const buf = new Uint8Array(bin.length);
        for (let i = 0; i < bin.length; i++) buf[i] = bin.charCodeAt(i);
        return buf.buffer;
    }
    function bufToB64(buf: ArrayBuffer): string {
        const bytes = new Uint8Array(buf);
        let str = '';
        for (const b of bytes) str += String.fromCharCode(b);
        return btoa(str).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
    }

    async function startWebAuthn() {
        waiting = true;
        result = null;

        let data: Record<string, unknown>;
        try {
            const res = await fetch('/japprends/webauthn/auth/start');
            data = await res.json() as Record<string, unknown>;
        } catch {
            waiting = false;
            result = { ok: false, msg: 'Erreur réseau. Recharge la page.' };
            return;
        }

        if (data.locked) {
            waiting = false;
            result = { ok: false, msg: (data.message as string) || 'Trop de tentatives. Réessaie plus tard.' };
            return;
        }

        if (!data.webauthn_required) {
            waiting = false;
            result = { ok: false, msg: 'Authentification non disponible. Recharge la page.' };
            return;
        }

        try {
            const opts = (data.webauthn_challenge as { publicKey: PublicKeyCredentialRequestOptions }).publicKey;
            opts.challenge = b64ToBuffer(opts.challenge as unknown as string);
            if (opts.allowCredentials) {
                opts.allowCredentials = opts.allowCredentials.map(c => ({
                    ...c,
                    id: b64ToBuffer(c.id as unknown as string),
                }));
            }

            const cred = await navigator.credentials.get({ publicKey: opts }) as PublicKeyCredential;
            waiting = false;

            const r = cred.response as AuthenticatorAssertionResponse;
            const payload = {
                token: data.webauthn_token,
                credential: {
                    id: cred.id,
                    rawId: bufToB64(cred.rawId),
                    type: cred.type,
                    response: {
                        authenticatorData: bufToB64(r.authenticatorData),
                        clientDataJSON:    bufToB64(r.clientDataJSON),
                        signature:         bufToB64(r.signature),
                        userHandle: r.userHandle ? bufToB64(r.userHandle) : null,
                    },
                },
            };

            const finishRes = await fetch('/japprends/webauthn/auth/finish', {
                method:  'POST',
                headers: { 'content-type': 'application/json' },
                body:    JSON.stringify(payload),
            });
            const resp = await finishRes.json() as { ok: boolean; message?: string };
            result = { ok: resp.ok, msg: resp.message ?? (resp.ok ? 'Connexion réussie.' : 'Échec.') };
            if (resp.ok) setTimeout(() => goto('/japprends/tdd'), 350);
        } catch (err) {
            waiting = false;
            result = { ok: false, msg: (err as Error).message ?? 'Erreur WebAuthn.' };
        }
    }

    onMount(() => { startWebAuthn(); });

    // ── Password form ───────────────────────────────────────────────────
    let pseudo    = $state('');
    let seed      = $state('');
    let pwd       = $state('');
    let otp       = $state('');
    let pwdLoading = $state(false);
    let pwdError   = $state('');

    async function loginPassword() {
        pwdError = '';
        pwdLoading = true;
        try {
            const r = await fetch('/japprends/login', {
                method:  'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify({
                    pseudo,
                    seed,
                    password:        pwd,
                    otp:             otp || undefined,
                    challenge_choice: 'secure-lock',
                    trap_value:      '',
                }),
            });
            const d = await r.json() as { ok: boolean; message?: string };
            if (d.ok) goto('/japprends/tdd');
            else pwdError = d.message ?? 'Identifiants invalides.';
        } catch { pwdError = 'Erreur réseau.'; }
        finally { pwdLoading = false; }
    }
</script>

<main class="admin-bg">
    <div class="card">
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>

        {#if mode === 'webauthn'}
            <h1>Admin Access</h1>

            {#if waiting}
                <p class="hint">Attente de connexion YubiKey…</p>
            {/if}

            {#if result}
                <p class="result" class:ok={result.ok} class:err={!result.ok}>{result.msg}</p>
                {#if !result.ok}
                    <button class="btn retry-btn" onclick={startWebAuthn}>Réessayer</button>
                {/if}
            {/if}

            <button class="link-btn" onclick={() => { mode = 'password'; }}>
                Utiliser un mot de passe
            </button>

        {:else}
            <h1>Admin — Mot de passe</h1>

            <div class="field">
                <label for="pseudo">Pseudo admin</label>
                <input id="pseudo" type="text" bind:value={pseudo} autocomplete="username" />
            </div>
            <div class="field">
                <label for="seed">Seed</label>
                <input id="seed" type="text" bind:value={seed} autocomplete="off" />
            </div>
            <div class="field">
                <label for="pwd">Mot de passe</label>
                <input id="pwd" type="password" bind:value={pwd} autocomplete="current-password" />
            </div>
            <div class="field">
                <label for="otp">Code 2FA <span class="optional">(si activé)</span></label>
                <input id="otp" type="text" bind:value={otp} autocomplete="one-time-code" maxlength="6" />
            </div>

            {#if pwdError}<p class="result err">{pwdError}</p>{/if}

            <button class="btn" disabled={pwdLoading} onclick={loginPassword}>
                {pwdLoading ? '…' : 'Se connecter'}
            </button>

            <button class="link-btn" onclick={() => { mode = 'webauthn'; startWebAuthn(); }}>
                ← Utiliser la YubiKey
            </button>
        {/if}
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
        max-width: 380px;
        background: var(--card);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .brand { text-align: center; }
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
        text-align: center;
        margin: 0;
        color: var(--foreground);
    }

    .hint {
        text-align: center;
        font-size: 0.875rem;
        color: var(--muted-foreground);
        margin: 0;
    }

    .result {
        text-align: center;
        font-size: 0.875rem;
        padding: 8px 12px;
        border-radius: var(--radius-sm);
        margin: 0;
    }
    .result.ok  { color: hsl(142 71% 45%); background: hsl(142 71% 45% / 0.12); }
    .result.err { color: hsl(0 72% 51%);   background: hsl(0 72% 51% / 0.1); }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
    }
    .field label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--foreground);
    }
    .field input {
        padding: 0.5rem 0.75rem;
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        background: var(--background);
        color: var(--foreground);
        font-size: 0.875rem;
        outline: none;
    }
    .field input:focus { border-color: var(--ring); }
    .optional { color: var(--muted-foreground); font-weight: 400; }

    .btn {
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
    .btn:disabled { opacity: 0.6; cursor: default; }
    .btn:not(:disabled):hover { opacity: 0.9; }

    .retry-btn {
        background: var(--muted);
        color: var(--muted-foreground);
    }

    .link-btn {
        background: none;
        border: none;
        color: var(--muted-foreground);
        font-size: 0.83rem;
        cursor: pointer;
        text-align: center;
        padding: 0;
        text-decoration: underline;
        text-underline-offset: 2px;
    }
    .link-btn:hover { color: var(--foreground); }
</style>
