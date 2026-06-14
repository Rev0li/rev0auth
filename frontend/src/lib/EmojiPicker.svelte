<script lang="ts">
    import { EMOJIS } from './emojis.js';

    // Sélecteur d'emoji : un bouton smiley qui déploie une card flottante avec
    // la grille des emojis OpenMoji. onpick reçoit le caractère unicode (la
    // saisie reste du texte, rendu OpenMoji via EmojiText à l'affichage).
    let { onpick }: { onpick: (char: string) => void } = $props();

    let open = $state(false);
    let root: HTMLDivElement;

    // Smiley du déclencheur : le premier emoji (😀) en rendu OpenMoji
    const triggerSrc = EMOJIS[0].src;

    function pick(char: string) {
        onpick(char);
        open = false;
    }

    function onWindowClick(e: MouseEvent) {
        if (open && root && !root.contains(e.target as Node)) open = false;
    }

    function onKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') open = false;
    }
</script>

<svelte:window onclick={onWindowClick} onkeydown={onKeydown} />

<div class="emoji-picker" bind:this={root}>
    <button
        type="button"
        class="emoji-trigger"
        class:active={open}
        onclick={() => (open = !open)}
        aria-label="Insérer un emoji"
        aria-expanded={open}
        title="Emoji"
    >
        <img src={triggerSrc} alt="" />
    </button>

    {#if open}
        <div class="emoji-pop" role="menu">
            {#each EMOJIS as e (e.char)}
                <button
                    type="button"
                    class="emoji-opt"
                    role="menuitem"
                    onclick={() => pick(e.char)}
                    aria-label="Ajouter {e.name}"
                    title={e.name}
                >
                    <img src={e.src} alt={e.char} />
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .emoji-picker { position: relative; display: inline-flex; }

    .emoji-trigger {
        display: inline-flex; align-items: center; justify-content: center;
        width: 30px; height: 30px; padding: 0;
        border: 1px solid var(--border); border-radius: var(--radius-sm);
        background: var(--card); cursor: pointer;
        transition: background 0.12s, border-color 0.12s;
    }
    .emoji-trigger:hover, .emoji-trigger.active { background: var(--muted); border-color: var(--primary); }
    .emoji-trigger img { width: 20px; height: 20px; }

    .emoji-pop {
        position: absolute; bottom: calc(100% + 6px); left: 0; z-index: 50;
        display: grid; grid-template-columns: repeat(5, 1fr); gap: 2px;
        padding: 6px; width: max-content; max-width: 232px;
        background: var(--card); border: 1px solid var(--border);
        border-radius: var(--radius-md); box-shadow: var(--shadow-hover);
    }
    .emoji-opt {
        display: inline-flex; align-items: center; justify-content: center;
        width: 36px; height: 36px; padding: 0;
        background: none; border: none; border-radius: var(--radius-sm);
        cursor: pointer; transition: background 0.12s, transform 0.12s;
    }
    .emoji-opt:hover { background: var(--muted); transform: scale(1.15); }
    .emoji-opt img { width: 24px; height: 24px; }
</style>
