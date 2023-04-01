<script lang="ts">
	import '../app.css'
	import { onDestroy, onMount } from 'svelte'
	import { invoke } from '@tauri-apps/api/tauri'
	import { PlayerState } from '$lib/models/spotify'
	import { listen } from '@tauri-apps/api/event'
	import { trackInfo, playerState } from '$lib/stores/player'
	import {
		getCurrentTrackInfo,
		getPlayerState,
		startPlaybackTicker,
		stopPlaybackTicker
	} from '$lib/services/spotify'

	let timer: number | undefined

	async function initPlayer() {
		const track = await getCurrentTrackInfo()
		if (track) trackInfo.set(track)
		const state = await getPlayerState()
		if (state) playerState.set(state)
	}

	function initTimer() {
		if ($playerState?.state === PlayerState.Playing && !timer) {
			timer = startPlaybackTicker()
		} else if (timer) {
			stopPlaybackTicker(timer)
			timer = undefined
		}
	}

	onMount(async () => {
		initPlayer().then(initTimer)
		return await listen('playback-state-changed', async () => {
			await initPlayer()
			initTimer()
		})
	})

	onMount(() => {
		invoke('init_spotify_event_emitter')
	})

	onDestroy(() => {
		invoke('remove_spotify_event_observers')
		if (timer) stopPlaybackTicker(timer)
	})
</script>

<div class="flex flex-col bg-zinc-900 h-screen w-screen">
	<slot />
</div>
