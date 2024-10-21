import { listen } from '@tauri-apps/api/event'
import { getQueueStatus } from '@backend'
import { playerStatus, queue } from '../stores/player'
import type { IQueueStatus } from '@type'

function subscribeQueueUpdate() {
	return listen<IQueueStatus>('queue-update', (event) => {
		// setTimeout(() => {
		getQueueStatus().then((data) => {
			queue.set(data)
			playerStatus.update((old) => ({
				...old,
				total: (data.current_track?.durationMs || 0) / 1000,
			}))
		})
		// }, 1000)
	})
}

function subscribePlayerPosition() {
	return listen<number>('player-position', (event) => {
		playerStatus.update((old) => ({ ...old, position: event.payload }))
	})
}

function subscribeVolumeChange() {
	return listen<number>('volume-change', (event) =>
		playerStatus.update((old) => ({ ...old, volume: event.payload })),
	)
}

function subscribeIsPlaying() {
	return listen<boolean>('player-playing', ({ payload: is_playing }) => {
		playerStatus.update((old) => ({ ...old, is_playing }))
	})
}

export function subscribeEvents() {
	const queuePos = subscribeQueueUpdate()
	const playerPosition = subscribePlayerPosition()
	const isPLaying = subscribeIsPlaying()
	const volume = subscribeVolumeChange()

	return () => {
		queuePos.then((fn) => fn())
		playerPosition.then((fn) => fn())
		isPLaying.then((fn) => fn())
		volume.then((fn) => fn())
	}
}
