use std::time::Instant;

use boba_core::pearl::SimpleEvent;

pub struct Update {
    delta_time: f32,
}

impl SimpleEvent for Update {}

impl Update {
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}

pub(crate) struct UpdateTimer {
    instant: Option<Instant>,
}

impl UpdateTimer {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn update(&mut self) -> Update {
        let delta_time = match self.instant.replace(Instant::now()) {
            Some(last) => Instant::now().duration_since(last).as_secs_f32(),
            None => 0f32,
        };

        Update { delta_time }
    }
}
