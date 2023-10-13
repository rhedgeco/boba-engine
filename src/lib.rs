pub use boba_core as core;
pub use milk_tea;

pub mod prelude {
    pub use crate::core::{pearl, BobaWorld, Event, EventListener, EventRegister, Pearl};
    pub use milk_tea::{
        events::{MilkTeaExit, MilkTeaStart, MilkTeaUpdate},
        pearls::{ControlFlow, Time},
        window::{MilkTeaWindow, RenderManager, WindowBuilder},
        MilkTea,
    };
    pub use taro_renderer::{pearls::TaroSkybox, TaroRenderBuilder, TaroRenderer};
}
