use boba_core::World;

use crate::events::MilkTeaUpdate;

pub fn run_headless(world: &mut World) {
    let mut update = MilkTeaUpdate::new();
    loop {
        let delta_time = update.next_delta();
        world.trigger::<MilkTeaUpdate>(&delta_time);
    }
}
