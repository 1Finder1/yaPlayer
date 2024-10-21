import type {ICover, ITrack} from "./track";

export interface IArtist {
    id: number,
    error?: string,
    reason?: string,
    name?: string,
    cover?: ICover,
    various?: boolean,
    composer?: boolean,
    genres?: string[],
    og_image?: string,
    op_image?: string,
    counts?: IArtistCounts,
    available?: boolean,
    ratings?: IArtistRatings,
    links: IArtistLink[],
    tickets_available?: boolean,
    likes_count?: number,
    popular_tracks: ITrack[],
    regions: string[],
    // decomposed: Decomposed[],
    description?: IArtistDescription,
    countries: string[],
    en_wikipedia_link?: string,
    db_aliases: string[],
    aliases: string[],
    init_date?: string,
    end_date?: string,
}

export interface IArtistCounts {
    tracks: number,
    direct_albums: number,
    also_albums: number,
    also_tracks: number,
}

export interface IArtistRatings {
    month: number,
    week?: number,
    day?: number,
}

export interface IArtistLink {
    title: string,
    href: string,
    item_type: string,
    social_network?: string,
}

export interface IArtistDescription {
    text: string,
    uri: string,
}

export interface IArtistLink {
    text: string,
    uri: string,
}
