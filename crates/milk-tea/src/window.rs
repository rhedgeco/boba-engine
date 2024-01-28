use boba_core::{
    world::{InsertContext, View},
    Pearl,
};
use extension_trait::extension_trait;
use winit::window::Window;

use crate::events::WindowInit;

pub struct MilkTeaWindow {
    window: Window,
}

impl MilkTeaWindow {
    pub(crate) fn new(window: Window) -> Self {
        Self { window }
    }

    pub(crate) fn native(&self) -> &Window {
        &self.window
    }
}

impl Pearl for MilkTeaWindow {
    fn on_insert(context: InsertContext<Self>) {
        let mut init = WindowInit::new(context.link);
        context.view.trigger_simple(&mut init);
    }
}

#[extension_trait]
pub(crate) impl MilkTeaWindowViewCrate for View<'_, MilkTeaWindow> {
    fn render(&self) {
        println!("Rendering Window");
    }
}
