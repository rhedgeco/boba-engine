use boba_engine::prelude::*;
use milk_tea::events::window::FocusChanged;

pub struct FocusPrinter;

impl Pearl for FocusPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<FocusChanged>();
    }
}

impl Listener<FocusChanged> for FocusPrinter {
    fn trigger(_: PearlView<Self>, event: &mut FocusChanged) {
        if event.focused() {
            println!("Window {:?} focused.", event.window_id());
        }
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
    world.insert(TaroWindow::new(cam1));
    world.insert(TaroWindow::new(cam2));
    world.insert(TaroWindow::new(cam3));
    world.insert(TaroSentinel);

    // add custom pearl to print focus changes
    world.insert(FocusPrinter);

    // run the world using milk tea
    milk_tea::run(&mut world);
}
