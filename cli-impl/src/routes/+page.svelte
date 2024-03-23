<script lang="ts">
	import { Moon, Sun } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api';

	type Settings = {
		dark_mode: boolean;
	};

	let darkMode = false;
	$: invoke('get_settings').then((res) => {
		let settings = res as Settings;
		darkMode = settings.dark_mode;
		switchDarkMode(darkMode);
	});

	async function switchDarkMode(isDarkMode: boolean) {
		await invoke('set_dark_mode', { switch: isDarkMode });

		isDarkMode
			? document.documentElement.classList.add('dark')
			: document.documentElement.classList.remove('dark');
	}

	function toggleDarkMode() {
		darkMode = !darkMode;
		switchDarkMode(darkMode);
	}
</script>

<div class="h-screen bg-zinc-100 p-3 dark:bg-zinc-900">
	<div class="rounded-lg bg-white px-6 py-8 shadow-xl ring-1 ring-slate-900/5 dark:bg-slate-800">
		<div>
			<button on:click={toggleDarkMode}>
				<span
					class="inline-flex items-center justify-center rounded-md bg-indigo-500 p-2 text-white shadow-lg"
				>
					{#if darkMode}
						<Moon />
					{:else}
						<Sun />
					{/if}
				</span>
			</button>
		</div>
		<h3 class="mt-5 text-base font-medium tracking-tight text-slate-900 dark:text-white">
			Writes Upside-Down
		</h3>
		<p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
			The Zero Gravity Pen can be used to write in any orientation, including upside-down. It even
			works in outer space.
		</p>
	</div>
</div>
