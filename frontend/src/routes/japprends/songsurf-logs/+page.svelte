<script lang="ts">
    let pseudo = $state('');
    let limit  = $state(100);
    let loading = $state(false);
    let error = $state('');
    let result = $state<unknown>(null);

    async function fetchLogs() {
        loading = true; error = ''; result = null;
        try {
            const url = `/japprends/songsurf-logs?pseudo=${encodeURIComponent(pseudo)}&limit=${limit}`;
            const r = await fetch(url);
            const d = await r.json();
            if (!r.ok || d.success === false) {
                error = d.error ?? `Erreur ${r.status}`;
            } else {
                result = d;
            }
        } catch (e) {
            error = (e as Error).message;
        } finally {
            loading = false;
        }
    }
</script>

<svelte:head><title>SongSurf logs — rev0auth</title></svelte:head>

<main class="songsurf-logs">
    <header>
        <h1>SongSurf — logs de téléchargement</h1>
        <a href="/japprends/dashboard">← Dashboard</a>
    </header>

    <form onsubmit={(e) => { e.preventDefault(); fetchLogs(); }} class="filters">
        <label>
            Pseudo (vide = tous)
            <input type="text" bind:value={pseudo} placeholder="alice" />
        </label>
        <label>
            Limit
            <input type="number" bind:value={limit} min="1" max="500" />
        </label>
        <button type="submit" disabled={loading}>
            {loading ? 'Chargement…' : 'Charger'}
        </button>
    </form>

    {#if error}
        <div class="error">⚠️ {error}</div>
    {/if}

    {#if result !== null}
        <pre class="output">{JSON.stringify(result, null, 2)}</pre>
    {/if}
</main>

<style>
    .songsurf-logs {
        max-width: 1100px; margin: 0 auto; padding: 1.5rem 1rem 3rem;
        color: var(--foreground);
    }
    header {
        display: flex; align-items: center; justify-content: space-between;
        margin-bottom: 1.5rem; padding-bottom: 1rem;
        border-bottom: 1px solid var(--border);
    }
    h1 { margin: 0; font-size: 1.4rem; font-weight: 700; }
    header a { color: var(--primary-hover); text-decoration: none; font-size: .8125rem; font-weight: 500; }
    header a:hover { text-decoration: underline; }

    .filters { display: flex; gap: 1rem; align-items: end; margin-bottom: 1rem; flex-wrap: wrap; }
    .filters label {
        display: flex; flex-direction: column; gap: .3rem;
        font-size: .75rem; font-weight: 600; color: var(--muted-foreground);
    }
    .filters input {
        padding: .45rem .6rem;
        border: 1px solid var(--border); border-radius: var(--radius-md);
        background: var(--background); color: var(--foreground);
        font: 400 0.875rem/1 var(--font-sans);
    }
    .filters input:focus { outline: none; border-color: var(--primary); }
    .filters button {
        padding: .55rem 1.1rem; cursor: pointer;
        background: var(--primary); color: var(--primary-foreground);
        border: none; border-radius: var(--radius-md);
        font: 500 0.875rem/1 var(--font-sans);
    }
    .filters button:disabled { opacity: .5; cursor: not-allowed; }

    .error {
        background: var(--destructive-bg); border: 1px solid var(--destructive-border);
        color: var(--destructive); padding: .75rem; border-radius: var(--radius-md);
    }
    .output {
        background: var(--muted); border: 1px solid var(--border);
        color: var(--foreground); padding: 1rem; border-radius: var(--radius-md);
        max-height: 600px; overflow: auto;
        font: 400 0.8125rem/1.5 var(--font-mono, monospace);
        white-space: pre; tab-size: 2;
    }
</style>
