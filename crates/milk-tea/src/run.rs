use boba_core::World;
use winit::{dpi::LogicalSize, event::Event, event_loop::EventLoop, window::WindowBuilder};

use crate::{events::update::UpdateTimer, MilkTeaWindow};

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
            Event::AboutToWait => {
                world.trigger_simple(&mut timer.update());
                for (_, w) in world.iter::<MilkTeaWindow>() {
                    w.window().request_redraw();
                }
            }
            _ => (),
        })
        .unwrap();
}
