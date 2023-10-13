use boba_core::Pearl;
use wgpu::{CommandEncoder, TextureView};

use crate::data::Colorf64;

pub enum TaroSkybox {
    Color(Colorf64),
}

impl Pearl for TaroSkybox {}

impl TaroSkybox {
    pub(crate) fn render(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        match self {
            Self::Color(color) => {
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Blank Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: color.r,
                                g: color.g,
                                b: color.b,
                                a: color.a,
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
