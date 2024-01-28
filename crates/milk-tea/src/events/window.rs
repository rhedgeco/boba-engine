use boba_core::{pearl::Event, world::Link};

use crate::MilkTeaWindow;

pub struct WindowInit;

impl Event for WindowInit {
    type Data<'a> = Link<MilkTeaWindow>;
}
