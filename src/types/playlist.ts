import type {IUser} from "./user";
import type {ICover, IPlaylistTrack} from "./track";

export interface IPlaylist {
     playlist_uuid: String,
     description?: string,
     description_formatted?: string,
     available: boolean,
     collective: boolean,
     cover: ICover,
     created: String,
     modified: String,
     background_color?: string,
     text_color: string,
     duration_ms: number,
     is_banner: boolean,
     is_premiere: boolean,
     kind: number,
     og_image: String,
     owner: IUser,
     revision: number,
     snapshot: number,
     // tags: Tag[], // TODO
     title: String,
     track_count: number,
     uid: number,
     visibility: String,
     tracks: IPlaylistTrack[],
     likes_count: number,
     // similar_playlists: Playlist[], TODO
    
}
