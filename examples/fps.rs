use boba_engine::prelude::*;

fn main() {
    let mut milk_tea = MilkTea::new();

    milk_tea.world.insert_callback::<MilkTeaUpdate>(|_, world| {
        let time = world.get_static::<Time>().unwrap();
        let fps = 1f64 / time.delta_time();
        println!("FPS: {fps}");
    });

    milk_tea.run();
}
