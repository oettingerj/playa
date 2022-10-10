import { invoke } from '@tauri-apps/api/tauri'

export interface Track {
	title: string
	artist: string
	album: string
	duration: number
	artworkUrl: string
}

const TrackKeyMappings: {
	[key in keyof Track]: string
} = {
	title: 'name',
	artist: 'artist',
	album: 'album',
	duration: 'duration',
	artworkUrl: 'artwork url'
}

export interface PlayerStateInfo {
	state: PlayerState
	playbackPosition: number
}

const PlayerStateKeyMappings: {
	[key in keyof PlayerStateInfo]: string
} = {
	state: 'player state',
	playbackPosition: 'player position'
}

export enum SpotifyCommand {
	NextTrack = 'next track',
	PrevTrack = 'previous track',
	PlayPause = 'playpause',
	Pause = 'pause',
	Play = 'play'
}

export enum PlayerState {
	Playing = 'playing',
	Paused = 'paused',
	Stopped = 'stopped'
}

export async function getCurrentTrackInfo(): Promise<Track> {
	const trackInfoKeys = Object.values(TrackKeyMappings)
	const trackInfoStoreKeys = Object.keys(TrackKeyMappings)
	const keysStr = trackInfoKeys.join(',')

	let output: string = await invoke('get_current_track_info', { keys: keysStr })

	console.log(output)

	output = output.replace(/{}\s/g, '')
	const values = output.split(',')

	let track = {}
	values.forEach((val, i) => {
		track = {
			...track,
			[trackInfoStoreKeys[i]]: val
		}
	})

	return track as Track
}

export async function getPlayerState(): Promise<PlayerStateInfo> {
	const stateInfoKeys = Object.values(PlayerStateKeyMappings)
	const stateInfoStoreKeys = Object.keys(PlayerStateKeyMappings)
	const keysStr = stateInfoKeys.join(',')

	let output: string = await invoke('get_player_state', { keys: keysStr })

	output = output.replace(/{}\s/g, '')
	const values = output.split(',')

	let state = {}
	values.forEach((val, i) => {
		state = {
			...state,
			[stateInfoStoreKeys[i]]: val
		}
	})

	return state as PlayerStateInfo
}

export async function sendSpotifyCommand(cmd: SpotifyCommand) {
	await invoke('send_spotify_command', { command: cmd })
}
