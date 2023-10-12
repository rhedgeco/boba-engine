use crate::events::{CloseRequest, MilkTeaEvent, RedrawRequest, Resumed, Suspended, Update};
use boba_core::{BobaWorld, EventListener, EventRegister, Pearl};
use thiserror::Error;
use winit::window::{Window, WindowId};

pub trait WindowManager: 'static {
    type Config;
    fn init(window: Window, config: Self::Config) -> Self;
    fn render(&mut self, world: &BobaWorld);
    fn window(&self) -> &Window;
    fn suspend(&mut self);
    fn resume(&mut self);
}

pub struct WindowBuilder<M: WindowManager> {
    pub title: String,
    pub exit_on_close: bool,
    pub config: M::Config,
}

impl<M: WindowManager> WindowBuilder<M> {
    pub fn new(config: M::Config) -> Self {
        Self {
            title: format!("Milk Tea"),
            exit_on_close: true,
            config,
        }
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

    pub fn config(&self) -> &M::Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut M::Config {
        &mut self.config
    }
}

impl<M: WindowManager> Pearl for WindowBuilder<M> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaEvent<Update>>();
    }
}

impl<M: WindowManager> EventListener<MilkTeaEvent<Update>> for WindowBuilder<M> {
    fn update<'a>(
        event: &mut <MilkTeaEvent<Update> as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        for builder in world.into_iter::<Self>() {
            let window = match Window::new(event.window_target()) {
                Ok(w) => w,
                Err(e) => {
                    let title = builder.title;
                    log::error!("Failed to create window '{title}': {e}");
                    continue;
                }
            };

            window.set_title(&builder.title);
            let window = MilkTeaWindow {
                id: window.id(),
                title: builder.title,
                exit_on_close: builder.exit_on_close,
                manager: M::init(window, builder.config),
            };

            world.insert(window);
        }
    }
}

#[derive(Debug, Error)]
#[error("Cannot access renderer while it is currently rendering.")]
pub struct CurrentlyRendering;

pub struct MilkTeaWindow<M: WindowManager> {
    id: WindowId,
    title: String,
    exit_on_close: bool,
    manager: M,
}

impl<M: WindowManager> MilkTeaWindow<M> {
    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn window(&self) -> &Window {
        self.manager.window()
    }
}

impl<M: WindowManager> Pearl for MilkTeaWindow<M> {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<MilkTeaEvent<CloseRequest>>();
        register.event::<MilkTeaEvent<RedrawRequest>>();
        register.event::<MilkTeaEvent<Suspended>>();
        register.event::<MilkTeaEvent<Resumed>>();
    }
}

impl<M: WindowManager> EventListener<MilkTeaEvent<CloseRequest>> for MilkTeaWindow<M> {
    fn update<'a>(
        event: &mut <MilkTeaEvent<CloseRequest> as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        let Some(window) = world.get_where::<Self>(|p| p.id() == event.id()) else {
            return;
        };

        if window.exit_on_close {
            let handle = window.handle();
            drop(window);
            world.remove(handle);
        }
    }
}

impl<M: WindowManager> EventListener<MilkTeaEvent<RedrawRequest>> for MilkTeaWindow<M> {
    fn update<'a>(
        event: &mut <MilkTeaEvent<RedrawRequest> as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        world
            .get_mut_where::<Self>(|p| p.id() == event.id())
            .map(|mut w| w.manager.render(world));
    }
}

impl<M: WindowManager> EventListener<MilkTeaEvent<Suspended>> for MilkTeaWindow<M> {
    fn update<'a>(
        _: &mut <MilkTeaEvent<Suspended> as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            window.manager.suspend();
        }
    }
}

impl<M: WindowManager> EventListener<MilkTeaEvent<Resumed>> for MilkTeaWindow<M> {
    fn update<'a>(
        _: &mut <MilkTeaEvent<Resumed> as boba_core::Event>::Data<'a>,
        world: &mut BobaWorld,
    ) {
        for mut window in world.iter::<Self>().filter_map(|e| e.borrow_mut()) {
            window.manager.resume();
        }
    }
}
