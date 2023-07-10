<script lang="ts">
    import BridgesListItem from './BridgesListItem.svelte';

    export let bridges: [string, string][];
</script>

<ul class="bg-ink-500 p-3 rounded-lg flex flex-col gap-1">
    {#each bridges as [mdns, ip]}
        <BridgesListItem {mdns} {ip} />
    {/each}
    <BridgesListItem mdns="0017884119b9.local" ip="192.168.1.219" />
    <BridgesListItem mdns="0017884119ba.local" ip="192.168.1.220" />
    <BridgesListItem mdns="0017884119bb.local" ip="192.168.1.221" />
</ul>
