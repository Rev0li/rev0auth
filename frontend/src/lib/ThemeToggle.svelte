<script lang="ts">
    import { browser } from '$app/environment';

    // inline : rendu dans le flux (navbar) au lieu du bouton flottant global
    let { inline = false }: { inline?: boolean } = $props();

    let isDark = $state(false);

    if (browser) {
        isDark = document.documentElement.classList.contains('dark');
    }

    function toggle() {
        isDark = !isDark;
        document.documentElement.classList.toggle('dark', isDark);
        document.documentElement.classList.toggle('light', !isDark);
        localStorage.setItem('rev0auth_color_scheme', isDark ? 'dark' : 'light');
    }
</script>

<button
    class="theme-toggle"
    class:floating={!inline}
    class:inline
    onclick={toggle}
    title="Basculer le thème"
    aria-label="Basculer clair / sombre"
>
    {isDark ? '☀' : '☾'}
</button>

<style>
    .theme-toggle {
        width: 36px;
        height: 36px;
        border-radius: 50%;
        border: 1px solid var(--border);
        background: var(--card);
        color: var(--foreground);
        font-size: 1rem;
        line-height: 1;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: var(--shadow-soft);
        transition: background 0.15s, box-shadow 0.15s, transform 0.12s;
        padding: 0;
    }
    .theme-toggle.floating {
        position: fixed;
        bottom: 20px;
        left: 20px;
        z-index: 9999;
    }
    .theme-toggle.inline {
        width: 30px;
        height: 30px;
        font-size: 0.875rem;
        box-shadow: none;
    }
    .theme-toggle:hover {
        background: var(--muted);
        box-shadow: var(--shadow-hover);
        transform: scale(1.08);
    }
</style>
