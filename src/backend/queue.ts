import {invoke} from "@tauri-apps/api/tauri";
import type {IQueueStatus} from "@type";

export async function getQueueStatus() {
    return await invoke<IQueueStatus>("get_queue_status")
}

export async function playPlaylist(user_id: number, playlist_id: number, track_id?: number) {
    return await invoke("play_playlist", {"userId": user_id, "playlistId": playlist_id, "trackId": track_id})
}