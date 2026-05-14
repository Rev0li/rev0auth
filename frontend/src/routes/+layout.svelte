<script lang="ts">
    import '../app.css';
    import ThemeToggle from '$lib/ThemeToggle.svelte';

    let { children } = $props();
</script>

<svelte:head>
    <link rel="preconnect" href="https://fonts.bunny.net" />
    <link href="https://fonts.bunny.net/css?family=geist:400,500,600,700|geist-mono:400,500" rel="stylesheet" />
</svelte:head>

{@render children()}

<ThemeToggle />
