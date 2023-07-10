<script>
    import LoadingWithLabel from '$lib/components/LoadingWithLabel.svelte';
    import Typeface from '$lib/components/primitives/Typeface.svelte';
    import { getDiscoveredBridges } from '../../bindings';

    const bridges = getDiscoveredBridges();
</script>

<div class="h-full w-full flex items-center justify-center">
    <div class="w-3/5 flex flex-col gap-9 items-center">
        <Typeface class="text-center" size="lg">Select your Hue Bridge</Typeface
        >

        {#await bridges}
            <LoadingWithLabel />
        {:then bridges}
            {#if bridges.length === 0}
                <LoadingWithLabel />
            {:else}
                <ul>
                    {#each bridges as [mdns, ip]}
                        <li>{`${mdns} - ${ip}`}</li>
                    {/each}
                </ul>
            {/if}
        {:catch error}
            <Typeface>{error.message}</Typeface>
        {/await}
    </div>
</div>
