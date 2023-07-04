<script>
    import { getDiscoveredBridges } from '../bindings';
    const discoveredBridgesPromise = getDiscoveredBridges();
</script>

<div>
    {#await discoveredBridgesPromise}
        <h1 class="text-4xl bg-nebula-400 text-snow-500">hello world!</h1>
        <p class="text-black">loading...</p>
    {:then discoveredBridges}
        <ul>
            {#each discoveredBridges as [mdns, ip]}
                <li>{`${mdns} - ${ip}`}</li>
            {/each}
        </ul>
    {:catch error}
        <p class="text-black">error: {error.message}</p>
    {/await}
</div>
