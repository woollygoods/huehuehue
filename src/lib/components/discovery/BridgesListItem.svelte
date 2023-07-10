<script lang="ts">
    import Typeface from '../primitives/Typeface.svelte';

    export let mdns: string;
    export let ip: string;
</script>

<li
    class="flex flex-row px-2 py-1 rounded-md hover:bg-ink-600 cursor-pointer transition-colors"
>
    <Typeface>{ip} ({mdns})</Typeface>
</li>
