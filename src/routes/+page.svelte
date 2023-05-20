<script lang="ts">
	import { sendSpotifyCommand } from '$lib/services/spotify'
	import { PlayerState, SpotifyCommand } from '$lib/models/spotify'
	import { trackInfo, playerState } from '$lib/stores/player'
	import { size } from '$lib/stores/window'
	import NextIcon from '../components/icons/NextIcon.svelte'
	import PauseIcon from '../components/icons/PauseIcon.svelte'
	import PlayIcon from '../components/icons/PlayIcon.svelte'
	import PrevIcon from '../components/icons/PrevIcon.svelte'

	const buttonClasses =
		'flex items-center justify-center text-gray-100 h-9 w-9 hover:text-gray-300'

	$: metadataWidth = $size.width - 50

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
		data-tauri-drag-region
		class="flex flex-col relative h-full w-full items-center"
	>
		<img
			class="object-contain w-full pointer-events-none"
			src={$trackInfo.artworkUrl}
			alt="album artwork"
		/>
		<div
			class="absolute bottom-0 flex items-center justify-center w-full p-2 metadata"
		>
			<progress
				class="absolute top-0 w-full h-[2px]"
				value={$playerState.playbackPosition}
				max={$trackInfo.duration}
			/>
			<div class="flex flex-col items-center justify-between select-none">
				<div class="flex flex-col gap-1 items-center">
					<span
						class="font-semibold text-sm text-slate-50 truncate text-contrast cursor-default"
						style="max-width: {metadataWidth}px;">{$trackInfo.title}</span
					>
					<span
						class="font-normal text-xs text-gray-100 truncate text-contrast cursor-default"
						style="max-width: {metadataWidth}px;">{$trackInfo.artist}</span
					>
				</div>
				<div class="flex gap-4 items-center justify-center w-full h-full">
					<button class={buttonClasses} on:click={previous}>
						<PrevIcon class="h-6" />
					</button>
					<button class={buttonClasses} on:click={playpause}>
						{#if $playerState.state === PlayerState.Playing}
							<PauseIcon class="h-6" />
						{:else}
							<PlayIcon class="h-6" />
						{/if}
					</button>
					<button class={buttonClasses} on:click={next}>
						<NextIcon class="h-6" />
					</button>
				</div>
			</div>
		</div>
	</div>
{:else}
	No item playing
{/if}

<style>
	progress::-webkit-progress-value {
		@apply bg-gray-100;
	}

	progress::-webkit-progress-bar {
		background: inherit;
		backdrop-filter: brightness(95%);
		-webkit-backdrop-filter: brightness(95%);
	}

	.metadata {
		backdrop-filter: blur(12px) brightness(85%);
		-webkit-backdrop-filter: blur(12px) brightness(85%);
	}
</style>
