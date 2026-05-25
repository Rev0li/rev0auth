<script lang="ts">
    import { goto } from '$app/navigation';
    import { onMount } from 'svelte';

    let waiting = $state(true);
    let result  = $state<{ ok: boolean; msg: string } | null>(null);

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
            result = { ok: false, msg: 'WebAuthn non configuré sur ce serveur.' };
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
            if (resp.ok) setTimeout(() => goto('/japprends/dashboard'), 350);
        } catch (err) {
            waiting = false;
            result = { ok: false, msg: (err as Error).message ?? 'Erreur WebAuthn.' };
        }
    }

    onMount(() => { startWebAuthn(); });
</script>

<main class="admin-bg">
    <div class="card">
        <div class="brand">
            <span class="brand-badge">rev0auth admin</span>
        </div>

        <h1>Admin Access</h1>
        <p class="hint-key">Touche ta YubiKey pour te connecter</p>

        {#if waiting && !result}
            <div class="spinner-row">
                <span class="spinner"></span>
                <span class="hint">En attente…</span>
            </div>
        {/if}

        {#if result}
            <p class="result" class:ok={result.ok} class:err={!result.ok}>{result.msg}</p>
            {#if !result.ok}
                <button class="btn" onclick={startWebAuthn}>Réessayer</button>
            {/if}
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

    .hint-key {
        font-size: 0.875rem;
        color: var(--muted-foreground);
        margin: 0;
    }

    .spinner-row {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.625rem;
    }
    .spinner {
        width: 16px; height: 16px;
        border: 2px solid var(--border);
        border-top-color: var(--primary);
        border-radius: 50%;
        animation: spin 0.7s linear infinite;
        flex-shrink: 0;
    }
    @keyframes spin { to { transform: rotate(360deg); } }
    .hint { font-size: 0.875rem; color: var(--muted-foreground); }

    .result {
        font-size: 0.875rem;
        padding: 8px 12px;
        border-radius: var(--radius-sm);
        margin: 0;
    }
    .result.ok  { color: hsl(142 71% 45%); background: hsl(142 71% 45% / 0.12); }
    .result.err { color: hsl(0 72% 51%);   background: hsl(0 72% 51% / 0.1); }

    .btn {
        width: 100%;
        padding: 0.6rem 1rem;
        background: var(--muted);
        color: var(--foreground);
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
    }
    .btn:hover { border-color: var(--primary); }
</style>
