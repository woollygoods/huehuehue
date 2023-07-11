<script lang="ts">
    import { goto } from '$app/navigation';
    import '../styles.css';

    let needsDiscovery = true;

    if (needsDiscovery) {
        goto('/discovery');
    }
</script>

<div class="bg-ink-400 w-screen h-screen p-6"><slot /></div>
