import type { PlayerStateInfo, Track } from '$lib/services/spotify'
import { writable, type Writable } from 'svelte/store'

export const trackInfo: Writable<Track | null> = writable(null)
export const playerState: Writable<PlayerStateInfo | null> = writable(null)
