use crate::audio::queue::structs::{QueueRepeatMode, QueueStatus};
use rand::seq::SliceRandom;
use rand::thread_rng;
use yandex_music::model::track_model::track::Track;

pub struct TrackQueue {
    orig_tracks: Vec<Track>,
    tracks_indexes_queue: Vec<usize>,
    pub current_track: Option<Track>,
    track_index: usize,

    pub is_shuffle: bool,
    pub repeat_mode: QueueRepeatMode,
    pub playlist_id: i32,
}

impl TrackQueue {
    // Создание объекта
    pub fn new() -> Self {
        Self {
            orig_tracks: vec![],
            tracks_indexes_queue: vec![],

            track_index: 0,
            current_track: None,

            is_shuffle: false,
            repeat_mode: QueueRepeatMode::None,

            playlist_id: 0,
        }
    }

    pub async fn get_status(&self) -> QueueStatus {
        QueueStatus {
            prev_track: self.get_prev_track(),
            current_track: self.current_track.clone(),
            next_track: self.get_next_track(),
        }
    }
}

impl TrackQueue {
    // Следующий трек

    fn get_next_track_index(&self) -> Option<usize> {
        let len = self.tracks_indexes_queue.len();
        let tracks_count = if len > 0 { len - 1 } else { 0 };

        if self.repeat_mode == QueueRepeatMode::Track {
            // Если у нас стоит трек на репите, то ничего не меняем
            return Some(self.track_index);
        } else if self.track_index == tracks_count && self.repeat_mode == QueueRepeatMode::Playlist
        {
            // Если у нас сейчас последний трек и у нас плейлист на репите, то выбираем первый трек
            return Some(0);
        } else if self.track_index + 1 < tracks_count {
            // Если мы не в конце очереди, то выбираем следующий трек
            return Some(self.tracks_indexes_queue[self.track_index + 1]);
        }

        // Если следующего трека нет
        return None;
    }

    pub fn next_track(&mut self) {
        if let Some(index) = self.get_next_track_index() {
            self.current_track = Some(self.orig_tracks[index as usize].clone());
            self.track_index = index
        }
    }

    pub fn get_next_track(&self) -> Option<Track> {
        if let Some(index) = self.get_next_track_index() {
            return Some(self.orig_tracks[index as usize].clone());
        }

        return None;
    }
}

impl TrackQueue {
    // Предыдущий трек

    fn get_prev_track_index(&self) -> Option<usize> {
        if self.track_index >= 1 {
            return Some(self.tracks_indexes_queue[self.track_index - 1]);
        }
        return None;
    }

    pub fn prev_track(&mut self) {
        if let Some(index) = self.get_prev_track_index() {
            self.current_track = Some(self.orig_tracks[index as usize].clone());
            self.track_index = index;
        }
    }

    pub fn get_prev_track(&self) -> Option<Track> {
        if let Some(index) = self.get_prev_track_index() {
            return Some(self.orig_tracks[index as usize].clone());
        }

        return None;
    }
}

impl TrackQueue {
    fn get_tracks_index_range(&self) -> Vec<usize> {
        (0..self.orig_tracks.len()).collect()
    }
    fn shuffle_tracks(&self, tracks: &mut Vec<usize>) {
        tracks.shuffle(&mut thread_rng());
    }

    pub fn shuffle_current_tracks(&mut self) {
        let mut t: Vec<usize> = (1..self.orig_tracks.len()).collect();

        self.shuffle_tracks(&mut t);

        t.splice(0..0, vec![0]);

        self.tracks_indexes_queue = t
    }

    pub fn change_shuffle(&mut self, shuffle: bool) {
        if self.is_shuffle == shuffle {
            return;
        }

        let mut tracks: Vec<usize>;

        if shuffle {
            // Если надо перемешать треки, то клонируем оригинальный порядок треков и мешаем их
            tracks = self.get_tracks_index_range();
            self.shuffle_tracks(&mut tracks)
        } else {
            // Если мы выключили перемешку треков, то применяем оригинальный порядок треков
            tracks = self.get_tracks_index_range();
        }

        if self.current_track.is_some() {
            // Если есть текущий трек, то меняем индекс на новый.
            self.track_index = tracks
                .iter()
                .find(|item| item.to_owned().to_owned() == self.track_index)
                .unwrap()
                .to_owned();
        }

        self.tracks_indexes_queue = tracks;
    }

    pub fn change_repeat(&mut self, mode: QueueRepeatMode) {
        self.repeat_mode = mode
    }

    pub fn change_tracks(&mut self, tracks: Vec<Track>) {
        self.orig_tracks = tracks;

        self.current_track = Some(self.orig_tracks[0].clone());
        self.track_index = 0;

        if self.is_shuffle {
            self.shuffle_current_tracks();
        } else {
            self.tracks_indexes_queue = self.get_tracks_index_range();
        }
    }

    pub fn play_by_id(&mut self, track_id: i32) {
        match self.orig_tracks.iter().position(|t| t.id == track_id) {
            None => {}
            Some(index) => {
                self.current_track = Some(self.orig_tracks[index].clone());
                self.track_index = index;
            }
        }
    }

    pub fn start_queue_from_id(&mut self, track_id: i32) {
        let mut tracks_queue = self.tracks_indexes_queue.clone();

        let track_index = self.orig_tracks.iter().position(|t| t.id == track_id);

        if track_index.is_none() {
            return;
        }

        let queue_index = tracks_queue
            .iter()
            .position(|i| i.to_owned() == track_index.unwrap());

        println!("{tracks_queue:?} {track_index:?} {queue_index:?}");

        if let Some(index) = queue_index {
            if index == 0 {
                return;
            }

            let tracks_before: Vec<usize> = tracks_queue.drain(0..index).collect();

            println!("{}", tracks_before.last().unwrap());

            tracks_queue.extend(tracks_before);

            self.current_track = Some(self.orig_tracks[tracks_queue[0]].clone());
        }

        self.tracks_indexes_queue = tracks_queue
    }
}
