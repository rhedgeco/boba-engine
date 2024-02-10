use boba_core::Pearl;
use extension_trait::extension_trait;
use wgpu::Texture;

use crate::Hardware;

#[derive(Default)]
pub struct TaroCamera {
    _private: (),
}

impl Pearl for TaroCamera {}

#[extension_trait]
pub impl TaroCameraView for TaroCamera {
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
        hardware.queue().submit(Some(encoder.finish()));
    }
}
