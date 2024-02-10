use std::sync::Arc;

use once_cell::sync::OnceCell;
use wgpu::{Adapter, Device, Instance, Queue, RequestAdapterOptions};

use crate::pearls::window::SurfaceManager;

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
}

impl Hardware {
    pub fn get<'a>() -> &'static Self {
        Self::init_with(None)
    }

    pub(crate) fn init_with(manager: Option<Arc<SurfaceManager>>) -> &'static Self {
        GLOBAL_HARDWARE.get_or_init(|| pollster::block_on(Self::build_hardware(manager)))
    }

    pub(crate) fn instance() -> &'static Instance {
        static INSTANCE: OnceCell<Instance> = OnceCell::new();
        INSTANCE.get_or_init(|| Instance::default())
    }

    async fn build_hardware(manager: Option<Arc<SurfaceManager>>) -> Self {
        let instance = Self::instance();
        let adapter = match manager {
            Some(data) => Self::request_adapter_with(instance, data).await,
            None => Self::request_adapter(instance).await,
        }
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

    async fn request_adapter(instance: &Instance) -> Option<Adapter> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
    }

    async fn request_adapter_with(
        instance: &Instance,
        manager: Arc<SurfaceManager>,
    ) -> Option<Adapter> {
        instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&manager.surface),
            })
            .await
    }
}
