<script lang="ts">
    import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
    import { cva, type VariantProps } from 'class-variance-authority';
    import Icon from './Icon.svelte';

    let styling = cva('transition-colors px-4 py-2 font-semibold', {
        variants: {
            variant: {
                dark: 'bg-ink-500 enabled:hover:bg-ink-600 text-snow-500',
                light: 'bg-snow-500 enabled:hover:bg-snow-600 text-ink-400',
                nebula: 'bg-nebula-500 enabled:hover:bg-nebula-600 text-ink-400',
            },
            rounding: {
                full: 'rounded-full',
                xl: 'rounded-xl',
                lg: 'rounded-lg',
                md: 'rounded-md',
            },
            fullWidth: {
                true: 'w-full grow',
            },
            disabled: {
                true: 'contrast-75 opacity-80',
            },
        },
        defaultVariants: {
            variant: 'dark',
            rounding: 'lg',
        },
    });

    export let icon: IconDefinition | undefined = undefined;
    export let label: string;
    export let onClick: () => void;
    export let disabled: boolean = false;
    export let fullWidth: boolean = false;
    export let variant: VariantProps<typeof styling>['variant'] = 'dark';
    export let rounding: VariantProps<typeof styling>['rounding'] = 'lg';
</script>

<button
    class={styling({ variant, rounding, fullWidth, disabled })}
    on:click={onClick}
    {disabled}
>
    <span class="flex flex-row gap-2 items-center justify-center">
        <Icon {icon} variant={variant === 'dark' ? 'light' : 'dark'} />
        {label}</span
    >
</button>
