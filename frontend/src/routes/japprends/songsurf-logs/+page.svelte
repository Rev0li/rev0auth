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
    .songsurf-logs { max-width: 1100px; margin: 2rem auto; padding: 0 1rem; }
    header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem; }
    h1 { margin: 0; font-size: 1.4rem; }
    .filters { display: flex; gap: 1rem; align-items: end; margin-bottom: 1rem; flex-wrap: wrap; }
    .filters label { display: flex; flex-direction: column; font-size: .85rem; color: #555; gap: .25rem; }
    .filters input { padding: .4rem .6rem; border: 1px solid #ccc; border-radius: 4px; }
    .filters button { padding: .5rem 1rem; cursor: pointer; }
    .error { background: #fee; border: 1px solid #fcc; padding: .75rem; border-radius: 4px; color: #900; }
    .output { background: #f7f7f7; border: 1px solid #ddd; padding: 1rem; border-radius: 4px; max-height: 600px; overflow: auto; font-size: .8rem; }
</style>
