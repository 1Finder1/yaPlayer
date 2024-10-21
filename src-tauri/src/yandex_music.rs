use std::fs::File;
use std::io::{copy, Cursor};
use std::sync::{Arc};
use tempfile::TempDir;
use yandex_music::model::account_model::account::Account;
use yandex_music::model::playlist_model::playlist::Playlist;
use yandex_music::YandexMusicClient;

pub struct YandexMusic {
    pub client: Arc<YandexMusicClient>, pub account: Account,
    temp_dir: TempDir,
}

impl YandexMusic {
    pub async fn new(temp_dir: TempDir) -> Self {
        let client = YandexMusicClient::new("y0_AgAAAAAuS3T3AAG8XgAAAAD7bHMSAAABBSZO0gVMR48VrqbTglP9-lR-hg");
        let account = client.get_account_status().await.unwrap().account;
        Self {
            client: Arc::new(client),
            account,
            temp_dir,
        }
    }

    pub async fn get_playlist(&self, user_id: i32, playlist_id: i32) -> Playlist {
        self.client.get_playlist(user_id, playlist_id).await.unwrap()
    }

    fn get_track_name(track_id:i32) -> String {
        format!("ya_track{}.mp3", track_id)
    }

    pub async fn download_track(&self, track_id: i32) -> String {
        let filename = Self::get_track_name(track_id);
        let filepath = self.temp_dir.path().join(filename);

        if filepath.exists() {
            return filepath.into_os_string().into_string().unwrap();
        }

        let client = self.client.clone();

        let download_info = client.get_track_download_info(track_id).await.unwrap();
        let info = download_info.iter().max_by_key(|item| item.bitrate_in_kbps).unwrap();

        let url = info.get_direct_link(&client.client).await.unwrap();
        println!("Download track url {}", url);

        let response = reqwest::get(url).await.expect("Ошибка получения файла");

        println!("{:?}", filepath);

        let mut file = File::create(filepath.clone()).unwrap();
        let mut cursor = Cursor::new(response.bytes().await.unwrap());

        copy(&mut cursor, &mut file).unwrap();

        return filepath.into_os_string().into_string().unwrap();
    }
}