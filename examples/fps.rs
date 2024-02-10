use boba_engine::prelude::*;

#[derive(Default)]
struct FpsPrinter;
impl Pearl for FpsPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for FpsPrinter {
    fn trigger(_: PearlView<Self>, event: &mut UpdateData) {
        println!("FPS: {}", 1. / event.delta_time());
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();

    // create a transform and camera
    let transform = world.insert(Transform::new());
    let cam = world.insert(TaroCamera::new(transform));

    // add a window and close sentinel
    world.insert(TaroWindow::new(cam));
    world.insert(TaroSentinel);

    // add the custom FPS printer pearl
    world.insert(FpsPrinter::default());

    // run the world using milk tea
    milk_tea::run(&mut world);
}
