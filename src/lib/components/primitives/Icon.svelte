<script lang="ts">
    import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
    import { cva, type VariantProps } from 'class-variance-authority';
    // @ts-ignore
    import Fa from 'svelte-fa/src/fa.svelte';
    import type { IconSize } from './types';

    const styling = cva('transition-colors', {
        variants: {
            variant: {
                light: 'text-snow-500',
                dark: 'text-ink-400',
                nebula: 'text-nebula-500',
            },
        },
    });

    interface $$Props extends VariantProps<typeof styling> {
        icon?: IconDefinition;
        class?: string;
        size?: IconSize;
    }

    export let icon: $$Props['icon'] = undefined;
    export let variant: $$Props['variant'] = 'dark';
    export let size: $$Props['size'] = '1x';
</script>

{#if icon}
    <Fa {icon} {size} fw class={styling({ variant, class: $$props.class })} />
{/if}
