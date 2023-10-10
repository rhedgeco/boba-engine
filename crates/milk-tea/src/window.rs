use boba_core::{EventListener, EventRegister, Pearl};
use winit::window::Window;

use crate::events::MilkTeaUpdate;

pub struct WindowConfig {
    pub title: String,
}

pub struct MilkTeaWindow {
    init_config: WindowConfig,
    window: Option<Window>,
}

impl Pearl for MilkTeaWindow {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaUpdate>();
    }
}

impl EventListener<MilkTeaUpdate> for MilkTeaWindow {
    fn update<'a>(
        event: &mut <MilkTeaUpdate as boba_core::Event>::Data<'a>,
        world: &mut boba_core::BobaWorld,
    ) {
        let mut remove_queue = Vec::new();
        for mut window in world.iter::<MilkTeaWindow>().filter_map(|e| e.borrow_mut()) {
            if window.window.is_some() {
                continue;
            }

            let new_window = match Window::new(event.window_target()) {
                Ok(window) => window,
                Err(e) => {
                    remove_queue.push(window.handle());
                    let title = &window.init_config.title;
                    eprintln!("Failed to create window '{title}': {e}");
                    continue;
                }
            };

            new_window.set_title(&window.init_config.title);
            window.window = Some(new_window);
        }
    }
}

impl MilkTeaWindow {
    pub fn new(config: WindowConfig) -> Self {
        Self {
            init_config: config,
            window: None,
        }
    }

    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }
}
