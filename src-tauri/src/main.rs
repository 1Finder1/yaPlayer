// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod audio;
mod commands;
mod yandex_music;

use crate::audio::backend::AudioPlayer;
use crate::audio::queue::backend::TrackQueue;
use crate::audio::structs::Event;
use crate::commands::api::{get_my_playlists, get_playlist};
use crate::commands::player::{get_player_info, play_pause, player_seek, player_volume};
use crate::commands::queue::{get_queue_status, play_playlist, queue_next_track, queue_prev_track};
use crate::yandex_music::YandexMusic;
use rodio::Sink;
use std::fs;
use std::sync::Arc;
use tauri::{Manager, WindowEvent};
use tempfile::tempdir;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let tempdir = tempdir().unwrap();
    let tempdir_path = tempdir.path().to_owned();
    let (event_tx, event_rx) = crossbeam_channel::bounded::<Event>(32);

    let yandex = Arc::new(YandexMusic::new(tempdir).await);
    let player_yandex = yandex.clone();

    let queue = Arc::new(Mutex::new(TrackQueue::new()));
    let player_queue = queue.clone();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Arc::new(Sink::try_new(&handle).unwrap());

    let player = Arc::new(Mutex::new(AudioPlayer::new(
        player_queue,
        event_tx,
        player_yandex,
        sink,
    )));
    let loop_player = player.clone();

    tauri::Builder::default()
        .manage(queue)
        .manage(yandex)
        .manage(player)
        .on_window_event(move |event| match event.event() {
            WindowEvent::CloseRequested { .. } => {
                fs::remove_dir_all(tempdir_path.as_path())
                    .expect("Ошибка при удалении временной папки");
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            play_playlist,
            get_playlist,
            get_queue_status,
            get_my_playlists,
            get_player_info,
            play_pause,
            player_seek,
            queue_next_track,
            queue_prev_track,
            player_volume
        ])
        .setup(|app| {
            let app_handle = app.handle();

            tauri::async_runtime::spawn(async move {
                let player = loop_player.clone();

                while let Ok(event) = event_rx.recv() {
                    match event {
                        // Event::Initialize => {}
                        // Event::TracksFetched(_) => {}
                        Event::TrackEnded => {
                            player.lock().await.on_track_end().await;
                        }
                        Event::TrackPosition(pos) => app_handle
                            .emit_all("player-position", pos)
                            .expect("Ошибка отправки события"),
                        Event::TrackChanged => app_handle
                            .emit_all(
                                "queue-update",
                                player.lock().await.queue.lock().await.get_status().await,
                            )
                            .expect("Ошибка отправки события"),
                        Event::PlayerPlaying(is_plying) => app_handle
                            .emit_all("player-playing", is_plying)
                            .expect("Ошибка отправки сообщения"),
                        Event::Volume(vol) => app_handle
                            .emit_all("volume-change", vol)
                            .expect("Ошибка отправки события"),
                    };
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("!!!!After")
}
