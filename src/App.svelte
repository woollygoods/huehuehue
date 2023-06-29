<script lang="ts">
    import { Route, Router } from 'svelte-routing';
    import Dashboard from './pages/Dashboard.svelte';
    import Discovery from './pages/Discovery.svelte';

    /**
     * To actually decide if the discovery is needed, get the state from the
     * rust backend.
     */
    let needsDiscovery = true;
    let originalURL = needsDiscovery ? '/discovery' : '/';
</script>

<div class="h-screen w-screen bg-ink-400 p-6">
    <Router url={originalURL}>
        <Route path="/" component={Dashboard} />
        <Route path="/discovery" component={Discovery} />
    </Router>
</div>
