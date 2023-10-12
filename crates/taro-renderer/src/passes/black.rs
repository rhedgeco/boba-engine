use wgpu::{CommandEncoder, TextureView};

pub struct BlackRenderPass;

impl BlackRenderPass {
    pub fn render(encoder: &mut CommandEncoder, view: &TextureView) {
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
    }
}
