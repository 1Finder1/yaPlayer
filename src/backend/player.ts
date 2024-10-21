import { invoke } from '@tauri-apps/api/tauri'
import type { IPlayerStatus } from '@type'

export async function getPlayerInfo() {
	return await invoke<IPlayerStatus>('get_player_info')
}

export async function playerPlayPause() {
	return await invoke('play_pause')
}

export async function playerSeek(seconds: number) {
	return await invoke('player_seek', { seconds })
}

export async function playerSetVolume(volume: number) {
	return await invoke('player_volume', { volume })
}
