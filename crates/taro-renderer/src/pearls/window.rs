use std::sync::Arc;

use boba_core::{
    pearl::{EventSource, Listener},
    world::{Link, PearlView, RemoveContext, WorldAccess},
    Pearl,
};
use milk_tea::{
    events::{
        update::UpdateData,
        window::{CloseRequest, RedrawRequest, WindowResized},
        Update,
    },
    winit::{
        dpi::LogicalSize,
        window::{Window, WindowBuilder},
    },
};
use wgpu::{CreateSurfaceError, Surface, SurfaceConfiguration, SurfaceTargetUnsafe};

use crate::{pearls::camera::TaroCameraView, Hardware};

use super::camera::TaroCamera;

pub(crate) struct SurfaceManager {
    pub surface: Surface<'static>,
    pub window: Window,
}

struct WindowData {
    manager: Arc<SurfaceManager>,
    config: SurfaceConfiguration,
}

impl WindowData {
    fn new(window: Window) -> Result<Self, CreateSurfaceError> {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let surface = unsafe {
            let target = SurfaceTargetUnsafe::from_window(&window).unwrap();
            Hardware::instance().create_surface_unsafe(target)?
        };

        let manager = Arc::new(SurfaceManager { surface, window });
        let hardware = Hardware::init_with(Some(manager.clone()));
        let config = manager
            .surface
            .get_default_config(hardware.adapter(), size.width, size.height)
            .unwrap();
        manager.surface.configure(hardware.device(), &config);

        Ok(Self { manager, config })
    }

    fn resize(&mut self) {
        let device = Hardware::get().device();
        let size = self.manager.window.inner_size();
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.manager.surface.configure(device, &self.config);
    }
}
pub struct TaroWindow {
    data: Option<WindowData>,
    source: Link<TaroCamera>,
    destroy_on_close: bool,
}

impl TaroWindow {
    pub fn new(source: Link<TaroCamera>) -> Self {
        Self {
            data: None,
            source,
            destroy_on_close: true,
        }
    }

    pub fn set_render_source(&mut self, camera: Link<TaroCamera>) {
        self.source = camera;
    }

    pub fn set_destroy_on_close(&mut self, destroy: bool) {
        self.destroy_on_close = destroy;
    }
}

impl Pearl for TaroWindow {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<CloseRequest>();
        source.listen::<RedrawRequest>();
        source.listen::<WindowResized>();
        source.listen::<Update>();
    }

    fn on_remove(ctx: RemoveContext<Self>) {
        // drop the window and surface when removed
        ctx.pearl.data.take();
    }
}

impl Listener<CloseRequest> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut CloseRequest) {
        let Some(data) = &pearl.data else {
            return;
        };

        if pearl.destroy_on_close && data.manager.window.id() == event.window_id() {
            log::info!("Closing {:?}", event.window_id());
            pearl.defer_destroy_self();
        }
    }
}

impl Listener<RedrawRequest> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut RedrawRequest) {
        let source = pearl.source;
        let Some(data) = &pearl.data else {
            return;
        };

        if data.manager.window.id() != event.window_id() {
            return;
        }

        let manager = data.manager.clone();
        let Some(mut camera) = pearl.world_mut().get_view(source) else {
            return;
        };

        log::info!("Rendering {:?}", event.window_id());
        let surface_texture = match manager.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                log::error!("Failed to get current texture for window. Error: {e}");
                return;
            }
        };

        camera.render(&surface_texture.texture);
        manager.window.pre_present_notify();
        surface_texture.present();
    }
}

impl Listener<WindowResized> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut WindowResized) {
        let Some(data) = &mut pearl.data else {
            return;
        };

        if data.manager.window.id() != event.window_id() {
            return;
        }

        data.resize();
    }
}

impl Listener<Update> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut UpdateData) {
        let data = match &mut pearl.data {
            Some(window) => window,
            w @ None => {
                let new_window = match WindowBuilder::new()
                    .with_title("Taro Window")
                    .with_inner_size(LogicalSize::new(1280, 720))
                    .build(event.window_target())
                {
                    Ok(window) => window,
                    Err(e) => {
                        log::error!("Failed to build window. Error: {e}");
                        pearl.defer_destroy_self();
                        return;
                    }
                };

                log::info!("Built new window {:?}", new_window.id());
                let data = match WindowData::new(new_window) {
                    Ok(data) => data,
                    Err(e) => {
                        log::error!("Failed to build window. Error: {e}");
                        pearl.defer_destroy_self();
                        return;
                    }
                };

                w.insert(data)
            }
        };

        data.manager.window.request_redraw();
    }
}
