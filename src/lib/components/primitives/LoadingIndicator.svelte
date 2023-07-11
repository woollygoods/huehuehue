<script lang="ts">
    import { faCircleNotch } from '@fortawesome/free-solid-svg-icons';
    import { twMerge } from 'tailwind-merge';
    import Icon from './Icon.svelte';
    import Typeface from './Typeface.svelte';
    import type { IconSize, TypefaceSize } from './types';

    export let iconSize: IconSize = '2x';
    export let label: string | undefined = undefined;
    export let labelSize: TypefaceSize = 'sm';
    export let layout: 'row' | 'col' = 'col';
</script>

<span
    class={twMerge(
        'flex gap-3 items-center justify-center',
        layout === 'row' ? 'flex-row' : 'flex-col'
    )}
>
    <Icon
        variant="light"
        size={iconSize}
        icon={faCircleNotch}
        class="animate-spin text-snow-300"
    />
    {#if label}
        <Typeface size={labelSize}>{label}</Typeface>
    {/if}
</span>
