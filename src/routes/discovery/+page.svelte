<script lang="ts">
    import BridgesList from '$lib/components/discovery/BridgesList.svelte';
    import Button from '$lib/components/primitives/Button.svelte';
    import LoadingIndicator from '$lib/components/primitives/LoadingIndicator.svelte';
    import Typeface from '$lib/components/primitives/Typeface.svelte';
    import { faArrowRight } from '@fortawesome/free-solid-svg-icons';
    import { getDiscoveredBridges } from '../../bindings';

    const bridges = getDiscoveredBridges();
    let selectedBridge: [string, string] | undefined = undefined;
</script>

<div class="h-full w-full flex items-center justify-center">
    <div class="w-3/5 flex flex-col gap-3 items-center">
        <Typeface class="text-center mb-6" size="xxl"
            >Select your Hue Bridge</Typeface
        >

        {#await bridges}
            <LoadingIndicator
                label="Searching for Bridges in your network..."
            />
        {:then bridges}
            {#if bridges.length !== 0}
                <LoadingIndicator
                    label="Searching for Bridges in your network..."
                />
            {:else}
                <div class="flex flex-col gap-6 w-2/3 min-w-[400px]">
                    <div class="opacity-70">
                        <LoadingIndicator
                            label="Still searching for Bridges in your network..."
                            iconSize="1.5x"
                            labelSize="sm"
                            layout="row"
                        />
                    </div>

                    <BridgesList
                        {bridges}
                        selected={selectedBridge}
                        onSelectedBridge={(bridge) => {
                            selectedBridge = bridge;
                        }}
                    />

                    <Button
                        icon={faArrowRight}
                        iconOnRight
                        label="Continue"
                        disabled={selectedBridge === undefined}
                        variant="nebulaSlightGradient"
                        shadow
                    />
                </div>
            {/if}
        {:catch error}
            <Typeface>{error.message}</Typeface>
        {/await}
    </div>
</div>
