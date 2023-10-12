use boba_core::BobaWorld;
use milk_tea::{
    window::{MilkTeaTarget, WindowRenderer},
    winit::window::Window,
};
use once_cell::sync::{Lazy, OnceCell};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

use crate::events::{TaroRender, TaroRenderData};

pub struct TaroHardware {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

static HARDWARE: OnceCell<TaroHardware> = OnceCell::new();
static INSTANCE: Lazy<Instance> = Lazy::new(|| {
    Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    })
});

pub struct TaroRenderer {
    target: MilkTeaTarget,
    window: Window,
    surface: Surface,
    config: SurfaceConfiguration,
}

impl WindowRenderer for TaroRenderer {
    fn window(&self) -> &Window {
        &self.window
    }

    fn init(target: MilkTeaTarget, window: Window) -> Self {
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&hardware.device, &config);

        Self {
            target,
            window,
            surface,
            config,
        }
    }

    fn render(&mut self, world: &mut BobaWorld) {
        log::info!("Rendering window {:?} with TaroRenderer", self.window.id());
        let Some(hardware) = HARDWARE.get() else {
            log::error!("Cannot render. TaroHardware is not initialized.");
            return;
        };

        // re-configure the surface if the window size has changed
        let window_size = self.window.inner_size();
        if window_size.width != self.config.width || window_size.height != self.config.height {
            self.config.width = window_size.width;
            self.config.height = window_size.height;
            self.surface.configure(&hardware.device, &self.config);
        }

        let output = match self.surface.get_current_texture() {
            Ok(o) => o,
            Err(e) => {
                log::error!("Error getting current window surface: {e}");
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut buffers = Vec::new();
        let render_data = TaroRenderData::new(&view, self.target, hardware, &mut buffers);
        world.trigger::<TaroRender>(render_data);

        if buffers.is_empty() {
            let mut encoder =
                hardware
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Empty Encoder"),
                    });

            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Blank Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            buffers.push(encoder.finish());
        }

        hardware.queue.submit(buffers);
        output.present();
    }
}
