import { writable, type Writable } from 'svelte/store'
import { appWindow } from '@tauri-apps/api/window'

interface Size {
	height: number
	width: number
}

export const size: Writable<Size> = writable({ height: 150, width: 350 })

export async function initSize() {
	const windowSize = await appWindow.innerSize()
	size.set({
		height: windowSize.height,
		width: windowSize.width
	})
}
