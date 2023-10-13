use std::sync::atomic::{AtomicU64, Ordering};

use crate::events::{CloseRequest, RedrawRequest, Resume, Suspend, WindowInit};
use boba_core::{BobaWorld, EventListener, EventRegister, Pearl};
use winit::window::{Window, WindowId};

pub trait RenderBuilder: 'static {
    type Renderer: RenderManager;
    fn build(self, window: Window) -> Self::Renderer;
}

pub trait RenderManager: 'static {
    fn render(&mut self, world: &BobaWorld);
    fn window_id(&self) -> WindowId;
    fn suspend(&mut self);
    fn resume(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilkTeaId {
    value: u64,
}

impl MilkTeaId {
    pub(crate) fn new() -> Self {
        static IDGEN: AtomicU64 = AtomicU64::new(0);
        Self {
            value: IDGEN.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub struct WindowBuilder<M: RenderBuilder> {
    id: MilkTeaId,
    title: String,
    exit_on_close: bool,
    render_builder: M,
}

impl<M: RenderBuilder> WindowBuilder<M> {
    pub fn new(render_builder: M) -> Self {
        Self {
            id: MilkTeaId::new(),
            title: format!("Milk Tea"),
            exit_on_close: true,
            render_builder,
        }
    }

    pub fn id(&self) -> MilkTeaId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn exit_on_close(&self) -> bool {
        self.exit_on_close
    }

    pub fn set_exit_on_close(&mut self, exit: bool) {
        self.exit_on_close = exit;
    }

    pub fn render_builder(&self) -> &M {
        &self.render_builder
    }

    pub fn render_builder_mut(&mut self) -> &mut M {
        &mut self.render_builder
    }
}

impl<M: RenderBuilder> Pearl for WindowBuilder<M> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<WindowInit>();
    }
}

impl<M: RenderBuilder> EventListener<WindowInit> for WindowBuilder<M> {
    fn update<'a>(event: &mut <WindowInit as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        for builder in world.into_iter::<Self>() {
            let window = match Window::new(event.target()) {
                Ok(w) => w,
                Err(e) => {
                    let title = builder.title;
                    log::error!("Failed to create window '{title}': {e}");
                    continue;
                }
            };

            window.set_title(&builder.title);
            let window = MilkTeaWindow {
                id: builder.id,
                title: builder.title,
                window_id: window.id(),
                exit_on_close: builder.exit_on_close,
                manager: Box::new(builder.render_builder.build(window)),
            };

            world.insert(window);
        }
    }
}

pub struct MilkTeaWindow {
    id: MilkTeaId,
    title: String,
    window_id: WindowId,
    exit_on_close: bool,
    manager: Box<dyn RenderManager>,
}

impl MilkTeaWindow {
    pub fn id(&self) -> MilkTeaId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Pearl for MilkTeaWindow {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<CloseRequest>();
        register.event::<RedrawRequest>();
        register.event::<Suspend>();
        register.event::<Resume>();
    }
}

impl EventListener<CloseRequest> for MilkTeaWindow {
    fn update<'a>(event: &mut <CloseRequest as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        let Some(window) = world.get_where::<Self>(|p| p.window_id == event.id()) else {
            return;
        };

        if window.exit_on_close {
            let handle = window.handle();
            drop(window);
            world.remove(handle);
        }
    }
}

impl EventListener<RedrawRequest> for MilkTeaWindow {
    fn update<'a>(
        event: &mut <RedrawRequest as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        world
            .get_mut_where::<Self>(|p| p.window_id == event.id())
            .map(|mut w| w.manager.render(world));
    }
}

impl EventListener<Suspend> for MilkTeaWindow {
    fn update<'a>(_: &mut <Suspend as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            window.manager.suspend();
        }
    }
}

impl EventListener<Resume> for MilkTeaWindow {
    fn update<'a>(_: &mut <Resume as boba_core::Event>::Data<'a>, world: &mut BobaWorld) {
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            window.manager.resume();
        }
    }
}
