<script lang="ts">
    import { faAmbulance } from '@fortawesome/free-solid-svg-icons';
    import Button from '../components/primitives/Button.svelte';
    import Typeface from '../components/primitives/Typeface.svelte';
</script>

<Typeface>this is text</Typeface>
<Button icon={faAmbulance} label="lable" onClick={() => {}} />
