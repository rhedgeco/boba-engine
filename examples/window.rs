use boba_engine::prelude::*;

fn main() {
    env_logger::init();
    let mut milk_tea = MilkTea::new();

    milk_tea.world.insert(WindowBuilder::<TaroRenderer>::new(
        TaroRenderConfig::default(),
    ));

    milk_tea.world.insert(WindowBuilder::<TaroRenderer>::new(
        TaroRenderConfig::default(),
    ));

    milk_tea.world.insert_callback::<MilkTeaUpdate>(|_, world| {
        if world.len::<MilkTeaWindow<TaroRenderer>>() == 0 {
            let mut control = world.get_static_mut::<ControlFlow>().unwrap();
            control.set_exit(true);
        }
    });

    milk_tea.run();
}
