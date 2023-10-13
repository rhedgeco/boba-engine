use boba_core::Pearl;
use wgpu::{CommandEncoder, TextureView};

use crate::{data::Colorf64, passes::SolidColorRenderPass};

pub enum TaroSkybox {
    Color(Colorf64),
}

impl Pearl for TaroSkybox {}

impl TaroSkybox {
    pub(crate) fn render(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        match self {
            Self::Color(color) => SolidColorRenderPass::render(color, encoder, view),
        }
    }
}
