use std::time::Instant;

use boba_core::Pearl;

pub struct Time {
    start: Instant,
    delta_time: f64,
    last_reset: Option<Instant>,
}

impl Pearl for Time {}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            start: Instant::now(),
            delta_time: 0f64,
            last_reset: None,
        }
    }

    pub(crate) fn reset_delta(&mut self) {
        let now = Instant::now();
        self.delta_time = match self.last_reset {
            Some(last) => now.duration_since(last).as_secs_f64(),
            None => 0f64,
        };
        self.last_reset = Some(now);
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn uptime(&self) -> f64 {
        Instant::now().duration_since(self.start).as_secs_f64()
    }
}
