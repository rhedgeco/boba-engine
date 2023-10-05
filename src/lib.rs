pub use boba_core as core;

pub mod prelude {
    pub use crate::core::{Event, EventListener, EventRegister, Pearl, PearlAccess, World};
}
