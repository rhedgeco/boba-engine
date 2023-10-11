use boba_core::{pearl::Handle, BobaWorld, EventListener, EventRegister, Pearl};
use winit::window::{Window, WindowId};

use crate::events::{CloseRequest, MilkTeaUpdate, RedrawRequest};

pub trait WindowRenderer: Sized + 'static {
    fn init(handle: Handle<MilkTeaWindow<Self>>, window: Window) -> Self;
    fn id(&self) -> WindowId;
    fn render(&mut self, world: &BobaWorld);
}

pub struct WindowConfig {
    pub title: String,
    pub exit_on_close: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: format!("Milk Tea"),
            exit_on_close: true,
        }
    }
}

pub struct MilkTeaWindow<R: WindowRenderer> {
    init_config: WindowConfig,
    renderer: Option<R>,
}

impl<R: WindowRenderer> Pearl for MilkTeaWindow<R> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaUpdate>();
        register.event::<RedrawRequest>();
        register.event::<CloseRequest>();
    }
}

impl<R: WindowRenderer> EventListener<MilkTeaUpdate> for MilkTeaWindow<R> {
    fn update<'a>(
        event: &mut <MilkTeaUpdate as boba_core::Event>::Data<'a>,
        world: &mut boba_core::BobaWorld,
    ) {
        let mut remove_queue = Vec::new();
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            if window.renderer.is_some() {
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
            window.renderer = Some(R::init(window.handle(), new_window));
        }

        for handle in remove_queue {
            world.remove(handle);
        }
    }
}

impl<R: WindowRenderer> EventListener<RedrawRequest> for MilkTeaWindow<R> {
    fn update<'a>(
        event: &mut <RedrawRequest as boba_core::Event>::Data<'a>,
        world: &mut boba_core::BobaWorld,
    ) {
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            let Some(renderer) = &mut window.renderer else {
                continue;
            };

            if renderer.id() == event.id() {
                renderer.render(&world);
            }
        }
    }
}

impl<R: WindowRenderer> EventListener<CloseRequest> for MilkTeaWindow<R> {
    fn update<'a>(event: &mut <CloseRequest as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        let mut handle = None;
        for window in world.iter::<Self>().filter_map(|e| e.borrow()) {
            let Some(renderer) = &window.renderer else {
                continue;
            };

            if renderer.id() != event.id() {
                continue;
            }

            if !window.init_config.exit_on_close {
                break;
            }

            handle = Some(window.handle());
        }

        if let Some(handle) = handle {
            world.remove(handle);
        }
    }
}

impl<R: WindowRenderer> MilkTeaWindow<R> {
    pub fn new(config: WindowConfig) -> Self {
        Self {
            init_config: config,
            renderer: None,
        }
    }
}
