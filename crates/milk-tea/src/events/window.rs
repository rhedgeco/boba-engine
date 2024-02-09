use boba_core::pearl::SimpleEvent;
use winit::window::WindowId;

pub struct CloseRequest {
    id: WindowId,
}

impl SimpleEvent for CloseRequest {}

impl CloseRequest {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}

pub struct RedrawRequest {
    id: WindowId,
}

impl SimpleEvent for RedrawRequest {}

impl RedrawRequest {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}
