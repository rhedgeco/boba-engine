use boba_core::{
    pearl::{EventSource, Listener},
    world::{PearlView, WorldAccess},
    Pearl,
};

use crate::events::Update;

use super::{MilkTeaWindow, MilkTeaWindowSettings};

pub struct CloseSentinel;

impl Pearl for CloseSentinel {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for CloseSentinel {
    fn trigger(pearl: PearlView<Self>, event: &mut Update) {
        if !pearl.world().has::<MilkTeaWindow>() && !pearl.world().has::<MilkTeaWindowSettings>() {
            log::info!("No windows found. Shutting down application.");
            event.quit_app();
        }
    }
}
