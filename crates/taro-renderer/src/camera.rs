use boba_core::{BobaWorld, Pearl};
use wgpu::Texture;

use crate::TaroHardware;

pub struct TaroCamera {
    _private: (),
}

impl Pearl for TaroCamera {}

impl TaroCamera {
    pub fn render(&mut self, hardware: &TaroHardware, texture: &Texture, world: &BobaWorld) {
        let _ = hardware;
        let _ = world;
        let _ = texture;
    }
}
