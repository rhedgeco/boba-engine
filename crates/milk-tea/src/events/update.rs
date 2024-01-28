use std::time::Instant;

use boba_core::pearl::Event;

pub struct MilkTeaUpdate {
    instant: Option<Instant>,
}

impl Event for MilkTeaUpdate {
    type Data<'a> = f32;
}

impl MilkTeaUpdate {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn next_delta(&mut self) -> f32 {
        match self.instant.replace(Instant::now()) {
            Some(last) => Instant::now().duration_since(last).as_secs_f32(),
            None => 0f32,
        }
    }
}
