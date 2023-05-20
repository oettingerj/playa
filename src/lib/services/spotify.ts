import {
	TrackKeyMappings,
	type Track,
	type PlayerStateInfo,
	PlayerStateKeyMappings,
	SpotifyCommand
} from '$lib/models/spotify'
import { playerState } from '$lib/stores/player'
import { invoke } from '@tauri-apps/api/tauri'

export async function getCurrentTrackInfo(): Promise<Track> {
	const trackInfoKeys = Object.values(TrackKeyMappings)
	const trackInfoStoreKeys = Object.keys(TrackKeyMappings)
	const keysStr = trackInfoKeys.join(',')

	let output: string = await invoke('get_current_track_info', { keys: keysStr })

	output = output.replace(/{}\s/g, '')
	const values = output.split(',')

	let track: Record<string, any> = {}
	values.forEach((val, i) => {
		track = {
			...track,
			[trackInfoStoreKeys[i]]: val.replace(/^\s+|\s+$/g, '')
		}
	})

	if (track.duration) {
		track.duration = parseFloat(track.duration) / 1000
	}

	return track as Track
}

export async function getPlayerState(): Promise<PlayerStateInfo> {
	const stateInfoKeys = Object.values(PlayerStateKeyMappings)
	const stateInfoStoreKeys = Object.keys(PlayerStateKeyMappings)
	const keysStr = stateInfoKeys.join(',')

	let output: string = await invoke('get_player_state', { keys: keysStr })

	output = output.replace(/{}\s/g, '')
	const values = output.split(',')

	let state: Record<string, any> = {}
	values.forEach((val, i) => {
		state = {
			...state,
			[stateInfoStoreKeys[i]]: val.replace(/^\s+|\s+$/g, '')
		}
	})

	if (state.playbackPosition) {
		state.playbackPosition = parseFloat(state.playbackPosition)
	}

	return state as PlayerStateInfo
}

export async function sendSpotifyCommand(cmd: SpotifyCommand) {
	await invoke('send_spotify_command', { command: cmd })
}

export function startPlaybackTicker(): number {
	return setInterval(() => {
		playerState.update((state) => {
			if (!state) return state
			return {
				...state,
				playbackPosition: state.playbackPosition + 0.1
			}
		})
	}, 100)
}

export function stopPlaybackTicker(timer: number) {
	clearInterval(timer)
}
