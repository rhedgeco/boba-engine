use boba_core::{world::InsertContext, Pearl};
use winit::window::Window;

use crate::events::WindowInit;

pub struct MilkTeaWindow {
    window: Window,
}

impl MilkTeaWindow {
    pub(crate) fn new(window: Window) -> Self {
        Self { window }
    }

    pub(crate) fn window(&self) -> &Window {
        &self.window
    }
}

impl Pearl for MilkTeaWindow {
    fn on_insert(context: InsertContext<Self>) {
        let mut init = WindowInit::new(context.link);
        context.view.trigger_simple(&mut init);
    }
}
