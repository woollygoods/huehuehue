<script lang="ts">
    import type { HueV2Room } from './bindings';
    let room: HueV2Room = {
        id: "1",
        type: 'room',
        id_v1: null,
        children: [],
        services: [],
        metadata: { name: 'test', archetype: 'LivingRoom' },
    };
    console.log(room);
</script>

<div class="h-screen w-screen bg-ink-500">
    <h1 class="text-4xl bg-nebula-400 text-snow-500">hello world!</h1>
</div>
