use boba_core::{
    world::{WorldAccess, WorldRemove},
    World,
};
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::events::{
    app::{Init, Resume, Suspend},
    update::UpdateTimer,
    window::{CloseRequest, FocusChanged, RedrawRequest, WindowResized},
    Update,
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
    let mut timer = UpdateTimer::new();

    // run the event loop
    if let Err(error) = event_loop.run(move |event, target| match event {
        Event::NewEvents(StartCause::Init) => world.trigger_simple(&mut Init::new()),
        Event::Resumed => world.trigger_simple(&mut Resume::new()),
        Event::Suspended => world.trigger_simple(&mut Suspend::new()),
        Event::WindowEvent { window_id, event } => match event {
            WindowEvent::RedrawRequested => {
                world.trigger_simple(&mut RedrawRequest::new(window_id));
            }
            WindowEvent::CloseRequested => {
                world.trigger_simple(&mut CloseRequest::new(window_id));
            }
            WindowEvent::Resized(size) => {
                world.trigger_simple(&mut WindowResized::new(window_id, size));
            }
            WindowEvent::Focused(focused) => {
                world.trigger_simple(&mut FocusChanged::new(window_id, focused));
            }
            _ => (),
        },
        Event::AboutToWait => {
            // trigger a world update
            let update_data = &mut timer.update(target);
            world.trigger::<Update>(update_data);

            // flush destroy queue after all events are finished
            world.flush_destroy_queue();
        }
        _ => (),
    }) {
        log::error!("Failed to execute event loop. Error: {error}");
    }
}
