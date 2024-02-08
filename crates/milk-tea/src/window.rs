use std::sync::atomic::{AtomicUsize, Ordering};

use boba_core::{
    pearl::{EventSource, Listener},
    world::{InsertContext, PearlView, WorldAccess},
    Pearl,
};
use extension_trait::extension_trait;
use winit::{
    dpi::LogicalSize,
    error::OsError,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

use crate::events::{window::CloseRequest, Update, WindowInit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilkTeaId(usize);

impl MilkTeaId {
    pub fn generate() -> Self {
        static GENERATOR: AtomicUsize = AtomicUsize::new(0);
        Self(GENERATOR.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct MilkTeaWindowSettings {
    pub title: String,
    pub size: (u32, u32),
    pub id: MilkTeaId,
    pub close_on_request: bool,
    pub redraw: bool,
}

impl Pearl for MilkTeaWindowSettings {}

impl Default for MilkTeaWindowSettings {
    fn default() -> Self {
        Self {
            title: format!("Milk Tea Window"),
            size: (1280, 720),
            id: MilkTeaId::generate(),
            close_on_request: true,
            redraw: true,
        }
    }
}

pub struct MilkTeaWindow {
    settings: MilkTeaWindowSettings,
    window: Window,
}

impl MilkTeaWindow {
    pub(crate) fn new(
        settings: MilkTeaWindowSettings,
        target: &EventLoopWindowTarget<()>,
    ) -> Result<Self, OsError> {
        let window = WindowBuilder::new()
            .with_title(settings.title.clone())
            .with_inner_size(LogicalSize::new(settings.size.0, settings.size.1))
            .build(target)?;

        Ok(Self { settings, window })
    }

    pub(crate) fn native(&self) -> &Window {
        &self.window
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }

    pub fn id(&self) -> MilkTeaId {
        self.settings.id
    }
}

impl Pearl for MilkTeaWindow {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
        source.listen::<CloseRequest>();
    }

    fn on_insert(mut ctx: InsertContext<Self>) {
        let mut init = WindowInit::new(&ctx.view);
        ctx.view.world_mut().trigger_simple(&mut init);
    }
}

impl Listener<Update> for MilkTeaWindow {
    fn trigger(pearl: PearlView<Self>, _: &mut Update) {
        if pearl.settings.redraw {
            pearl.request_redraw();
        }
    }
}

impl Listener<CloseRequest> for MilkTeaWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut CloseRequest) {
        if pearl.settings.close_on_request && pearl.id() == event.id() {
            pearl.defer_destroy_self();
        }
    }
}

#[extension_trait]
pub(crate) impl MilkTeaWindowViewCrate for PearlView<'_, MilkTeaWindow> {
    fn render(&mut self) {
        println!("Rendering Window {:?}", self.settings.id);
    }
}
