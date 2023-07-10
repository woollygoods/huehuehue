<script>
    import Greet from '$lib/Greet.svelte';
    import Button from '$lib/primitives/Button.svelte';
    import Input from '$lib/primitives/Input.svelte';
    import Typeface from '$lib/primitives/Typeface.svelte';
    import { faGauge } from '@fortawesome/free-solid-svg-icons';
</script>

<Typeface>Gaggi?</Typeface>
<Greet />
<Button label="asdasd" icon={faGauge} />
<Input />
