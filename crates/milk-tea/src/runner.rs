use boba_core::{pearl::collections::PearlArena, Resources};

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
    pub pearls: PearlArena,
    pub resources: Resources,
}

impl MilkTeaRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        let mut updater = MilkTeaUpdater::new();

        loop {
            match self.resources.get::<MilkTeaSettings>() {
                Some(settings) => {
                    if settings.exit {
                        break;
                    }
                }
                None => {
                    self.resources.insert(MilkTeaSettings::default());
                }
            }

            updater.update(&mut self.pearls, &mut self.resources);
        }
    }
}
