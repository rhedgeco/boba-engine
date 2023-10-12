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

    milk_tea
        .world
        .insert_callback::<MilkTeaUpdate>(|event, world| {
            if world.len::<MilkTeaWindow<TaroRenderer>>() == 0 {
                event.control_flow_mut().set_exit();
            }
        });

    milk_tea.run();
}
