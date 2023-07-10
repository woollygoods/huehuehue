<script lang="ts">
    import { faCircleNotch } from '@fortawesome/free-solid-svg-icons';
    import Icon from './Icon.svelte';

    export let size: `${number}x` = '2x';
</script>

<Icon
    variant="light"
    {size}
    icon={faCircleNotch}
    class="animate-spin text-snow-300"
/>
