use boba_core::{
    pearl::{EventSource, Listener},
    world::PearlView,
    Pearl,
};
use milk_tea::{
    events::{Data, MilkTea, Update},
    pearls::Window,
};

use crate::renderer::TaroRenderer;

pub struct TaroSentinel;

impl Pearl for TaroSentinel {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTea<Update>>();
    }
}

impl Listener<MilkTea<Update>> for TaroSentinel {
    fn trigger(pearl: PearlView<Self>, event: &mut Data<Update>) {
        if !pearl.world().has::<Window<TaroRenderer>>() {
            event.exit_app()
        }
    }
}
