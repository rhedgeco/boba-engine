use boba_core::{BobaWorld, Pearl};
use wgpu::{CommandBuffer, Texture};

use crate::{passes::BlackRenderPass, pearls::TaroSkybox, TaroHardware};

pub struct TaroCamera {
    _private: (),
}

impl Pearl for TaroCamera {}

impl TaroCamera {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn render(
        &mut self,
        hardware: &TaroHardware,
        texture: &Texture,
        world: &BobaWorld,
    ) -> CommandBuffer {
        log::info!("Rendering TaroCamera");

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = hardware
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Taro Camera Encoder"),
            });

        if let Some(skybox) = world.get_global::<TaroSkybox>() {
            skybox.render(&mut encoder, &view);
        } else {
            BlackRenderPass::render(&mut encoder, &view);
        }

        encoder.finish()
    }
}
