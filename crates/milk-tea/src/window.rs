use boba_core::{EventListener, EventRegister, Pearl};
use winit::window::Window;

use crate::events::MilkTeaUpdate;

pub struct WindowBuilder {
    pub title: String,
}

impl Pearl for WindowBuilder {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaUpdate>();
    }
}

impl EventListener<MilkTeaUpdate> for WindowBuilder {
    fn update<'a>(
        event: &mut <MilkTeaUpdate as boba_core::Event>::Data<'a>,
        world: &mut boba_core::BobaWorld,
    ) {
        let mut remove_queue = Vec::new();
        let mut insert_queue = Vec::new();
        for builder in world.iter::<Self>().filter_map(|e| e.borrow()) {
            remove_queue.push(builder.handle());
            match Window::new(event.window_target()) {
                Err(e) => {
                    eprintln!("Failed to create window: {e}");
                }
                Ok(window) => {
                    window.set_title(&builder.title);
                    insert_queue.push(MilkTeaWindow { window });
                }
            };
        }

        for handle in remove_queue {
            world.remove(handle);
        }

        for window in insert_queue {
            world.insert(window);
        }
    }
}

pub struct MilkTeaWindow {
    window: Window,
}

impl Pearl for MilkTeaWindow {}

impl MilkTeaWindow {
    pub fn window(&self) -> &Window {
        &self.window
    }
}
