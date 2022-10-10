<script lang="ts" context="module">
	import '../app.css'
	import { listen } from '@tauri-apps/api/event'
	import { trackInfo, playerState } from '$lib/stores/player'
	import { getCurrentTrackInfo, getPlayerState } from '$lib/services/spotify'

	async function initPlayer() {
		const track = await getCurrentTrackInfo()
		if (track) trackInfo.set(track)
		const state = await getPlayerState()
		if (state) playerState.set(state)
	}
	initPlayer()

	listen('playback-state-changed', async () => {
		initPlayer()
	})
</script>

<script lang="ts">
	import { onDestroy, onMount } from 'svelte'
	import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window'

	onMount(() => {
		invoke('init_spotify_event_emitter')
	})

	onDestroy(() => {
		invoke('remove_spotify_event_observers')
	})

	function closeWindow() {
		appWindow.hide()
	}
</script>

<div
	class="flex flex-col bg-zinc-900 bg-opacity-70 h-screen w-screen"
	style="border-radius: 10px;"
>
	<div data-tauri-drag-region class="flex fixed w-full items-center p-2">
		<button on:click={closeWindow} class="group cursor-default p-0.5">
			<div class="h-[14px] w-[14px] rounded-full bg-red-500 text-zinc-900">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 20 20"
					fill="currentColor"
					class="opacity-0 group-hover:opacity-100"
				>
					<path
						d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
					/>
				</svg>
			</div>
		</button>
	</div>
	<slot />
</div>
