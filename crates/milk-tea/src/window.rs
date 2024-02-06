use boba_core::{
    world::{InsertContext, PearlView, WorldAccess},
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
    fn on_insert(mut ctx: InsertContext<Self>) {
        let mut init = WindowInit::new(ctx.view.link());
        ctx.view.world_mut().trigger_simple(&mut init);
    }
}

#[extension_trait]
pub(crate) impl MilkTeaWindowViewCrate for PearlView<'_, MilkTeaWindow> {
    fn render(&self) {
        println!("Rendering Window");
    }
}
