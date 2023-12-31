use crate::{data::Colorf64, passes::SolidColorRenderPass, TaroCamera};
use boba_core::{pearl::Handle, BobaWorld};
use milk_tea::{
    window::{RenderBuilder, RenderManager},
    winit::window::{Window, WindowId},
};
use once_cell::sync::{Lazy, OnceCell};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

pub struct TaroHardware {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

static HARDWARE: OnceCell<TaroHardware> = OnceCell::new();

#[derive(Default)]
pub struct TaroRenderBuilder {
    pub render_cam: Option<Handle<TaroCamera>>,
}

impl RenderBuilder for TaroRenderBuilder {
    type Renderer = TaroRenderer;

    fn build(self, window: Window) -> Self::Renderer {
        static INSTANCE: Lazy<Instance> = Lazy::new(|| {
            Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
            })
        });

        let surface = unsafe { INSTANCE.create_surface(&window) }.unwrap();
        let hardware = HARDWARE.get_or_init(|| {
            let adapter =
                pollster::block_on(INSTANCE.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                }))
                .unwrap();
            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            ))
            .unwrap();
            TaroHardware {
                adapter,
                device,
                queue,
            }
        });
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&hardware.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&hardware.device, &surface_config);
        Self::Renderer {
            window,
            surface,
            surface_config,
            camera: self.render_cam,
        }
    }
}

pub struct TaroRenderer {
    window: Window,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    camera: Option<Handle<TaroCamera>>,
}

impl RenderManager for TaroRenderer {
    fn window_id(&self) -> WindowId {
        self.window.id()
    }

    fn suspend(&mut self) {
        log::error!("Suspend is currently not implemented");
    }

    fn resume(&mut self) {
        log::error!("Resume is currently not implemented");
    }

    fn render(&mut self, world: &BobaWorld) {
        let Some(hardware) = HARDWARE.get() else {
            log::error!("Cannot render. TaroHardware is not initialized.");
            return;
        };

        // re-configure the surface if the window size has changed
        let window_size = self.window.inner_size();
        if window_size.width != self.surface_config.width
            || window_size.height != self.surface_config.height
        {
            self.surface_config.width = window_size.width;
            self.surface_config.height = window_size.height;
            self.surface
                .configure(&hardware.device, &self.surface_config);
        }

        let output = match self.surface.get_current_texture() {
            Ok(o) => o,
            Err(e) => {
                log::error!("Error getting current window surface: {e}");
                return;
            }
        };

        let mut command_buffers = Vec::new();
        if let Some(Some(mut camera)) = self.camera.map(|h| world.get_mut(h)) {
            command_buffers.push(camera.render(hardware, &output.texture, world));
        } else {
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder =
                hardware
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Empty Encoder"),
                    });

            SolidColorRenderPass::render(&Colorf64::BLACK, &mut encoder, &view);
            command_buffers.push(encoder.finish());
        }

        hardware.queue.submit(command_buffers);
        output.present();
    }
}
