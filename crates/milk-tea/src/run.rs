use boba_core::{
    world::{WorldAccess, WorldInsert},
    World,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    events::{update::UpdateTimer, window::BeforeRender},
    window::MilkTeaWindowViewCrate,
    MilkTeaWindow,
};

pub fn run_headless(world: &mut World) {
    let mut timer = UpdateTimer::new();
    loop {
        world.trigger_simple(&mut timer.update());
    }
}

pub fn run_windowed(world: &mut World) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    world.insert(MilkTeaWindow::new(window));

    let mut timer = UpdateTimer::new();
    event_loop
        .run(move |event, _target| match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::RedrawRequested => {
                    if let Some((link, _)) = world
                        .iter::<MilkTeaWindow>()
                        .find(|(_, window)| window.native().id() == window_id)
                    {
                        world.trigger_simple(&mut BeforeRender::new(link));
                        world.get_view(link).map(|view| view.render());
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                world.trigger_simple(&mut timer.update());
                for (_, window) in world.iter::<MilkTeaWindow>() {
                    window.native().request_redraw();
                }
            }
            _ => (),
        })
        .unwrap();
}
