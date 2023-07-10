<script>
    import { faGauge } from '@fortawesome/free-solid-svg-icons';
    import Button from '../components/primitives/Button.svelte';
    import Input from '../components/primitives/Input.svelte';
    import Typeface from '../components/primitives/Typeface.svelte';
    import Greet from '../lib/Greet.svelte';
</script>

<Typeface>Gaggi?</Typeface>
<Greet />
<Button label="asdasd" icon={faGauge} />
<Input />
