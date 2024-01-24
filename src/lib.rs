pub use boba_core as core;

pub mod prelude {
    pub use boba_core::{
        pearl::{Event, EventSource, Listener, SimpleEvent},
        world::{InsertContext, Link, RemoveContext, View},
        Pearl, World,
    };

    pub use boba_3d::{glam::*, transform::TransformView, Transform};
}
