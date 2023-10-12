use boba_engine::prelude::*;
use milk_tea::events::MilkTeaEvent;

fn main() {
    env_logger::init();
    let mut milk_tea = MilkTea::new();

    milk_tea.world.insert(WindowBuilder::<TaroRenderer>::new(
        TaroRenderConfig::default(),
    ));

    milk_tea.world.insert(WindowBuilder::<TaroRenderer>::new(
        TaroRenderConfig::default(),
    ));

    milk_tea
        .world
        .insert_callback::<MilkTeaEvent<Update>>(|event, world| {
            if world.len::<MilkTeaWindow<TaroRenderer>>() == 0 {
                event.control_flow_mut().set_exit();
            }
        });

    milk_tea.run();
}
