use boba_core::BobaWorld;
use winit::event_loop::EventLoop;

use crate::{
    events::{
        CloseRequest, MilkTeaExit, MilkTeaStart, MilkTeaUpdate, RedrawRequest, Resume, Suspend,
        WindowInit, WindowInitData,
    },
    pearls::{ControlFlow, Time},
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
        let mut suspended = true;
        let event_loop = EventLoop::new();
        self.world.insert_static(Time::new());
        self.world.insert_static(ControlFlow::new());

        use winit::event::Event as E;
        event_loop.run(move |event, target, control| {
            match event {
                E::NewEvents(cause) => {
                    // reset delta time for the next iteration
                    if let Some(mut time) = self.world.get_static_mut::<Time>() {
                        time.reset_delta();
                    }

                    // apply control flow actions
                    if let Some(control_pearl) = self.world.get_static_mut::<ControlFlow>() {
                        if control_pearl.will_exit() {
                            control.set_exit()
                        }
                    }

                    // run start cause based events
                    use winit::event::StartCause as Cause;
                    match cause {
                        // call start event if the loop was just initiated
                        Cause::Init => self.world.trigger::<MilkTeaStart>(()),
                        _ => {}
                    }
                }
                E::WindowEvent { window_id, event } => {
                    use winit::event::WindowEvent as WE;
                    match event {
                        WE::CloseRequested => {
                            let close = CloseRequest::new(window_id);
                            self.world.trigger::<CloseRequest>(close);
                        }
                        _ => {}
                    }
                }
                E::Resumed => {
                    suspended = false;
                    self.world.trigger::<Resume>(());
                }
                E::Suspended => {
                    suspended = true;
                    self.world.trigger::<Suspend>(());
                }
                E::RedrawRequested(id) if !suspended => {
                    self.world.trigger::<RedrawRequest>(RedrawRequest::new(id));
                }
                E::MainEventsCleared => {
                    if !suspended {
                        let init_data = WindowInitData::new(target);
                        self.world.trigger::<WindowInit>(init_data);
                    }

                    self.world.trigger::<MilkTeaUpdate>(());
                }
                E::LoopDestroyed => self.world.trigger::<MilkTeaExit>(()),
                _ => {}
            }
        });
    }
}
