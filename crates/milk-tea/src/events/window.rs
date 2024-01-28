use boba_core::{pearl::SimpleEvent, world::Link};

use crate::MilkTeaWindow;

pub struct WindowInit {
    link: Link<MilkTeaWindow>,
}

impl SimpleEvent for WindowInit {}

impl WindowInit {
    pub(crate) fn new(link: Link<MilkTeaWindow>) -> Self {
        Self { link }
    }

    pub fn link(&self) -> Link<MilkTeaWindow> {
        self.link
    }
}

pub struct BeforeRender {
    link: Link<MilkTeaWindow>,
}

impl SimpleEvent for BeforeRender {}

impl BeforeRender {
    pub(crate) fn new(link: Link<MilkTeaWindow>) -> Self {
        Self { link }
    }

    pub fn window_link(&self) -> Link<MilkTeaWindow> {
        self.link
    }
}
