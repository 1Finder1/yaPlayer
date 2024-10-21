import type {PageLoad} from "./$types";
import {invoke} from "@tauri-apps/api/tauri";

export const load: PageLoad = async () => {
    return {
        playlists: await invoke('get_my_playlists')
    }
}