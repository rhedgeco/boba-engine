use boba_core::Event;
use winit::{event_loop::EventLoopWindowTarget, window::WindowId};

pub struct Suspend(());
impl Event for Suspend {
    type Data<'a> = ();
}

pub struct Resume(());
impl Event for Resume {
    type Data<'a> = ();
}

pub(crate) struct RedrawRequest {
    id: WindowId,
}

impl Event for RedrawRequest {
    type Data<'a> = Self;
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
    type Data<'a> = Self;
}

impl CloseRequest {
    pub(crate) fn new(id: WindowId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }
}

pub(crate) struct WindowInit(());
pub(crate) struct WindowInitData<'a> {
    target: &'a EventLoopWindowTarget<()>,
}

impl Event for WindowInit {
    type Data<'a> = WindowInitData<'a>;
}

impl<'a> WindowInitData<'a> {
    pub(crate) fn new(target: &'a EventLoopWindowTarget<()>) -> Self {
        Self { target }
    }

    pub fn target(&self) -> &EventLoopWindowTarget<()> {
        self.target
    }
}
