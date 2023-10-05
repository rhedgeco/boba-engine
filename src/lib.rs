pub use boba_core as core;

pub mod prelude {
    pub use crate::core::{
        pearl, Event, EventListener, EventRegister, Pearl, PearlAccess, Resources, World,
    };
}
