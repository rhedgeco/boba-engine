use boba_core::{
    pearl::{EventSource, Listener},
    world::PearlView,
    Pearl,
};
use milk_tea::{
    events::{app::Update, MilkTea},
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
    fn trigger(pearl: PearlView<Self>, event: &mut MilkTea<Update>) {
        if !pearl.world().has::<Window<TaroRenderer>>() {
            event.exit_app()
        }
    }
}
