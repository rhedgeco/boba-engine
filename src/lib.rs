pub use boba_core as core;
pub use milk_tea;

pub mod prelude {
    pub use crate::core::{pearl, BobaWorld, Event, EventListener, EventRegister, Pearl};
    pub use milk_tea::{
        events::{Exit, Start, Update},
        window::{MilkTeaWindow, WindowBuilder, WindowManager},
        MilkTea,
    };
    pub use taro_renderer::{TaroRenderConfig, TaroRenderer};
}
