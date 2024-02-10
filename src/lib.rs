pub use boba_core as core;

pub mod prelude {
    pub use boba_core::{
        pearl::{Event, EventSource, Listener, SimpleEvent},
        world::{
            InsertContext, Link, PearlView, RemoveContext, WorldAccess, WorldInsert, WorldRemove,
        },
        Pearl, World,
    };

    pub use boba_3d::{glam::*, transform::TransformView, Transform};
    pub use milk_tea::events::{update::UpdateData, Update};
    pub use taro_renderer::pearls::{TaroCamera, TaroSentinel, TaroWindow};
}
