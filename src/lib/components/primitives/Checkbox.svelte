<script lang="ts">
    import { faCheck } from '@fortawesome/free-solid-svg-icons';
    import { cva, type VariantProps } from 'class-variance-authority';
    import Icon from './Icon.svelte';

    let styling = cva('flex items-center justify-center rounded-sm', {
        variants: {
            variant: {
                dark: 'bg-ink-400 text-nebula-500',
                light: 'bg-snow-500 text-ink-500',
                nebula: 'bg-nebula-500 text-ink-400',
            },
            checked: {
                true: 'bg-opacity-100 hover:bg-opacity-75',
                false: 'bg-opacity-0 hover:bg-opacity-40',
            },
            outline: {
                true: 'border-2',
            },
            size: {
                md: 'h-5 w-5',
                lg: 'h-6 w-6',
            },
        },
        defaultVariants: {
            outline: true,
            variant: 'nebula',
            size: 'md',
        },
        compoundVariants: [
            {
                variant: 'dark',
                outline: true,
                class: 'border-ink-400',
            },
            {
                variant: 'light',
                outline: true,
                class: 'border-snow-500',
            },
            {
                variant: 'nebula',
                outline: true,
                class: 'border-nebula-500',
            },
        ],
    });

    interface $$Props extends VariantProps<typeof styling> {
        checked: boolean;
        class?: string;
    }

    export let checked: $$Props['checked'] = true;
    export let variant: $$Props['variant'] = 'nebula';
    export let outline: $$Props['outline'] = true;
    export let size: $$Props['size'] = 'md';
</script>

<div class={styling({ variant, outline, size, checked, class: $$props.class })}>
    {#if checked}
        <Icon icon={faCheck} variant={variant === 'dark' ? 'light' : 'dark'} />
    {/if}
</div>
