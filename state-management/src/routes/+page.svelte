<script lang="ts">
	import { invoke } from '@tauri-apps/api';

	type Settings = {
		language: String;
		theme: String;
	};

	let languageValue = 'en';
	let themeValue = 'dark';

	let settings: Settings;
	$: invoke('get_settings').then((res: Settings) => {
		settings.language = res.language;
		settings.theme = res.theme;
	});

	async function onLaunguageChanged() {
		await invoke('set_launguage', { newLaunguage: languageValue });
	}

	async function onThemeChanged() {
		await invoke('set_theme', { newTheme: themeValue });
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
