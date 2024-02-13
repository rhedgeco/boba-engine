use std::sync::Arc;

use boba_core::{
    pearl::{EventSource, Listener},
    world::PearlView,
    Pearl,
};
use winit::{
    dpi::LogicalSize,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowBuilder,
};

use crate::events::{
    update::UpdateData,
    window::{CloseRequest, RedrawRequest},
    Update,
};

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static {}
impl<T: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static> WindowHandle for T {}

pub trait Renderer: Sized + 'static {
    fn unload(&mut self);
    fn load(&mut self, handle: impl WindowHandle) -> bool;
    fn render(pearl: &mut PearlView<Window<Self>>);
}

pub struct Window<T: Renderer> {
    builder: WindowBuilder,
    window: Option<Arc<winit::window::Window>>,
    renderer: T,
}

impl<T: Renderer> Window<T> {
    pub fn new(renderer: T) -> Self {
        Self {
            builder: WindowBuilder::new()
                .with_title("Milk Tea Window")
                .with_inner_size(LogicalSize::new(1280, 720)),
            window: None,
            renderer,
        }
    }

    pub fn renderer(&self) -> &T {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut T {
        &mut self.renderer
    }

    pub fn inner_size(&self) -> (u32, u32) {
        match &self.window {
            Some(window) => {
                let size = window.inner_size().to_logical(window.scale_factor());
                (size.width, size.height)
            }
            None => {
                let size = self
                    .builder
                    .window_attributes()
                    .inner_size
                    .unwrap()
                    .to_logical(1.0);
                (size.width, size.height)
            }
        }
    }

    pub fn pre_present_notify(&self) {
        if let Some(window) = &self.window {
            window.pre_present_notify();
        }
    }
}

impl<T: Renderer> Pearl for Window<T> {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<RedrawRequest>();
        source.listen::<CloseRequest>();
        source.listen::<Update>();
    }
}

impl<T: Renderer> Listener<RedrawRequest> for Window<T> {
    fn trigger(mut pearl: PearlView<Self>, event: &mut RedrawRequest) {
        let Some(window) = pearl.window.as_ref() else {
            return;
        };

        if window.id() != event.window_id() {
            return;
        }

        T::render(&mut pearl);
    }
}

impl<T: Renderer> Listener<CloseRequest> for Window<T> {
    fn trigger(mut pearl: PearlView<Self>, event: &mut CloseRequest) {
        let Some(window) = pearl.window.as_ref() else {
            return;
        };

        if window.id() != event.window_id() {
            return;
        }

        pearl.defer_destroy_self();
    }
}

impl<T: Renderer> Listener<Update> for Window<T> {
    fn trigger(mut pearl: PearlView<Self>, event: &mut UpdateData) {
        let window = match &pearl.window {
            Some(window) => window,
            None => {
                let builder = pearl.builder.clone();
                let window = match builder.build(event.window_target()) {
                    Ok(window) => Arc::new(window),
                    Err(e) => {
                        log::error!("Failed to create window. Error: {e}");
                        pearl.defer_destroy_self();
                        return;
                    }
                };

                if !pearl.renderer.load(window.clone()) {
                    log::error!("Failed to load window renderer. Destroying window.");
                    pearl.defer_destroy_self();
                    return;
                }

                pearl.window.insert(window)
            }
        };

        window.request_redraw();
    }
}
