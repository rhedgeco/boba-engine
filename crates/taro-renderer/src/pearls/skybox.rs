use boba_core::Pearl;
use wgpu::{CommandEncoder, TextureView};

pub enum TaroSkybox {
    Color { r: f64, g: f64, b: f64, a: f64 },
}

impl Pearl for TaroSkybox {}

impl TaroSkybox {
    pub(crate) fn render(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        match self {
            Self::Color { r, g, b, a } => {
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Blank Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: *r,
                                g: *g,
                                b: *b,
                                a: *a,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
            }
        }
    }
}
