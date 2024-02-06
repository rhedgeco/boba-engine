use boba_engine::prelude::*;

#[derive(Default)]
struct FpsPrinter;
impl Pearl for FpsPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTeaUpdate>();
    }
}

impl Listener<MilkTeaUpdate> for FpsPrinter {
    fn trigger(_: PearlView<Self>, event: &mut MilkTeaUpdate) {
        println!("FPS: {}", 1. / event.delta_time());
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(FpsPrinter::default());
    run_windowed(&mut world);
}
