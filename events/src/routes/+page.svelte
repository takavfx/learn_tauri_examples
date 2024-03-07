<script lang="ts">
	import { Tabs, TabItem } from 'flowbite-svelte';
	import ClickEvent from '$lib/components/ClickEvent.svelte';
	import CountEvent from '$lib/components/CountEvent.svelte';
</script>

<Tabs>
	<TabItem open title="Click">
		<ClickEvent />
	</TabItem>
	<TabItem title="Count">
		<CountEvent />
	</TabItem>
</Tabs>
