use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Default)]
pub struct TrackPosition {
    pub current_position: Arc<RwLock<Duration>>,
    pub total: Arc<RwLock<Duration>>
}

impl TrackPosition {
    pub fn set_current_position(&self, position: Duration) {
        if let Ok(mut current) = self.current_position.write() {
            *current = position
        }
    }

    pub fn set_total_duration(&self, duration: Duration) {
        if let Ok(mut total) = self.total.write() {
            *total = duration
        }
    }

    pub fn reset(&self) {
        self.set_current_position(Duration::ZERO);
        self.set_total_duration(Duration::ZERO);
    }

    pub fn get_progress(&self) -> (Duration, Duration) {
        (
            *self.current_position.read().unwrap(),
            *self.total.read().unwrap(),
        )
    }
}
