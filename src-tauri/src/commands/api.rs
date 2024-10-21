use crate::yandex_music::YandexMusic;
use std::sync::Arc;
use tauri::State;
use yandex_music::model::playlist_model::playlist::Playlist;

#[tauri::command]
pub async fn get_my_playlists(yandex: State<'_, Arc<YandexMusic>>) -> Result<Vec<Playlist>, ()> {
    let user_id = yandex.account.uid.unwrap();
    let playlists = yandex.client.get_all_playlists(user_id).await.unwrap();

    return Ok(playlists);
}

#[tauri::command]
pub async fn get_playlist(
    playlist_id: i32,
    owner_id: i32,
    yandex: State<'_, Arc<YandexMusic>>,
) -> Result<Playlist, ()> {
    let playlist = yandex
        .client
        .get_playlist(owner_id, playlist_id)
        .await
        .unwrap();

    Ok(playlist)
}
