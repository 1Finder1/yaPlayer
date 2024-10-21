use crate::audio::backend::AudioPlayer;
use crate::audio::queue::backend::TrackQueue;
use crate::audio::queue::structs::QueueStatus;
use crate::yandex_music::YandexMusic;
use std::sync::Arc;
use tauri::{State, Window};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn play_playlist(
    user_id: i32,
    playlist_id: i32,
    track_id: Option<i32>,
    window: Window,
    queue: State<'_, Arc<Mutex<TrackQueue>>>,
    yandex: State<'_, Arc<YandexMusic>>,
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<(), ()> {
    let mut q = queue.lock().await;

    println!("{user_id}, {playlist_id} {track_id:?} {}", q.playlist_id);

    if q.playlist_id == playlist_id {
        if track_id.is_some()
            && ((q.current_track.is_some()
                && q.current_track.as_ref().unwrap().id != track_id.unwrap())
                || q.current_track.is_none())
        {
            q.start_queue_from_id(track_id.unwrap());
            drop(q);

            window
                .emit("queue-update", ())
                .expect("Ошибка отправки события");

            player.lock().await.play_current_track().await;
            return Ok(());
        }
        player.lock().await.play_pause();
        return Ok(());
    }

    let playlist = yandex.get_playlist(user_id, playlist_id).await;

    let tracks = playlist.tracks.iter().map(|t| t.track.to_owned()).collect();
    // let tracks = vec![];
    q.change_tracks(tracks);
    q.playlist_id = playlist_id;

    if let Some(track) = track_id {
        q.start_queue_from_id(track)
    }

    window
        .emit("queue-update", q.get_status().await)
        .expect("Ошибка отправки события");

    drop(q);
    player.lock().await.play_current_track().await;

    return Ok(());
}

#[tauri::command]
pub async fn queue_next_track(
    queue: State<'_, Arc<Mutex<TrackQueue>>>,
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<(), ()> {
    let mut q = queue.lock().await;

    q.next_track();
    player.lock().await.play_current_track().await;

    Ok(())
}

#[tauri::command]
pub async fn queue_prev_track(
    queue: State<'_, Arc<Mutex<TrackQueue>>>,
    player: State<'_, Arc<Mutex<AudioPlayer>>>,
) -> Result<(), ()> {
    let mut q = queue.lock().await;

    q.prev_track();
    player.lock().await.play_current_track().await;

    Ok(())
}

#[tauri::command]
pub async fn get_queue_status(queue: State<'_, Arc<Mutex<TrackQueue>>>) -> Result<QueueStatus, ()> {
    let q = queue.lock().await;

    return Ok(QueueStatus {
        prev_track: q.get_prev_track(),
        current_track: q.current_track.clone(),
        next_track: q.get_next_track(),
    });
}
