<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { Select, Label } from 'flowbite-svelte';

	type Settings = {
		language: string;
		theme: string;
	};

	let languageSelected = 'en';
	let themeSelected = 'dark';

	const languageSets = [
		{ value: 'en', name: 'Engilish' },
		{ value: 'ja', name: '日本語' }
	];

	const themeSets = [
		{ value: 'dark', name: 'Dark' },
		{ value: 'light', name: 'Light' },
		{ value: 'class', name: 'OS Setting' }
	];

	$: invoke('get_settings').then((res: Settings) => {
		languageSelected = res.language;
		themeSelected = res.theme;
	});

	async function onLaunguageChanged() {
		await invoke('set_language', { newLanguage: languageSelected });
	}

	async function onThemeChanged() {
		await invoke('set_theme', { newTheme: themeSelected });
	}
</script>

<div class="flex flex-col">
	<ul class="m-3 divide-y-2">
		<li class="flex justify-between py-3">
			<div class="flex flex-col">
				<Label class="font-semibold">Language</Label>
				<Label class="text-gray-400 ">Language settings</Label>
			</div>
			<Select
				size="lg"
				items={languageSets}
				bind:value={languageSelected}
				on:change={onLaunguageChanged}
				class="rounded-lg border-2 border-rose-500 p-3"
			/>
		</li>
		<li class="flex justify-between py-3">
			<div class="flex flex-col">
				<Label class="font-semibold">Theme</Label>
				<Label class="text-gray-400 ">Theme settings</Label>
			</div>
			<Select
				size="lg"
				items={themeSets}
				bind:value={themeSelected}
				on:change={onThemeChanged}
				class="rounded-lg border-2 border-rose-500 p-3"
			/>
		</li>
	</ul>
</div>
