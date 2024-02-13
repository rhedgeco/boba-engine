use boba_core::{
    pearl::{EventSource, Listener},
    world::{PearlView, WorldAccess},
    Pearl,
};
use milk_tea::{
    events::{update::UpdateData, Update},
    pearls::Window,
};

use crate::renderer::TaroRenderer;

pub struct TaroSentinel;

impl Pearl for TaroSentinel {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for TaroSentinel {
    fn trigger(pearl: PearlView<Self>, event: &mut UpdateData) {
        if !pearl.world().has::<Window<TaroRenderer>>() {
            event.window_target().exit()
        }
    }
}
