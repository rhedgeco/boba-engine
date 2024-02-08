use boba_engine::prelude::*;

#[derive(Default)]
struct FpsPrinter;
impl Pearl for FpsPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for FpsPrinter {
    fn trigger(_: PearlView<Self>, event: &mut Update) {
        println!("FPS: {}", 1. / event.delta_time());
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(FpsPrinter::default());
    world.insert(MilkTeaWindowSettings::default());
    world.insert(CloseSentinel); // closes the app when there are no more windows
    milk_tea::run(&mut world);
}
