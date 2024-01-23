pub use boba_core as core;

pub mod prelude {
    pub use boba_core::{
        pearl::{Event, EventSource, Listener, SimpleEvent},
        world::{InsertLink, Link, Removed, View},
        Pearl, World,
    };

    pub use boba_3d::{glam::*, transform::Link as TransformLink, Transform, TransformTree};
}
