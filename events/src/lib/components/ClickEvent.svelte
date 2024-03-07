<script lang="ts">
	import { GradientButton } from 'flowbite-svelte';
	import { emit } from '@tauri-apps/api/event';

	async function onClicked() {
		emit('click', { theMessage: 'Tauri is awsome!' });
	}
</script>

<GradientButton outline color="greenToBlue" on:click={onClicked}>Click to shout!</GradientButton>
