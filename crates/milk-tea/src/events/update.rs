use std::time::Instant;

use boba_core::{pearl::collections::PearlArena, Resources};

pub struct MilkTeaUpdater {
    instant: Option<Instant>,
}

impl MilkTeaUpdater {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn update(&mut self, pearls: &mut PearlArena, resources: &mut Resources) {
        let now = Instant::now();
        let delta_time = match self.instant {
            Some(last) => now.duration_since(last).as_secs_f64(),
            None => 0f64,
        };
        self.instant = Some(now);

        pearls.trigger(&mut Update { delta_time }, resources);
        pearls.trigger(&mut LateUpdate { delta_time }, resources);
    }
}

pub struct Update {
    delta_time: f64,
}

impl Update {
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

pub struct LateUpdate {
    delta_time: f64,
}

impl LateUpdate {
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
