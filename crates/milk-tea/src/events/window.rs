use boba_core::{
    pearl::SimpleEvent,
    world::{Link, PearlView},
};

use crate::{window::MilkTeaId, MilkTeaWindow};

pub struct WindowInit {
    link: Link<MilkTeaWindow>,
    id: MilkTeaId,
}

impl SimpleEvent for WindowInit {}

impl WindowInit {
    pub(crate) fn new(window: &PearlView<MilkTeaWindow>) -> Self {
        Self {
            link: window.link(),
            id: window.id(),
        }
    }

    pub fn link(&self) -> Link<MilkTeaWindow> {
        self.link
    }

    pub fn id(&self) -> MilkTeaId {
        self.id
    }
}

pub struct PreRender {
    link: Link<MilkTeaWindow>,
    id: MilkTeaId,
}

impl SimpleEvent for PreRender {}

impl PreRender {
    pub(crate) fn new(window: &PearlView<MilkTeaWindow>) -> Self {
        Self {
            link: window.link(),
            id: window.id(),
        }
    }

    pub fn link(&self) -> Link<MilkTeaWindow> {
        self.link
    }

    pub fn id(&self) -> MilkTeaId {
        self.id
    }
}

pub struct CloseRequest {
    link: Link<MilkTeaWindow>,
    id: MilkTeaId,
}

impl SimpleEvent for CloseRequest {}

impl CloseRequest {
    pub(crate) fn new(link: Link<MilkTeaWindow>, id: MilkTeaId) -> Self {
        Self { link, id }
    }

    pub fn link(&self) -> Link<MilkTeaWindow> {
        self.link
    }

    pub fn id(&self) -> MilkTeaId {
        self.id
    }
}
