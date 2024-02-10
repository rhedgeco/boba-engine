use boba_core::pearl::SimpleEvent;
use winit::{dpi::PhysicalSize, window::WindowId};

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

pub struct WindowResized {
    size: PhysicalSize<u32>,
    id: WindowId,
}

impl SimpleEvent for WindowResized {}

impl WindowResized {
    pub(crate) fn new(id: WindowId, size: PhysicalSize<u32>) -> Self {
        Self { size, id }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}
