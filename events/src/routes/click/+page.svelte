<script lang="ts">
	import { emit, listen } from '@tauri-apps/api/event';
	import { appWindow, WebviewWindow } from '@tauri-apps/api/window';

	// appWindow.emit('event', { message: 'Tauri is awsome!' });

	async function onClicked() {
		// const unlisten = await listen('click', (event) => console.log(event));
		emit('click', { theMessage: 'Tauri is awsome!' });
	}
</script>

<button on:click={onClicked}>Hello</button>
