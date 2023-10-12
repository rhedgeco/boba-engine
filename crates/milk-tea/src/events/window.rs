use boba_core::Event;
use winit::window::WindowId;

pub struct RedrawRequest {
    id: WindowId,
}

impl Event for RedrawRequest {
    type Data<'a> = &'a Self;
}

impl RedrawRequest {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }
}

pub struct CloseRequest {
    id: WindowId,
}

impl Event for CloseRequest {
    type Data<'a> = &'a Self;
}

impl CloseRequest {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }
}

pub struct Suspended {
    _private: (),
}

impl Suspended {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Event for Suspended {
    type Data<'a> = &'a Self;
}

pub struct Resumed {
    _private: (),
}

impl Resumed {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Event for Resumed {
    type Data<'a> = &'a Self;
}
