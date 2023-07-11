<script lang="ts">
    import BridgesListItem from './BridgesListItem.svelte';

    export let bridges: [string, string][];
    export let selected: [string, string] | undefined = undefined;
    export let onSelectedBridge: (bridge: [string, string]) => void;
</script>

<ul class="bg-ink-500 p-3 rounded-lg flex flex-col gap-1">
    {#each bridges as [mdns, ip]}
        <BridgesListItem
            {mdns}
            {ip}
            onSelect={onSelectedBridge}
            isSelected={selected && selected[0] === mdns && selected[1] === ip}
        />
    {/each}
    <BridgesListItem
        mdns="0017884119b9.local"
        ip="192.168.1.219"
        onSelect={onSelectedBridge}
        isSelected={selected &&
            selected[0] === '0017884119b9.local' &&
            selected[1] === '192.168.1.219'}
    />
    <BridgesListItem
        mdns="0017884119ba.local"
        ip="192.168.1.220"
        onSelect={onSelectedBridge}
        isSelected={selected &&
            selected[0] === '0017884119ba.local' &&
            selected[1] === '192.168.1.220'}
    />
    <BridgesListItem
        mdns="0017884119bb.local"
        ip="192.168.1.221"
        onSelect={onSelectedBridge}
        isSelected={selected &&
            selected[0] === '0017884119bb.local' &&
            selected[1] === '192.168.1.221'}
    />
</ul>
