pub use boba_core as core;

pub mod prelude {
    pub use boba_core::{
        pearl::{Event, EventSource, Listener},
        world::{Inserted, Link, PearlView, Removed},
        Pearl, World,
    };

    pub use boba_signal::{Signal, SignalBuilder, SignalRegister, WorldSignalExt};

    pub use boba_3d::{glam::*, transform::TransformView, Transform};
    pub use milk_tea::{
        events::{app::Update, MilkTea},
        pearls::Window,
    };
    pub use taro_renderer::{
        pearls::{TaroCamera, TaroSentinel},
        TaroRenderer,
    };
}
