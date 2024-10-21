use crate::audio::queue::backend::TrackQueue;
use crate::audio::structs::Event;
use crate::audio::track_position::TrackPosition;
use crate::yandex_music::YandexMusic;
use crossbeam_channel::Sender;
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct AudioPlayer {
    // stream: OutputStream,
    pub sink: Arc<Sink>,
    // stream_config: StreamConfig,
    client: Arc<YandexMusic>,
    event_tx: Sender<Event>,

    pub queue: Arc<Mutex<TrackQueue>>,
    pub track_position: Arc<TrackPosition>,
    pub is_playing: Arc<AtomicBool>,
    pub is_muted: bool,
    pub volume: u8,
}

impl AudioPlayer {
    pub fn new(
        queue: Arc<Mutex<TrackQueue>>,
        event_tx: Sender<Event>,
        client: Arc<YandexMusic>,
        sink: Arc<Sink>,
    ) -> Self {
        // let (stream, sink, stream_config) = init_player().unwrap();

        let player = Self {
            // stream,
            sink,
            // stream_config,
            client,
            event_tx,

            queue,
            track_position: Arc::new(TrackPosition::default()),
            is_playing: Arc::new(AtomicBool::new(false)),
            is_muted: false,
            volume: 100,
        };

        let progress = player.track_position.clone();
        let sink = player.sink.clone();
        let event_tx = player.event_tx.clone();
        let playing = player.is_playing.clone();

        thread::spawn(move || loop {
            let pos = sink.get_pos();

            let is_playing = playing.load(Ordering::Relaxed);

            if is_playing {
                progress.set_current_position(pos);
                event_tx.send(Event::TrackPosition(pos.as_secs())).unwrap();
            }

            if is_playing && sink.empty() {
                println!("Send end event");
                playing.store(false, Ordering::Relaxed);
                event_tx.send(Event::TrackEnded).unwrap()
            }
            thread::sleep(Duration::from_secs(1));
        });

        return player;
    }

    pub async fn play_track(&self, path: String) {
        self.stop_track();

        let track_progress = self.track_position.clone();
        let playing = self.is_playing.clone();

        let file = File::open(path).unwrap();
        let file_buf = BufReader::new(file);

        let decoder = Decoder::new_mp3(file_buf).unwrap();

        if let Some(total) = decoder.total_duration() {
            track_progress.set_total_duration(total);
        } else {
            track_progress.set_total_duration(Duration::from_secs(
                self.queue
                    .lock()
                    .await
                    .current_track
                    .as_ref()
                    .unwrap()
                    .duration_ms
                    .unwrap_or(0) as u64,
            ));
        }

        self.sink.append(decoder);
        playing.store(true, Ordering::Relaxed);
        self.event_tx
            .send(Event::TrackChanged)
            .expect("Ошибка отправки события");
        self.sink.play();
    }

    pub fn stop_track(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
        self.sink.stop();
    }

    pub async fn on_track_end(&mut self) {
        self.queue.lock().await.next_track();

        self.play_current_track().await;
    }

    async fn get_current_track(&self) -> Option<String> {
        let track = self.queue.lock().await.current_track.clone();

        if let Some(track) = track {
            let filepath = self.client.download_track(track.id).await;
            return Some(filepath);
        }

        return None;
    }

    pub async fn play_current_track(&self) {
        let path = self.get_current_track().await.unwrap();

        self.play_track(path).await;
    }

    pub fn play_pause(&self) {
        let is_pause = self.sink.is_paused();
        if is_pause {
            self.sink.play();
        } else {
            self.sink.pause();
            self.track_position
                .set_current_position(self.sink.get_pos())
        }

        self.event_tx
            .send(Event::PlayerPlaying(!is_pause))
            .expect("Ошибка отправки события");
        self.is_playing.store(is_pause, Ordering::Relaxed)
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.is_muted = false;
        self.volume = volume;
        self.sink.set_volume(self.volume as f32 / 100.0);
        self.event_tx.send(Event::Volume(volume)).unwrap()
    }

    pub fn set_mute(&mut self, mute: bool) {
        self.is_muted = mute;

        self.sink.set_volume(if mute {
            0.0
        } else {
            self.volume as f32 / 100.0
        })
    }

    pub fn seek(&self, seconds: u64) {
        self.sink.try_seek(Duration::from_secs(seconds)).unwrap();
    }
}
