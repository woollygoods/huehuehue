<script lang="ts">
    import LoadingSpinner from '$lib/components/primitives/LoadingSpinner.svelte';
    import Typeface from '$lib/components/primitives/Typeface.svelte';
</script>

<span class="flex flex-col gap-3 items-center justify-center">
    <LoadingSpinner />
    <Typeface class="w-4/5 text-center text-snow-300 opacity-75" size="sm"
        >Searching for Bridges in your network...</Typeface
    >
</span>
