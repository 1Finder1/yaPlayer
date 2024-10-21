use serde::Serialize;

pub enum Event {
    // Events
    // Initialize,
    // TracksFetched(Vec<Track>),
    TrackEnded,
    TrackPosition(u64),
    TrackChanged,

    PlayerPlaying(bool),
    Volume(u8), // Commands
                // Play(),
                // Resume,
                // Pause,
                // Volume(u8),
                // VolumeUp(u8),
                // VolumeDown(u8),
                // Next,
                // Previous,
                // Seek(u32),
                // SeekForward(u32),
                // SeekBackward(u32),
                // ToggleMute,
}

#[derive(Serialize)]
pub struct PlayerStatus {
    pub position: u64,
    pub total: u64,
    pub is_playing: bool,
    pub volume: u8,
}
