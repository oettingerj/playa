<script lang="ts">
	import { sendSpotifyCommand } from '$lib/services/spotify'
	import { PlayerState, SpotifyCommand } from '$lib/models/spotify'
	import { trackInfo, playerState } from '$lib/stores/player'
	import { size } from '$lib/stores/window'
	import NextIcon from '../components/icons/NextIcon.svelte'
	import PauseIcon from '../components/icons/PauseIcon.svelte'
	import PlayIcon from '../components/icons/PlayIcon.svelte'
	import PrevIcon from '../components/icons/PrevIcon.svelte'
	import { fade } from 'svelte/transition'

	const buttonClasses =
		'text-gray-100 h-9 w-9 cursor-default hover:text-gray-300'
	let showControls = false

	$: artworkSize = $size.height - 60
	$: metadataWidth = $size.width - artworkSize - 50

	function handleMouseEnter() {
		showControls = true
	}
	function handleMouseLeave() {
		showControls = false
	}

	function previous() {
		sendSpotifyCommand(SpotifyCommand.PrevTrack)
	}

	function next() {
		sendSpotifyCommand(SpotifyCommand.NextTrack)
	}

	function playpause() {
		sendSpotifyCommand(SpotifyCommand.PlayPause)
		if ($playerState?.state && $playerState.state === PlayerState.Paused) {
			$playerState.state = PlayerState.Playing
		} else if (
			$playerState?.state &&
			$playerState.state === PlayerState.Playing
		) {
			$playerState.state = PlayerState.Paused
		}
	}
</script>

{#if $trackInfo && $playerState}
	<div
		on:mouseenter={handleMouseEnter}
		on:mouseleave={handleMouseLeave}
		class="flex relative h-full w-full items-center"
	>
		<img
			data-tauri-drag-region
			class="absolute top-0 left-0 object-contain w-full pointer-events-none"
			src={$trackInfo.artworkUrl}
			alt="album artwork"
		/>
		{#if showControls}
			<div
				transition:fade={{ duration: 100 }}
				class="absolute bottom-0 w-full px-5 py-2 backdrop-blur-md bg-gray-800/80"
			>
				<div class="flex flex-col items-center justify-between select-none">
					<div class="flex flex-col grow gap-1 items-center">
						<span
							class="font-semibold text-md text-slate-50 truncate cursor-default"
							style="max-width: {metadataWidth}px;">{$trackInfo.title}</span
						>
						<span
							class="font-normal text-sm text-gray-300 truncate cursor-default"
							style="max-width: {metadataWidth}px;">{$trackInfo.artist}</span
						>
					</div>
					<div class="flex gap-6 items-center justify-center w-full h-full">
						<button class={buttonClasses} on:click={previous}>
							<PrevIcon class="h-8" />
						</button>
						<button class={buttonClasses} on:click={playpause}>
							{#if $playerState.state === PlayerState.Playing}
								<PauseIcon class="h-8" />
							{:else}
								<PlayIcon class="h-8" />
							{/if}
						</button>
						<button class={buttonClasses} on:click={next}>
							<NextIcon class="h-8" />
						</button>
					</div>
				</div>
			</div>
		{/if}
		<progress
			class="absolute bottom-0 w-full h-[3px]"
			value={$playerState.playbackPosition}
			max={$trackInfo.duration}
		/>
	</div>
{:else}
	No item playing
{/if}

<style>
	progress::-webkit-progress-value {
		@apply bg-gray-100;
	}

	progress::-webkit-progress-bar {
		@apply bg-gray-400/50;
	}
</style>
