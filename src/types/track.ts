import type {IUser} from "./user";
import type {IArtist} from "./artist";
import type {IAlbum} from "./album";


interface MosaicCover {
    type: 'mosaic',
    itemsUri: string[]
}

interface PicCover {
    type: 'pic'
    uri: string

}
export type ICover = MosaicCover & PicCover & {
    dir?: string,
    version?: string,
    is_custom?: boolean,
    custom?: boolean,
    prefix?: string,
    copyright_name?: string,
    copyright_cline?: string,
    error?: string,
}

export interface PartialTrack {
    id: number,
    album_id: number,
    timestamp: string,
}

export interface ITrack {
    id: number,
    title?: string,
    available?: boolean,
    artists: IArtist[],
    albums: IAlbum[], // TODO
    available_for_premium_users?: boolean,
    lyrics_available?: boolean,
    best?: boolean,
    og_image?: string,
    item_type?: string,
    coverUri?: string,
    // major?: TrackMajor,
    durationMs?: number,
    storage_dir?: string,
    file_size?: number,
    substituted: ITrack,
    matched_track: ITrack,
    // normalization: TrackNormalization[], // TODO
    error?: string,
    can_publish?: boolean,
    state?: string,
    desired_visibility?: string,
    filename?: string,
    user_info?: IUser,
    // meta_data?: TrackMetadata, // TODO
    regions: string[],
    available_as_rbt?: boolean,
    content_warning?: string,
    explicit?: boolean,
    preview_duration_ms?: number,
    available_full_without_permission?: boolean,
    version?: string,
    remember_position?: boolean,
    background_video_uri?: string,
    short_description?: string,
    is_suitable_for_children?: boolean,
    track_source?: string,
    available_for_options: string[],
    // r128?: TrackR128,
    // lyrics_info?: TrackLyricsInfo,
    track_sharing_flag?: string,
    disclaimers: string[],
    // derived_colors?: TrackDerivedColors,
    // fade?: TrackFade,
    special_audio_resources: string[],
    player_id?: string,
}

export interface IPlaylistTrack {
    id: number,
    original_index: number,
    timestamp: String,
    track: ITrack,
    recent: boolean,
    original_shuffle_index: number
}


export interface ITrackPosition {
    volume: number,
    index: number,
}

