use boba_core::{pearl, EventListener};
use winit::window::Window;

use crate::events::MilkTeaUpdate;

#[pearl(listen(MilkTeaUpdate))]
pub struct WindowBuilder {
    pub title: String,
}

impl EventListener<MilkTeaUpdate> for WindowBuilder {
    fn update<'a>(
        event: &mut <MilkTeaUpdate as boba_core::Event>::Data<'a>,
        world: &mut boba_core::BobaWorld,
    ) {
        let mut remove_queue = Vec::new();
        let mut insert_queue = Vec::new();
        for builder in world.iter::<Self>() {
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

#[pearl]
pub struct MilkTeaWindow {
    window: Window,
}

impl MilkTeaWindow {
    pub fn window(&self) -> &Window {
        &self.window
    }
}
