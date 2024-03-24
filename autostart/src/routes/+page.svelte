<script lang="ts">
	import { Label, Toggle } from 'flowbite-svelte';
	import { enable, disable, isEnabled } from 'tauri-plugin-autostart-api';

	let autoStart = false;
	$: isEnabled().then((isEnabled) => (autoStart = isEnabled));

	async function onAutostartChanged() {
		autoStart = !autoStart;
		autoStart ? enable() : disable();
	}
</script>

<div class="flex flex-col">
	<ul class="m-3 divide-y-2">
		<li class="flex justify-between py-3">
			<div class="flex flex-col">
				<Label class="font-semibold">Autostart</Label>
				<Label class="text-gray-400 ">App autostart when operating system started.</Label>
			</div>
			<Toggle checked={autoStart} on:change={onAutostartChanged} />
		</li>
	</ul>
</div>
