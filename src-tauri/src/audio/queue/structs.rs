use serde::Serialize;
use yandex_music::model::track_model::track::Track;

#[derive(PartialEq)]
pub enum QueueRepeatMode {
    None,
    Track,
    Playlist,
}

#[derive(Serialize, Clone)]
pub struct QueueStatus {
    pub prev_track: Option<Track>,
    pub current_track: Option<Track>,
    pub next_track: Option<Track>,
}
