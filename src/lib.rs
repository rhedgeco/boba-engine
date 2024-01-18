pub use boba_core as core;

pub mod prelude {
    pub use boba_core::{
        world::{Link, Removed},
        Event, Pearl, World,
    };
}
