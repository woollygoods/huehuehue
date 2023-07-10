<script lang="ts">
    import LoadingWithLabel from '$lib/components/LoadingWithLabel.svelte';
    import BridgesList from '$lib/components/discovery/BridgesList.svelte';
    import Button from '$lib/components/primitives/Button.svelte';
    import LoadingSpinner from '$lib/components/primitives/LoadingSpinner.svelte';
    import Typeface from '$lib/components/primitives/Typeface.svelte';
    import { faArrowRight } from '@fortawesome/free-solid-svg-icons';
    import { getDiscoveredBridges } from '../../bindings';

    const bridges = getDiscoveredBridges();
    let selectedBridge: [string, string] | undefined = undefined;
</script>

<div class="h-full w-full flex items-center justify-center">
    <div class="w-3/5 flex flex-col gap-6 items-center">
        <Typeface class="text-center" size="lg">Select your Hue Bridge</Typeface
        >

        {#await bridges}
            <LoadingWithLabel
                label="Searching for Bridges in your network..."
            />
        {:then bridges}
            {#if bridges.length !== 0}
                <LoadingWithLabel
                    label="Searching for Bridges in your network..."
                />
            {:else}
                <div class="flex flex-col gap-3">
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
                        variant="nebula"
                    />
                    <span
                        class="mt-3 bg-ink-500 px-3 py-2 rounded-lg flex flex-row gap-3 items-center justify-center opacity-75"
                    >
                        <LoadingSpinner size="1.5x" />
                        <Typeface size="sm"
                            >Don't worry, HueHueHue is still searching for
                            Bridges if yours isn't listed yet.</Typeface
                        ></span
                    >
                </div>
            {/if}
        {:catch error}
            <Typeface>{error.message}</Typeface>
        {/await}
    </div>
</div>
