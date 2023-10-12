use boba_engine::prelude::*;

fn main() {
    env_logger::init();
    let mut milk_tea = MilkTea::new();

    milk_tea
        .world
        .insert(WindowBuilder::<TaroRenderer>::new(WindowConfig::default()));

    milk_tea
        .world
        .insert(WindowBuilder::<TaroRenderer>::new(WindowConfig::default()));

    milk_tea.run();
}
