use std::sync::Arc;

use boba_core::{
    pearl::{EventSource, Listener},
    world::{PearlView, WorldQueue},
    Pearl,
};
use winit::{
    dpi::LogicalSize,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowBuilder,
};

use crate::events::{
    app::Update,
    window::{Close, Redraw},
    MilkTea,
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
        source.listen::<MilkTea<Redraw>>();
        source.listen::<MilkTea<Close>>();
        source.listen::<MilkTea<Update>>();
    }
}

impl<T: Renderer> Listener<MilkTea<Redraw>> for Window<T> {
    fn trigger(mut pearl: PearlView<Self>, event: &mut MilkTea<Redraw>) {
        let Some(window) = pearl.window.as_ref() else {
            return;
        };

        if window.id() != event.window_id() {
            return;
        }

        T::render(&mut pearl);
    }
}

impl<T: Renderer> Listener<MilkTea<Close>> for Window<T> {
    fn trigger(mut pearl: PearlView<Self>, event: &mut MilkTea<Close>) {
        let Some(window) = pearl.window.as_ref() else {
            return;
        };

        if window.id() != event.window_id() {
            return;
        }

        pearl.destroy_self();
    }
}

impl<T: Renderer> Listener<MilkTea<Update>> for Window<T> {
    fn trigger(pearl: PearlView<Self>, event: &mut MilkTea<Update>) {
        match &pearl.window {
            Some(window) => window.request_redraw(),
            None => {
                let link = pearl.link();
                event.target_defer(move |world, target| {
                    let mut queue = WorldQueue::new(world);
                    let Some(mut pearl) = PearlView::new(link, &mut queue) else {
                        return;
                    };

                    let builder = pearl.builder.clone();
                    let window = match builder.build(target) {
                        Ok(window) => Arc::new(window),
                        Err(e) => {
                            log::error!("Failed to create window. Error: {e}");
                            pearl.destroy_self();
                            return;
                        }
                    };

                    if !pearl.renderer.load(window.clone()) {
                        log::error!("Failed to load window renderer. Destroying window.");
                        pearl.destroy_self();
                        return;
                    }

                    pearl.window = Some(window);
                });
            }
        };
    }
}
