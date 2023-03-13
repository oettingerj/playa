export interface Track {
	title: string
	artist: string
	album: string
	duration: number
	artworkUrl: string
}

export const TrackKeyMappings: {
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

export const PlayerStateKeyMappings: {
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
