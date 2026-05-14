<script lang="ts">
    import { browser } from '$app/environment';

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
    onclick={toggle}
    title="Basculer le thème"
    aria-label="Basculer clair / sombre"
>
    {isDark ? '☀' : '☾'}
</button>

<style>
    .theme-toggle {
        position: fixed;
        bottom: 20px;
        left: 20px;
        z-index: 9999;
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
    .theme-toggle:hover {
        background: var(--muted);
        box-shadow: var(--shadow-hover);
        transform: scale(1.08);
    }
</style>
