use boba_3d::{glam::Mat4, Transform};
use boba_core::{
    world::{Link, PearlView},
    Pearl,
};
use extension_trait::extension_trait;
use wgpu::Texture;

use crate::{events::TaroRender, renderer::Hardware};

pub struct TaroCamera {
    pub transform: Link<Transform>,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    view_matrix: Mat4,
}

impl Pearl for TaroCamera {}

impl TaroCamera {
    pub fn new(transform: Link<Transform>) -> Self {
        Self {
            transform,
            fovy: 45.,
            znear: 0.1,
            zfar: 1000.,
            view_matrix: Mat4::IDENTITY,
        }
    }
}

#[extension_trait]
pub impl TaroCameraView for PearlView<'_, '_, TaroCamera> {
    fn render(&mut self, texture: &Texture, hardware: &Hardware) {
        // update view matrix
        if let Some(transform) = self.world().get(self.transform) {
            self.view_matrix = transform.world_matrix();
        }

        // build projection matrix
        let aspect_ratio = (texture.width() / texture.height()) as f32;
        let fovy = self.fovy.clamp(0., 360.);
        let znear = self.znear.max(0.001);
        let zfar = self.zfar.max(znear);
        let proj_matrix = Mat4::perspective_rh(fovy, aspect_ratio, znear, zfar);

        // create texture view and render event
        let tex_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut render_event =
            TaroRender::new(self.link(), tex_view, self.view_matrix, proj_matrix);

        // render the initial blank screen
        let mut encoder = hardware
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_event.texture_view(),
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
        render_event.queue(0, encoder.finish());

        // trigger the render event on all listeners
        self.world_mut().trigger_simple(&mut render_event);

        // submit the encoder to be rendered
        hardware.queue().submit(render_event);
    }
}
