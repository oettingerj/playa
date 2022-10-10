<script lang="ts">
	import {
		PlayerState,
		sendSpotifyCommand,
		SpotifyCommand
	} from '$lib/services/spotify'
	import { trackInfo, playerState } from '$lib/stores/player'
	import { size } from '$lib/stores/window'

	const buttonClasses =
		'text-white h-8 w-8 cursor-default hover:text-spotify-green'

	$: artworkSize = $size.height - 60
	$: metadataWidth = $size.width - artworkSize - 50

	function previous() {
		sendSpotifyCommand(SpotifyCommand.PrevTrack)
	}

	function next() {
		sendSpotifyCommand(SpotifyCommand.NextTrack)
	}

	function playpause() {
		sendSpotifyCommand(SpotifyCommand.PlayPause)
		if ($playerState?.state === PlayerState.Paused) {
			$playerState.state = PlayerState.Playing
		} else if ($playerState?.state === PlayerState.Playing) {
			$playerState.state = PlayerState.Paused
		}
	}
</script>

{#if $trackInfo && $playerState}
	<div class="flex h-full w-full items-center px-5">
		<img
			class="object-contain"
			style="height: {artworkSize}px; width: {artworkSize}px;"
			src={$trackInfo.artworkUrl}
			alt="album artwork"
		/>
		<div class="flex flex-col ml-5 justify-between">
			<div class="flex flex-col grow gap-1 items-start">
				<span
					class="font-semibold text-md text-white truncate cursor-default"
					style="max-width: {metadataWidth}px;">{$trackInfo.title}</span
				>
				<span
					class="font-normal text-sm text-gray-300 truncate cursor-default"
					style="max-width: {metadataWidth}px;">{$trackInfo.artist}</span
				>
			</div>
			<div class="flex gap-6 items-center w-full h-full">
				<button class={buttonClasses} on:click={previous}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						fill="currentColor"
					>
						<path
							d="M9.195 18.44c1.25.713 2.805-.19 2.805-1.629v-2.34l6.945 3.968c1.25.714 2.805-.188 2.805-1.628V8.688c0-1.44-1.555-2.342-2.805-1.628L12 11.03v-2.34c0-1.44-1.555-2.343-2.805-1.629l-7.108 4.062c-1.26.72-1.26 2.536 0 3.256l7.108 4.061z"
						/>
					</svg>
				</button>
				<button class={buttonClasses} on:click={playpause}>
					{#if $playerState.state === PlayerState.Playing}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M6.75 5.25a.75.75 0 01.75-.75H9a.75.75 0 01.75.75v13.5a.75.75 0 01-.75.75H7.5a.75.75 0 01-.75-.75V5.25zm7.5 0A.75.75 0 0115 4.5h1.5a.75.75 0 01.75.75v13.5a.75.75 0 01-.75.75H15a.75.75 0 01-.75-.75V5.25z"
								clip-rule="evenodd"
							/>
						</svg>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 24 24"
							fill="currentColor"
						>
							<path
								fill-rule="evenodd"
								d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z"
								clip-rule="evenodd"
							/>
						</svg>
					{/if}
				</button>
				<button class={buttonClasses} on:click={next}>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						fill="currentColor"
					>
						<path
							d="M5.055 7.06c-1.25-.714-2.805.189-2.805 1.628v8.123c0 1.44 1.555 2.342 2.805 1.628L12 14.471v2.34c0 1.44 1.555 2.342 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256L14.805 7.06C13.555 6.346 12 7.25 12 8.688v2.34L5.055 7.06z"
						/>
					</svg>
				</button>
			</div>
		</div>
	</div>
{:else}
	No item playing
{/if}
