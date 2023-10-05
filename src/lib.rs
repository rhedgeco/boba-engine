pub use boba_core as core;

pub mod prelude {
    pub use crate::core::{
        Event, EventListener, EventRegister, Pearl, PearlAccess, Resources, World,
    };
    pub use boba_proc::pearl;
}

// place self in extern prelude so re-exports of this crate work with proc macros
extern crate self as boba_engine;
