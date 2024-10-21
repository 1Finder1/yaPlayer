import type {ITrack} from "./track";

export interface ITrackPosition {
    current_position: number
    total: number
}

export interface IPlayerStatus {
    is_playing: boolean
    volume: number
    position: number
    total: number
}


export interface IQueueStatus {
    prev_track?: ITrack
    current_track?: ITrack
    next_track?: ITrack
}
