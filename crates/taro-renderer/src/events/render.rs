use boba_core::Event;
use milk_tea::window::MilkTeaTarget;
use wgpu::{CommandBuffer, CommandEncoder, TextureView};

use crate::TaroHardware;

pub struct TaroRender {
    _private: (),
}

pub struct TaroRenderData<'a> {
    target: MilkTeaTarget,
    view: &'a TextureView,
    hardware: &'a TaroHardware,
    pub(crate) buffers: &'a mut Vec<CommandBuffer>,
}

impl<'a> TaroRenderData<'a> {
    pub(crate) fn new(
        view: &'a TextureView,
        target: MilkTeaTarget,
        hardware: &'a TaroHardware,
        buffers: &'a mut Vec<CommandBuffer>,
    ) -> Self {
        Self {
            view,
            target,
            hardware,
            buffers,
        }
    }

    pub fn target(&self) -> MilkTeaTarget {
        self.target
    }

    pub fn hardware(&self) -> &TaroHardware {
        self.hardware
    }

    pub fn current_view(&self) -> &TextureView {
        self.view
    }

    pub fn push_encoder(&mut self, encoder: CommandEncoder) {
        self.buffers.push(encoder.finish());
    }
}

impl Event for TaroRender {
    type Data<'a> = TaroRenderData<'a>;
}
