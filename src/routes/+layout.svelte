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

<div class="flex flex-col bg-zinc-900 h-screen w-screen">
	<div data-tauri-drag-region class="flex fixed w-full items-center p-1">
		<button
			on:click={closeWindow}
			class="cursor-default text-white h-5 w-5 hover:text-spotify-green"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M6 18L18 6M6 6l12 12"
				/>
			</svg>
		</button>
	</div>
	<slot />
</div>
