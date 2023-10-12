use boba_engine::prelude::*;
use milk_tea::events::MilkTeaEvent;

fn main() {
    let mut milk_tea = MilkTea::new();

    milk_tea
        .world
        .insert_callback::<MilkTeaEvent<Update>>(|event, _| {
            let fps = 1f64 / event.delta_time();
            println!("FPS: {fps}");
        });

    milk_tea.run();
}
