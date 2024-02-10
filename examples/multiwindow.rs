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
    world.insert(FocusPrinter);
    let cam1 = world.insert(TaroCamera::default());
    let cam2 = world.insert(TaroCamera::default());
    let cam3 = world.insert(TaroCamera::default());
    world.insert(TaroWindow::new(cam1));
    world.insert(TaroWindow::new(cam2));
    world.insert(TaroWindow::new(cam3));
    world.insert(TaroSentinel); // closes the app when there are no more windows
    milk_tea::run(&mut world);
}
