use boba_core::BobaWorld;
use winit::event_loop::EventLoop;

use crate::events::{MilkTeaExit, MilkTeaStart, MilkTeaUpdate};

pub struct MilkTeaSettings {
    pub close_when_no_windows: bool,
}

impl Default for MilkTeaSettings {
    fn default() -> Self {
        Self {
            close_when_no_windows: true,
        }
    }
}

pub struct MilkTea {
    pub world: BobaWorld,
    pub settings: MilkTeaSettings,
}

impl Default for MilkTea {
    fn default() -> Self {
        Self {
            world: Default::default(),
            settings: Default::default(),
        }
    }
}

impl MilkTea {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();
        let mut update = MilkTeaUpdate::new();

        use winit::event::Event as E;
        event_loop.run(move |event, target, control| match event {
            E::NewEvents(cause) => match cause {
                winit::event::StartCause::Init => {
                    let start_data = MilkTeaStart::create_data(target, control);
                    self.world.trigger::<MilkTeaStart>(start_data);
                }
                _ => {}
            },
            E::MainEventsCleared => {
                self.world
                    .trigger::<MilkTeaUpdate>(update.next_data(target, control));
            }
            E::LoopDestroyed => self.world.trigger::<MilkTeaExit>(()),
            _ => {}
        });
    }
}
