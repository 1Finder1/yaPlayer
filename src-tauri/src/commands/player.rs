use crate::audio::backend::AudioPlayer;
use crate::audio::structs::PlayerStatus;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_player_info(
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<PlayerStatus, ()> {
    let p = &player.lock().await;

    let x = Ok(PlayerStatus {
        is_playing: p.is_playing.load(Ordering::Relaxed).to_owned(),
        position: p.track_position.current_position.read().unwrap().as_secs(),
        total: p.track_position.total.read().unwrap().as_secs(),
        volume: p.volume,
    });
    x
}

#[tauri::command]
pub async fn play_pause(player: State<'_, Arc<Mutex<AudioPlayer>>>) -> Result<(), ()> {
    player.lock().await.play_pause();

    Ok(())
}

#[tauri::command]
pub async fn player_seek(
    seconds: u64,
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<(), ()> {
    player.lock().await.seek(seconds);
    Ok(())
}

#[tauri::command]
pub async fn player_volume(
    volume: u8,
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<(), ()> {
    player.lock().await.set_volume(volume);
    Ok(())
}
