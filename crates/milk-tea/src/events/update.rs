use std::time::Instant;

use boba_core::pearl::Event;
use winit::event_loop::EventLoopWindowTarget;

pub struct Update;

impl Event for Update {
    type Data<'a> = UpdateData<'a>;
}

pub struct UpdateData<'a> {
    target: &'a EventLoopWindowTarget<()>,
    delta_time: f32,
}

impl UpdateData<'_> {
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn window_target(&self) -> &EventLoopWindowTarget<()> {
        self.target
    }
}

pub(crate) struct UpdateTimer {
    instant: Option<Instant>,
}

impl UpdateTimer {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub(crate) fn update<'a>(&mut self, target: &'a EventLoopWindowTarget<()>) -> UpdateData<'a> {
        let delta_time = match self.instant.replace(Instant::now()) {
            Some(last) => Instant::now().duration_since(last).as_secs_f32(),
            None => 0f32,
        };

        UpdateData { target, delta_time }
    }
}
