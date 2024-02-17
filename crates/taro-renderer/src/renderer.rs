use boba_core::world::{Link, PearlView};
use milk_tea::pearls::window::{Renderer, Window, WindowHandle};
use once_cell::sync::OnceCell;
use wgpu::{
    Adapter, Device, Instance, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration,
};

use crate::pearls::{camera::TaroCameraView, TaroCamera};

pub struct TaroRenderer {
    surface: Option<Surface<'static>>,
    config: Option<SurfaceConfiguration>,
    camera: Link<TaroCamera>,
}

impl TaroRenderer {
    pub fn new(camera: Link<TaroCamera>) -> Self {
        Self {
            surface: None,
            config: None,
            camera,
        }
    }
}

impl Renderer for TaroRenderer {
    fn unload(&mut self) {
        self.surface = None;
    }

    fn load(&mut self, handle: impl WindowHandle) -> bool {
        self.surface = match Hardware::instance().create_surface(handle) {
            Ok(surface) => Some(surface),
            Err(e) => {
                log::error!("Failed to create Taro surface. Error: {e}");
                return false;
            }
        };

        return true;
    }

    fn render(pearl: &mut PearlView<Window<Self>>) {
        let size = pearl.inner_size();
        let width = size.0.max(1);
        let height = size.1.max(1);
        let renderer = pearl.renderer_mut();
        let Some(surface) = renderer.surface.as_ref() else {
            log::warn!("Tried to render unloaded TaroRenderer.");
            return;
        };

        let hardware = Hardware::init_with(surface);
        match renderer.config.as_mut() {
            Some(config) => {
                if config.width != width || config.height != height {
                    config.width = width;
                    config.height = height;
                    surface.configure(hardware.device(), config);
                }
            }
            None => match surface.get_default_config(hardware.adapter(), width, height) {
                Some(config) => {
                    let config = renderer.config.insert(config);
                    config.present_mode = PresentMode::AutoNoVsync;
                    surface.configure(hardware.device(), config);
                }
                None => {
                    log::error!("Tried to render window but surface is unsupported.");
                    return;
                }
            },
        };

        let cam_link = renderer.camera;
        let surface_texture = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                log::error!("Error getting surface texture. Error: {e}");
                return;
            }
        };

        let Some(mut camera) = pearl.get_view(cam_link) else {
            log::warn!("Tried to render window but camera was invalid.");
            return;
        };

        camera.render(&surface_texture.texture, hardware);
        pearl.pre_present_notify();
        surface_texture.present();
    }
}

static GLOBAL_HARDWARE: OnceCell<Hardware> = OnceCell::new();

pub struct Hardware {
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Hardware {
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    fn init_with(surface: &Surface<'static>) -> &'static Self {
        GLOBAL_HARDWARE.get_or_init(|| pollster::block_on(Self::build_hardware(surface)))
    }

    fn instance() -> &'static Instance {
        static INSTANCE: OnceCell<Instance> = OnceCell::new();
        INSTANCE.get_or_init(|| Instance::default())
    }

    async fn build_hardware(surface: &Surface<'static>) -> Self {
        let instance = Self::instance();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .expect("Failed to aquire valid hardware adapter");

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
        {
            Ok(data) => data,
            Err(e) => panic!("Failed to create hardware device. Error: {e}"),
        };

        Hardware {
            adapter,
            device,
            queue,
        }
    }
}
