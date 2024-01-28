use boba_core::World;
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

use crate::events::MilkTeaUpdate;

pub fn run_headless(world: &mut World) {
    let mut update = MilkTeaUpdate::new();
    loop {
        let delta_time = update.next_delta();
        world.trigger::<MilkTeaUpdate>(&delta_time);
    }
}

pub fn run_windowed(world: &mut World) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Milk Tea Window")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    let mut update = MilkTeaUpdate::new();
    event_loop
        .run(move |event, _target| match event {
            Event::AboutToWait => {
                let delta_time = update.next_delta();
                world.trigger::<MilkTeaUpdate>(&delta_time);
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
