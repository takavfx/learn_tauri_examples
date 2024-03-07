<script lang="ts">
	import { Button, Indicator } from 'flowbite-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { EnvelopeSolid } from 'flowbite-svelte-icons';

	type Payload = {
		count: number;
	};
	let count = 0;

	listen('count_event', (event) => {
		let payload = event.payload as Payload;
		count = payload.count;
	});
</script>

<Button class="relative" size="sm">
	<EnvelopeSolid class="text-white dark:text-white" />
	<span class="sr-only">Notifications</span>
	<Indicator color="blue" border size="xl" placement="top-right" class="text-xs font-bold"
		>{count}</Indicator
	>
</Button>
