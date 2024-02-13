use boba_engine::prelude::*;
use milk_tea::events::window::{CloseRequest, FocusChanged};

pub struct StatePrinter;

impl Pearl for StatePrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<FocusChanged>();
        source.listen::<CloseRequest>();
    }
}

impl Listener<FocusChanged> for StatePrinter {
    fn trigger(_: PearlView<Self>, event: &mut FocusChanged) {
        if event.focused() {
            println!("Window {:?} focused.", event.window_id());
        }
    }
}

impl Listener<CloseRequest> for StatePrinter {
    fn trigger(_: PearlView<Self>, event: &mut CloseRequest) {
        println!("Closing Window {:?}.", event.window_id());
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();

    // create a transforms and cameras
    let t1 = world.insert(Transform::new());
    let t2 = world.insert(Transform::new());
    let t3 = world.insert(Transform::new());
    let cam1 = world.insert(TaroCamera::new(t1));
    let cam2 = world.insert(TaroCamera::new(t2));
    let cam3 = world.insert(TaroCamera::new(t3));

    // create windows and link cameras
    world.insert(Window::new(TaroRenderer::new(cam1)));
    world.insert(Window::new(TaroRenderer::new(cam2)));
    world.insert(Window::new(TaroRenderer::new(cam3)));
    world.insert(TaroSentinel);

    // add custom pearl to print focus changes
    world.insert(StatePrinter);

    // run the world using milk tea
    milk_tea::run(&mut world);
}
