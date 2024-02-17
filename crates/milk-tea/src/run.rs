use boba_core::World;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::events::{
    app::{Init, Resume, Suspend},
    base::EventTimer,
    window::{Close, Focus, Redraw, Resize},
    MilkTea, Update,
};

pub fn run(world: &mut World) {
    run_with_flow(world, true);
}

pub fn run_with_flow(world: &mut World, poll: bool) {
    // create event loop
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            log::error!("Failed to create event loop. Error: {error}");
            return;
        }
    };

    // set the polling mode according to poll choice
    event_loop.set_control_flow(match poll {
        false => ControlFlow::Wait,
        true => ControlFlow::Poll,
    });

    // create a timer to create update events every loop
    let mut timer = EventTimer::new();

    // run the event loop
    if let Err(error) = event_loop.run(move |event, target| {
        match event {
            Event::NewEvents(StartCause::Init) => {
                // initial update of timer to start game time
                timer.update_timer();

                // then run the init event
                let event = &mut timer.build_simple(Init::new(), target);
                world.trigger::<MilkTea<Init>>(event);
            }
            Event::Resumed => {
                let event = &mut timer.build_simple(Resume::new(), target);
                world.trigger::<MilkTea<Resume>>(event);
            }
            Event::Suspended => {
                let event = &mut timer.build_simple(Suspend::new(), target);
                world.trigger::<MilkTea<Suspend>>(event);
            }
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::RedrawRequested => {
                    let event = &mut timer.build_simple(Redraw::new(window_id), target);
                    world.trigger::<MilkTea<Redraw>>(event);
                }
                WindowEvent::CloseRequested => {
                    let event = &mut timer.build_simple(Close::new(window_id), target);
                    world.trigger::<MilkTea<Close>>(event);
                }
                WindowEvent::Resized(size) => {
                    let event = &mut timer.build_simple(Resize::new(window_id, size), target);
                    world.trigger::<MilkTea<Resize>>(event);
                }
                WindowEvent::Focused(focused) => {
                    let event = &mut timer.build_simple(Focus::new(window_id, focused), target);
                    world.trigger::<MilkTea<Focus>>(event);
                }
                _ => (),
            },
            Event::AboutToWait => {
                // trigger a world update
                let event = &mut timer.build_simple(Update, target);
                world.trigger::<MilkTea<Update>>(event);

                // update the inner timer values
                timer.update_timer();
            }
            _ => (),
        }
    }) {
        log::error!("Failed to execute event loop. Error: {error}");
    }
}
