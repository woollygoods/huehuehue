<script lang="ts">
    import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
    import { cva, type VariantProps } from 'class-variance-authority';
    import Fa from 'svelte-fa/src/fa.svelte';

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
    }

    export let icon: $$Props['icon'] = undefined;
    export let variant: $$Props['variant'] = 'dark';
</script>

{#if icon}
    <Fa {icon} class={styling({ variant, class: $$props.class })} />
{/if}
