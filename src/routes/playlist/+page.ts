import type {PageLoad} from "./$types";
import {invoke} from "@tauri-apps/api/tauri";
import type {IPlaylist} from "@type";

export const load: PageLoad = async ({url}) => {
    const data = await invoke<IPlaylist>('get_playlist', {
        ownerId: +(url.searchParams.get('owner') || '0'),
        playlistId: +(url.searchParams.get('id') || '0')
    })

    return {
        playlist: data
    }
}