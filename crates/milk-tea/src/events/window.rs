use winit::{dpi::PhysicalSize, window::WindowId};

pub struct Close {
    id: WindowId,
}

impl Close {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}

pub struct Redraw {
    id: WindowId,
}

impl Redraw {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}

pub struct Resize {
    size: PhysicalSize<u32>,
    id: WindowId,
}

impl Resize {
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

pub struct Focus {
    focused: bool,
    id: WindowId,
}

impl Focus {
    pub(crate) fn new(id: WindowId, focused: bool) -> Self {
        Self { focused, id }
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn window_id(&self) -> WindowId {
        self.id
    }
}
