use boba_core::{BobaWorld, Pearl};
use wgpu::{CommandBuffer, Texture};

use crate::{data::Colorf64, passes::SolidColorRenderPass, pearls::TaroSkybox, TaroHardware};

#[derive(Default)]
pub enum CameraSkybox {
    #[default]
    Global,
    Local(TaroSkybox),
}

#[derive(Default)]
pub struct TaroCamera {
    pub skybox: CameraSkybox,
}

impl Pearl for TaroCamera {}

impl TaroCamera {
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

        match &self.skybox {
            CameraSkybox::Local(skybox) => skybox.render(&mut encoder, &view),
            CameraSkybox::Global => {
                if let Some(skybox) = world.get_global::<TaroSkybox>() {
                    skybox.render(&mut encoder, &view);
                } else {
                    SolidColorRenderPass::render(&Colorf64::BLACK, &mut encoder, &view);
                }
            }
        }

        encoder.finish()
    }
}
