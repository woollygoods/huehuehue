<script lang="ts">
    import { getDiscoveredBridges } from './bindings';

    const discoveredBridgesPromise = getDiscoveredBridges();
</script>

{#await discoveredBridgesPromise}
    <p class='text-black'>loading...</p>
{:then discoveredBridges}
    <ul>
        {#each discoveredBridges as [mdns, ip]}
            <li>{`${mdns} - ${ip}`}</li>
        {/each}
    </ul>
{:catch error}
    <p class='text-black'>error: {error.message}</p>
{/await}

