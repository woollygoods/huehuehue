<script lang="ts">
    import { cva, type VariantProps } from 'class-variance-authority';

    let styling = cva('transition-colors px-4 py-2 font-semibold', {
        variants: {
            coloring: {
                default: 'bg-ink-500 enabled:hover:bg-ink-600 text-snow-500',
                nebula: 'bg-nebula-500 enabled:hover:bg-nebula-600 text-ink-400',
            },
            rounding: {
                full: 'rounded-full',
                xl: 'rounded-xl',
                lg: 'rounded-lg',
                md: 'rounded-md',
                sm: 'rounded-sm',
            },
            fullWidth: {
                true: 'w-full grow',
            },
            disabled: {
                true: 'contrast-75 opacity-80',
            },
        },
        defaultVariants: {
            coloring: 'default',
            rounding: 'lg',
        },
    });

    export let label: string;
    export let onClick: () => void;
    export let disabled: boolean = false;
    export let fullWidth: boolean = false;
    export let coloring: VariantProps<typeof styling>['coloring'] = 'default';
    export let rounding: VariantProps<typeof styling>['rounding'] = 'lg';
</script>

<button
    class={styling({ coloring, rounding, fullWidth, disabled })}
    on:click={onClick}
    {disabled}>{label}</button
>
