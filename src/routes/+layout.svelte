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

	onMount(() => {
		invoke('init_spotify_event_emitter')
	})

	onDestroy(() => {
		invoke('remove_spotify_event_observers')
	})
</script>

<main class="bg-zinc-900 h-screen w-screen">
	<slot />
</main>
