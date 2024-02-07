use boba_core::{
    world::{Link, WorldAccess, WorldInsert, WorldRemove},
    World,
};
use indexmap::IndexMap;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{
    events::{update::UpdateTimer, window::CloseRequest},
    window::{MilkTeaId, MilkTeaWindowSettings, MilkTeaWindowViewCrate},
    MilkTeaWindow,
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
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(match poll {
        false => ControlFlow::Wait,
        true => ControlFlow::Poll,
    });

    let mut window_library = IndexMap::<WindowId, WindowEntry>::new();
    let mut timer = UpdateTimer::new();
    event_loop
        .run(move |event, target| match event {
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
                // trigger a world update
                world.trigger_simple(&mut timer.update());
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
        })
        .unwrap();
}
