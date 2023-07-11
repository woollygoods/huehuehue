<script lang="ts">
    import { cva, type VariantProps } from 'class-variance-authority';
    import type { SvelteHTMLElements } from 'svelte/elements';

    let styling = cva('select-none', {
        variants: {
            variant: {
                light: 'text-snow-500',
                dark: 'text-ink-400',
                code: 'text-snow-500 font-mono',
            },
            size: {
                xxl: 'text-4xl font-black',
                xl: 'text-2xl font-extrabold',
                lg: 'text-xl font-bold',
                md: 'text-base',
                sm: 'text-sm',
            },
        },
        defaultVariants: {
            variant: 'light',
            size: 'md',
        },
    });

    interface $$Props
        extends Partial<SvelteHTMLElements['p']>,
            VariantProps<typeof styling> {}

    export let variant: $$Props['variant'] = 'light';
    export let size: $$Props['size'] = 'md';
</script>

<p {...$$props} class={styling({ variant, size, class: $$props.class })}>
    <slot />
</p>
