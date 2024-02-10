use boba_engine::prelude::*;
use taro_renderer::pearls::camera::TaroCamera;

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
    world.insert(FpsPrinter::default());
    let cam = world.insert(TaroCamera::default());
    world.insert(TaroWindow::new(cam));
    world.insert(TaroSentinel); // closes the app when there are no more windows
    milk_tea::run(&mut world);
}
