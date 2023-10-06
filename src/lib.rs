pub use boba_core as core;
pub use milk_tea;

pub mod prelude {
    pub use crate::core::{
        pearl, ArenaView, BobaArena, Event, EventListener, EventRegister, Pearl, Resources,
    };
    pub use milk_tea::{LateUpdate, MilkTeaRunner, MilkTeaSettings, Update};
}
