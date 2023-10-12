use crate::events::{CloseRequest, MilkTeaUpdate, RedrawRequest};
use boba_core::{BobaWorld, EventListener, EventRegister, Pearl};
use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};
use thiserror::Error;
use winit::window::{Window, WindowId};

pub trait WindowRenderer: Sized + 'static {
    fn init(target: MilkTeaTarget, window: Window) -> Self;
    fn render(&mut self, world: &mut BobaWorld);
    fn window(&self) -> &Window;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilkTeaTarget {
    id: u64,
}

impl MilkTeaTarget {
    fn new() -> Self {
        static ID: AtomicU64 = AtomicU64::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub struct WindowBuilder<R: WindowRenderer> {
    target: MilkTeaTarget,
    config: WindowConfig,
    _marker: PhantomData<*const R>,
}

impl<R: WindowRenderer> Pearl for WindowBuilder<R> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaUpdate>();
    }
}

impl<R: WindowRenderer> WindowBuilder<R> {
    pub fn new(config: WindowConfig) -> Self {
        Self {
            target: MilkTeaTarget::new(),
            config,
            _marker: PhantomData,
        }
    }
}

impl<R: WindowRenderer> EventListener<MilkTeaUpdate> for WindowBuilder<R> {
    fn update<'a>(
        event: &mut <MilkTeaUpdate as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        for builder in world.into_iter::<Self>() {
            let window = match Window::new(event.window_target()) {
                Ok(w) => w,
                Err(e) => {
                    let target = builder.target;
                    let title = builder.config.title;
                    eprintln!("Failed to create window '{title}' [{target:?}]: {e}");
                    continue;
                }
            };

            window.set_title(&builder.config.title);
            let window = MilkTeaWindow {
                id: window.id(),
                target: builder.target,
                config: builder.config,
                renderer: Some(R::init(builder.target, window)),
            };

            world.insert(window);
        }
    }
}

#[derive(Debug, Error)]
#[error("Cannot access renderer while it is currently rendering.")]
pub struct CurrentlyRendering;

pub struct MilkTeaWindow<R: WindowRenderer> {
    id: WindowId,
    target: MilkTeaTarget,
    config: WindowConfig,
    renderer: Option<R>,
}

impl<R: WindowRenderer> MilkTeaWindow<R> {
    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn target(&self) -> MilkTeaTarget {
        self.target
    }
}

impl<R: WindowRenderer> Pearl for MilkTeaWindow<R> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<CloseRequest>();
        register.event::<RedrawRequest>();
    }
}

impl<R: WindowRenderer> EventListener<CloseRequest> for MilkTeaWindow<R> {
    fn update<'a>(event: &mut <CloseRequest as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        let Some(window) = world.get_where::<Self>(|p| p.id() == event.id()) else {
            return;
        };

        if window.config.exit_on_close {
            let handle = window.handle();
            drop(window);
            world.remove(handle);
        }
    }
}

impl<R: WindowRenderer> EventListener<RedrawRequest> for MilkTeaWindow<R> {
    fn update<'a>(
        event: &mut <RedrawRequest as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        // access the window associated with the requested id
        let Some(mut window) = world.get_mut_where::<Self>(|p| p.id() == event.id()) else {
            return;
        };

        // take the renderer out so that the world may still be used mutably
        let handle = window.handle();
        let mut renderer = match window.renderer.take() {
            Some(renderer) => {
                drop(window);
                renderer
            }
            None => {
                eprintln!(
                    "Tried to render window ['{}':{:?}], but it was in an invalid state. \
                    Did you remove it from the world while it was rendering?",
                    window.config.title, window.id
                );
                drop(window);
                world.remove(handle);
                return;
            }
        };

        // render the world in its current state
        renderer.render(world);

        // print error if the target window was moved or destroyed while it was rendering
        let mut window = match world.get_mut(handle) {
            Some(window) => window,
            None => {
                eprintln!(
                    "A window was destroyed or its world handle was changed while rendering. \
                    It is advised that you do not remove a window while it is rendering."
                );

                // try to recover the original window if it was moved and now has a new handle
                match world.get_mut_where::<Self>(|p| p.id() == event.id()) {
                    Some(window) => window,
                    None => return,
                }
            }
        };

        // return the renderer to its rightful place
        window.renderer = Some(renderer);
    }
}
