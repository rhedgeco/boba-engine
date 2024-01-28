use boba_engine::prelude::*;

#[derive(Default)]
struct FpsPrinter;
impl Pearl for FpsPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTeaUpdate>();
    }
}

impl Listener<MilkTeaUpdate> for FpsPrinter {
    fn update(_: &mut View<'_, Self>, delta_time: &f32) {
        println!("FPS: {}", 1. / delta_time);
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(FpsPrinter::default());
    run_windowed(&mut world);
}
