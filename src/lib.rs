pub use boba_core as core;
pub use milk_tea;

pub mod prelude {
    pub use crate::core::{
        pearl,
        pearl::collections::{PearlArena, PearlArenaView},
        Event, EventListener, EventRegister, Pearl, Resources,
    };
    pub use milk_tea::{
        events::{LateUpdate, Update},
        MilkTeaRunner, MilkTeaSettings,
    };
}
