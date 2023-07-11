<script lang="ts">
    import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
    import { cva, type VariantProps } from 'class-variance-authority';
    import Fa from 'svelte-fa/src/fa.svelte';
    import type { HTMLButtonAttributes } from 'svelte/elements';
    import { twMerge } from 'tailwind-merge';
    import Icon from './Icon.svelte';

    let styling = cva('transition-all px-4 py-2 font-semibold', {
        variants: {
            variant: {
                dark: 'bg-ink-500 enabled:hover:bg-ink-600 text-snow-500',
                light: 'bg-snow-500 enabled:hover:bg-snow-600 text-ink-400',
                nebula: 'bg-nebula-500 enabled:hover:bg-nebula-600 text-ink-400',
                nebulaSlightGradient:
                    'bg-gradient-to-br from-nebula-400 to-nebula-600 hover:brightness-110',
            },
            rounding: {
                full: 'rounded-full',
                xl: 'rounded-xl',
                lg: 'rounded-lg',
                md: 'rounded-md',
            },
            size: {
                lg: 'text-base',
                md: 'text-sm tracking-wide',
            },
            fullWidth: {
                true: 'w-full grow',
            },
            disabled: {
                true: 'contrast-75 opacity-80',
            },
            shadow: {
                true: 'shadow-md shadow-deepDark/10',
            },
        },
        defaultVariants: {
            variant: 'dark',
            rounding: 'lg',
            size: 'lg',
        },
    });

    /**
     * This defines the props for this component.
     * NOTE: 'disabled' is omit from the cva to use the actual button disabled prop.
     */
    interface $$Props
        extends Partial<HTMLButtonAttributes>,
            Omit<VariantProps<typeof styling>, 'disabled'> {
        icon?: IconDefinition;
        iconOnRight?: boolean;
        label: string;
        fullWidth?: boolean;
    }

    export let icon: $$Props['icon'] = undefined;
    export let iconOnRight: $$Props['iconOnRight'] = false;
    export let label: $$Props['label'] = '';
    export let disabled: $$Props['disabled'] = false;
    export let fullWidth: $$Props['fullWidth'] = false;
    export let variant: $$Props['variant'] = 'dark';
    export let rounding: $$Props['rounding'] = 'lg';
    export let size: $$Props['size'] = 'lg';
    export let shadow: $$Props['shadow'] = false;
</script>

<button
    on:click
    {...$$props}
    class={styling({
        variant,
        rounding,
        fullWidth,
        disabled,
        size,
        shadow,
        class: $$props.class,
    })}
>
    <span
        class={twMerge(
            'flex flex-row gap-2 items-center justify-center',
            iconOnRight ? 'flex-row-reverse' : ''
        )}
    >
        <Icon {icon} variant={variant === 'dark' ? 'light' : 'dark'} />
        {label}</span
    >
</button>
