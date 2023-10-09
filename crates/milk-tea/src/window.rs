use boba_core::{pearl, BobaWorld, EventListener};
use winit::window::Window;

use crate::events::{MilkTeaUpdate, UpdateData};

#[pearl(listen(MilkTeaUpdate))]
pub struct WindowBuilder {
    pub title: String,
}

impl EventListener<MilkTeaUpdate> for WindowBuilder {
    fn update<'a>(event: &mut UpdateData, world: &mut BobaWorld) {
        let mut remove_queue = Vec::new();
        let mut insert_queue = Vec::new();
        for builder in world.pearls.iter::<Self>() {
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
            world.pearls.remove(handle);
        }

        for window in insert_queue {
            world.pearls.insert(window);
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
