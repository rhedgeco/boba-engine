use boba_core::BobaWorld;
use winit::event_loop::EventLoop;

use crate::events::{
    CloseRequest, Exit, MilkTeaEvent, MilkTeaTimer, RedrawRequest, Resumed, Start, Suspended,
    Update,
};

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
        let mut timer = MilkTeaTimer::new();

        use winit::event::Event as E;
        event_loop.run(move |event, target, control| {
            let mut instant = timer.instant(target, control);
            match event {
                E::NewEvents(cause) => match cause {
                    winit::event::StartCause::Init => {
                        self.world
                            .trigger::<MilkTeaEvent<Start>>(instant.build(Start));
                    }
                    _ => {}
                },
                E::WindowEvent { window_id, event } => {
                    use winit::event::WindowEvent as WE;
                    match event {
                        WE::CloseRequested => {
                            self.world.trigger::<MilkTeaEvent<CloseRequest>>(
                                instant.build(CloseRequest::new(window_id)),
                            );
                        }
                        _ => {}
                    }
                }
                E::Resumed => {
                    self.world
                        .trigger::<MilkTeaEvent<Resumed>>(instant.build(Resumed));
                }
                E::Suspended => {
                    self.world
                        .trigger::<MilkTeaEvent<Suspended>>(instant.build(Suspended));
                }
                E::RedrawRequested(id) => {
                    self.world.trigger::<MilkTeaEvent<RedrawRequest>>(
                        instant.build(RedrawRequest::new(id)),
                    );
                }
                E::MainEventsCleared => {
                    self.world
                        .trigger::<MilkTeaEvent<Update>>(instant.build(Update));
                }
                E::LoopDestroyed => {
                    self.world
                        .trigger::<MilkTeaEvent<Exit>>(instant.build(Exit));
                }
                _ => {}
            }
        });
    }
}
