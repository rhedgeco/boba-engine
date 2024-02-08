use boba_core::{
    world::{Link, WorldAccess, WorldInsert, WorldRemove},
    World,
};
use indexmap::IndexMap;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{
    events::{
        app::{Init, Resume, Suspend},
        update::UpdateTimer,
        window::CloseRequest,
    },
    pearls::{
        window::{MilkTeaId, MilkTeaWindowViewCrate},
        MilkTeaWindow, MilkTeaWindowSettings,
    },
};

#[derive(Clone, Copy)]
pub(crate) struct WindowEntry {
    pub link: Link<MilkTeaWindow>,
    pub id: MilkTeaId,
}

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

    // create a library for storing mapping between window id and other window data
    let mut window_library = IndexMap::<WindowId, WindowEntry>::new();

    // create a timer to create update events every loop
    let mut timer = UpdateTimer::new();

    // run the event loop
    if let Err(error) = event_loop.run(move |event, target| match event {
        Event::NewEvents(StartCause::Init) => world.trigger_simple(&mut Init::new()),
        Event::Resumed => world.trigger_simple(&mut Resume::new()),
        Event::Suspended => world.trigger_simple(&mut Suspend::new()),
        Event::WindowEvent { window_id, event } => {
            let Some(entry) = window_library.get(&window_id).cloned() else {
                log::error!("received a window event but window was not accounted for");
                return;
            };

            match event {
                WindowEvent::RedrawRequested => {
                    if let Some(mut window) = world.get_view(entry.link) {
                        window.render();
                    }
                }
                WindowEvent::CloseRequested => {
                    world.trigger_simple(&mut CloseRequest::new(entry.link, entry.id));
                }
                _ => (),
            }
        }
        Event::AboutToWait => {
            // trigger a world update and exit if needed
            let update = &mut timer.update();
            world.trigger_simple(update);
            if update.will_quit() {
                target.exit();
                return;
            }

            // flush destroy queue
            world.flush_destroy_queue();

            // create pending windows
            while let Some((_, settings)) = world.pop::<MilkTeaWindowSettings>() {
                match MilkTeaWindow::new(settings, target) {
                    Ok(window) => {
                        let id = window.id();
                        let native_id = window.native().id();
                        let link = world.insert(window);
                        window_library.insert(native_id, WindowEntry { link, id });
                    }
                    Err(e) => {
                        log::error!("Failed to create window. Error: {e}");
                        continue;
                    }
                };
            }
        }
        _ => (),
    }) {
        log::error!("Failed to execute event loop. Error: {error}");
    }
}
