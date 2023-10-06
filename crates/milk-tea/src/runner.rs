use boba_core::arena::BobaArena;

use crate::events::MilkTeaUpdater;

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
    pub arena: BobaArena,
}

impl MilkTeaRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        let mut updater = MilkTeaUpdater::new();

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

            updater.update(&mut self.arena);
        }
    }
}
