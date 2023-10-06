pub use boba_core as core;
pub use milk_tea;

pub mod prelude {
    pub use crate::core::{
        pearl, Event, EventListener, EventRegister, Pearl, PearlAccess, Resources, World,
    };
    pub use milk_tea::{
        events::{LateUpdate, Update},
        MilkTeaRunner, MilkTeaSettings,
    };
}
