<script lang="ts">
    import Checkbox from '../primitives/Checkbox.svelte';
    import Typeface from '../primitives/Typeface.svelte';

    export let mdns: string;
    export let ip: string;
    export let isSelected: boolean = false;
    export let onSelect: (bridge: [string, string]) => void;
</script>

<li>
    <button
        class="w-full flex flex-row items-center px-2 py-1 rounded-md hover:bg-ink-600
cursor-pointer transition-colors"
        on:click={() => onSelect([mdns, ip])}
    >
        <Typeface>{ip} ({mdns})</Typeface>
        <Checkbox checked={isSelected} class="ml-auto" />
    </button>
</li>
