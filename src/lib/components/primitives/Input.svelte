<script lang="ts">
    import { cva } from 'class-variance-authority';

    let styling = cva('transition-colors', {
        variants: {},
    });

    export let value: string = '';
</script>

<input {value} />
