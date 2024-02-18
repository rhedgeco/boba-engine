use boba_3d::glam::Mat4;
use boba_core::world::Link;
use wgpu::{CommandBuffer, TextureView};

use crate::pearls::TaroCamera;

pub struct TaroRender {
    view_matrix: Mat4,
    proj_matrix: Mat4,
    tex_view: TextureView,

    link: Link<TaroCamera>,
    buffers: Vec<(usize, CommandBuffer)>,
}

impl TaroRender {
    pub(crate) fn new(
        link: Link<TaroCamera>,
        tex_view: TextureView,
        view_matrix: Mat4,
        proj_matrix: Mat4,
    ) -> Self {
        Self {
            view_matrix,
            proj_matrix,
            tex_view,
            link,
            buffers: Vec::new(),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.tex_view
    }

    pub fn camera_link(&self) -> Link<TaroCamera> {
        self.link
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn queue(&mut self, order: usize, buffer: CommandBuffer) {
        let buffer_indexer = self.buffers.iter().enumerate();
        let index = match buffer_indexer.rev().find(|(_, (o, _))| o <= &order) {
            Some((index, _)) => index + 1,
            None => 0,
        };

        self.buffers.insert(index, (order, buffer));
    }
}

impl IntoIterator for TaroRender {
    type Item = CommandBuffer;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.buffers.into_iter(),
        }
    }
}

pub struct IntoIter {
    inner: std::vec::IntoIter<(usize, CommandBuffer)>,
}

impl Iterator for IntoIter {
    type Item = CommandBuffer;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.1)
    }
}
