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

    export let icon: IconDefinition | undefined = undefined;
    export let variant: VariantProps<typeof styling>['variant'] = 'dark';
</script>

{#if icon}
    <Fa {icon} class={styling({ variant, class: $$props.class })} />
{/if}
