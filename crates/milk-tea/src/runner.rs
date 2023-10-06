use boba_core::arena::BobaArena;
use std::time::Instant;

pub struct Update {
    delta_time: f64,
}

impl Update {
    fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

pub struct LateUpdate {
    delta_time: f64,
}

impl LateUpdate {
    fn new(delta_time: f64) -> Self {
        Self { delta_time }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}

#[derive(Default)]
pub struct MilkTeaSettings {
    exit: bool,
}

impl MilkTeaSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

#[derive(Default)]
pub struct MilkTeaRunner {
    instant: Option<Instant>,
    pub arena: BobaArena,
}

impl MilkTeaRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        loop {
            match self.arena.resources().get::<MilkTeaSettings>() {
                Some(settings) => {
                    if settings.exit {
                        break;
                    }
                }
                None => {
                    self.arena
                        .resources_mut()
                        .insert(MilkTeaSettings::default());
                }
            }

            let now = Instant::now();
            let delta_time = match self.instant {
                Some(last) => now.duration_since(last).as_secs_f64(),
                None => 0f64,
            };
            self.instant = Some(now);

            self.arena.trigger(&mut Update::new(delta_time));
            self.arena.trigger(&mut LateUpdate::new(delta_time));
        }
    }
}
