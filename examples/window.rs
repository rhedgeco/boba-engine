use boba_engine::prelude::*;

fn main() {
    let mut milk_tea = MilkTea::new();

    milk_tea.world.insert(WindowBuilder {
        title: format!("Boba Engine"),
    });

    milk_tea.world.insert_callback::<MilkTeaUpdate>(|event, _| {
        let fps = 1f64 / event.delta_time();
        println!("FPS: {fps}");
    });

    milk_tea.run();
}
