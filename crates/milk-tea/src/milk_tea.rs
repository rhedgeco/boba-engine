use boba_core::World;

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
    pub world: World,
}

impl MilkTeaRunner {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn run(&mut self) {
        let mut updater = MilkTeaUpdater::new();

        loop {
            match self.world.get_resource::<MilkTeaSettings>() {
                Some(settings) => {
                    if settings.exit {
                        break;
                    }
                }
                None => {
                    self.world.insert_resource(MilkTeaSettings::default());
                }
            }

            updater.update(&mut self.world);
        }
    }
}
