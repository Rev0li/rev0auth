<script lang="ts">
    import { splitEmojis } from './emojis.js';

    let { text }: { text: string } = $props();

    let parts = $derived(splitEmojis(text));
</script>

{#each parts as part, i (i)}
    {#if 'emoji' in part}
        <img class="openmoji" src={part.emoji.src} alt={part.emoji.char} title={part.emoji.name} />
    {:else}
        {part.text}
    {/if}
{/each}

<style>
    .openmoji {
        width: 1.35em;
        height: 1.35em;
        vertical-align: -0.3em;
        display: inline-block;
    }
</style>
