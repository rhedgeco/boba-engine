use boba_engine::prelude::*;

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(MilkTeaWindowSettings::default());
    world.insert(MilkTeaWindowSettings::default());
    world.insert(MilkTeaWindowSettings::default());
    world.insert(CloseSentinel); // closes the app when there are no more windows
    milk_tea::run(&mut world);
}
