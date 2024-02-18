use boba_core::World;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::events::{
    app::{Init, Resume, Suspend, Update},
    milktea::EventTimer,
    window::{Close, Focus, Redraw, Resize},
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
        let executor = timer.next(target);
        match event {
            Event::NewEvents(StartCause::Init) => {
                executor.trigger(world, Init::new());
            }
            Event::Resumed => {
                executor.trigger(world, Resume::new());
            }
            Event::Suspended => {
                executor.trigger(world, Suspend::new());
            }
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::RedrawRequested => {
                    executor.trigger(world, Redraw::new(window_id));
                }
                WindowEvent::CloseRequested => {
                    executor.trigger(world, Close::new(window_id));
                }
                WindowEvent::Resized(size) => {
                    executor.trigger(world, Resize::new(window_id, size));
                }
                WindowEvent::Focused(focused) => {
                    executor.trigger(world, Focus::new(window_id, focused));
                }
                _ => (),
            },
            Event::AboutToWait => {
                executor.trigger(world, Update::new());
            }
            _ => (),
        }
    }) {
        log::error!("Failed to execute event loop. Error: {error}");
    }
}
