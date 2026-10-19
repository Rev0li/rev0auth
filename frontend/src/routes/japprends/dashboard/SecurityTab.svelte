<script lang="ts">
    import { startRegistration } from '@simplewebauthn/browser';

    // Clés de sécurité qui ouvrent la console. Une clé de secours s'ajoute
    // depuis cette session déjà validée ; une clé perdue se retire ici, et se
    // remplace avec un code d'installation généré sur le serveur.
    type Key = { id: number; name: string; createdAt: string; lastUsedAt: string | null };

    let keys = $state<Key[]>([]);
    let loading = $state(true);
    let name = $state('');
    let adding = $state(false);
    let error = $state('');
    let confirmId = $state<number | null>(null);
    let busy = $state<Record<number, boolean>>({});

    function fmt(iso: string) {
        return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' });
    }

    async function load() {
        loading = true;
        try {
            const r = await fetch('/japprends/passkey');
            keys = await r.json();
        } finally { loading = false; }
    }

    async function post(url: string, body: unknown) {
        const r = await fetch(url, {
            method: 'POST',
            headers: { 'content-type': 'application/json' },
            body: JSON.stringify(body),
        });
        const d = await r.json().catch(() => ({}));
        if (!r.ok || d.ok === false) throw new Error(d.message ?? 'Erreur.');
        return d;
    }

    async function add() {
        error = '';
        adding = true;
        try {
            const { options } = await post('/japprends/passkey/register-options', {});
            const response = await startRegistration({ optionsJSON: options });
            await post('/japprends/passkey/register-verify', { response, name });
            name = '';
            await load();
        } catch (e) {
            error = e instanceof Error && e.name === 'NotAllowedError'
                ? 'Opération annulée ou clé non détectée.'
                : e instanceof Error ? e.message : 'Erreur.';
        } finally { adding = false; }
    }

    async function remove(id: number) {
        busy[id] = true;
        try {
            const r = await fetch(`/japprends/passkey/${id}`, { method: 'DELETE' });
            if (r.ok) keys = keys.filter(k => k.id !== id);
        } finally {
            busy[id] = false;
            confirmId = null;
        }
    }

    $effect(() => { load(); });
</script>

<div class="security-tab">
    <div class="tab-header">
        <h2>Clés de sécurité</h2>
    </div>

    <div class="add-form">
        <input bind:value={name} placeholder="Nom de la clé (ex. YubiKey de secours)" maxlength="60" aria-label="Nom de la clé" />
        <button class="btn-add" onclick={add} disabled={adding}>
            {adding ? 'En attente de la clé…' : '+ Ajouter une clé'}
        </button>
    </div>
    {#if error}<p class="error" role="alert">{error}</p>{/if}

    {#if loading}
        <p class="loading">Chargement…</p>
    {:else}
        {#if keys.length === 1}
            <p class="hint">Une seule clé enregistrée : ajoute une clé de secours si tu en as une deuxième.</p>
        {/if}
        <div class="key-list">
            {#each keys as key (key.id)}
                <div class="key-row">
                    <div class="key-main">
                        <span class="key-name">{key.name}</span>
                        <span class="key-meta">
                            ajoutée le {fmt(key.createdAt)}
                            · {key.lastUsedAt ? `utilisée le ${fmt(key.lastUsedAt)}` : 'jamais utilisée'}
                        </span>
                    </div>
                    <div class="key-actions">
                        {#if confirmId === key.id}
                            <button class="btn-confirm" disabled={busy[key.id]} onclick={() => remove(key.id)}>Oui, retirer</button>
                            <button class="btn-cancel" onclick={() => (confirmId = null)}>Annuler</button>
                        {:else}
                            <button class="btn-del" aria-label="Retirer la clé {key.name}" onclick={() => (confirmId = key.id)}>✕</button>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .security-tab { display: flex; flex-direction: column; gap: 1rem; }

    .tab-header h2 { margin: 0; font-size: 1rem; font-weight: 600; }

    .add-form {
        display: flex; gap: 0.5rem; flex-wrap: wrap;
        padding: 0.75rem; background: var(--muted);
        border: 1px solid var(--border); border-radius: var(--radius-md);
    }
    .add-form input { flex: 1; min-width: 160px; }
    .btn-add {
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md); padding: 8px 16px;
        font: 500 0.875rem/1 var(--font-sans); cursor: pointer; white-space: nowrap;
    }
    .btn-add:disabled { opacity: 0.4; cursor: not-allowed; }

    .error { margin: 0; font-size: 0.875rem; color: var(--destructive); }
    .hint { margin: 0; font-size: 0.8125rem; color: var(--muted-foreground); }
    .loading { color: var(--muted-foreground); font-size: 0.875rem; text-align: center; margin: 2rem 0; }

    .key-list { display: flex; flex-direction: column; gap: 0.5rem; }
    .key-row {
        display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 0.5rem;
        padding: 0.75rem 1rem; border: 1px solid var(--border);
        border-radius: var(--radius-md); background: var(--card);
    }
    .key-main { display: flex; flex-direction: column; gap: 2px; }
    .key-name { font-weight: 600; font-size: 0.875rem; }
    .key-meta { font-size: 0.8125rem; color: var(--muted-foreground); }

    .key-actions { display: flex; align-items: center; gap: 0.4rem; }
    .btn-del {
        background: none; border: none; color: var(--muted-foreground);
        cursor: pointer; font-size: 0.75rem; padding: 4px 8px;
        border-radius: var(--radius-sm);
    }
    .btn-del:hover { background: var(--destructive-bg); color: var(--destructive); }
    .btn-confirm, .btn-cancel {
        font: 500 0.8125rem/1 var(--font-sans);
        border-radius: var(--radius-sm); padding: 4px 10px; cursor: pointer;
    }
    .btn-confirm { background: var(--destructive); color: #fff; border: 1px solid var(--destructive); }
    .btn-confirm:disabled { opacity: 0.4; cursor: not-allowed; }
    .btn-cancel { background: var(--muted); color: var(--foreground); border: 1px solid var(--border); }
</style>
