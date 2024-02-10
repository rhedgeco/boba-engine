use boba_3d::{glam::Mat4, Transform};
use boba_core::{
    world::{Link, PearlView, WorldAccess},
    Pearl,
};
use extension_trait::extension_trait;
use wgpu::Texture;

use crate::Hardware;

pub struct TaroCamera {
    transform: Link<Transform>,
    position_matrix: Mat4,
}

impl Pearl for TaroCamera {}

impl TaroCamera {
    pub fn new(transform: Link<Transform>) -> Self {
        Self {
            transform,
            position_matrix: Mat4::IDENTITY,
        }
    }

    pub fn link_transform(&mut self, transform: Link<Transform>) {
        self.transform = transform;
    }
}

#[extension_trait]
pub impl TaroCameraView for PearlView<'_, TaroCamera> {
    fn render(&mut self, texture: &Texture) {
        let hardware = Hardware::get();
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = hardware
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // update position matrix
        if let Some(transform) = self.world().get(self.transform) {
            self.position_matrix = transform.world_matrix();
        }

        hardware.queue().submit(Some(encoder.finish()));
    }
}
