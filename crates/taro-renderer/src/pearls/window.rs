use boba_core::{
    pearl::{EventSource, Listener},
    world::PearlView,
    Pearl,
};
use milk_tea::{
    events::{
        update::UpdateData,
        window::{CloseRequest, RedrawRequest},
        Update,
    },
    winit::{
        dpi::LogicalSize,
        window::{Window, WindowBuilder},
    },
};

pub struct TaroWindow {
    window: Option<Window>,
    destroy_on_close: bool,
}

impl Default for TaroWindow {
    fn default() -> Self {
        Self {
            window: None,
            destroy_on_close: true,
        }
    }
}

impl TaroWindow {
    pub fn set_destroy_on_close(&mut self, destroy: bool) {
        self.destroy_on_close = destroy;
    }
}

impl Pearl for TaroWindow {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<CloseRequest>();
        source.listen::<RedrawRequest>();
        source.listen::<Update>();
    }
}

impl Listener<CloseRequest> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut CloseRequest) {
        let Some(window) = &pearl.window else {
            return;
        };

        if pearl.destroy_on_close && window.id() == event.window_id() {
            log::info!("Closing {:?}", window.id());
            pearl.defer_destroy_self();
        }
    }
}

impl Listener<RedrawRequest> for TaroWindow {
    fn trigger(pearl: PearlView<Self>, event: &mut RedrawRequest) {
        let Some(window) = &pearl.window else {
            return;
        };

        if window.id() != event.window_id() {
            return;
        }

        log::info!("Rendering {:?}", window.id());
    }
}

impl Listener<Update> for TaroWindow {
    fn trigger(mut pearl: PearlView<Self>, event: &mut UpdateData) {
        let window = match &mut pearl.window {
            Some(window) => window,
            w @ None => {
                let new_window = match WindowBuilder::new()
                    .with_title("Taro Window")
                    .with_inner_size(LogicalSize::new(1280, 720))
                    .build(event.window_target())
                {
                    Ok(window) => window,
                    Err(e) => {
                        log::error!("Failed to build window. Error: {e}");
                        pearl.defer_destroy_self();
                        return;
                    }
                };

                log::info!("Built new window {:?}", new_window.id());
                w.insert(new_window)
            }
        };

        window.request_redraw();
    }
}
