use boba_engine::prelude::*;
use taro_renderer::pearls::camera::TaroCamera;

fn main() {
    env_logger::init();
    let mut world = World::new();
    let cam1 = world.insert(TaroCamera::default());
    let cam2 = world.insert(TaroCamera::default());
    let cam3 = world.insert(TaroCamera::default());
    world.insert(TaroWindow::new(cam1));
    world.insert(TaroWindow::new(cam2));
    world.insert(TaroWindow::new(cam3));
    world.insert(TaroSentinel); // closes the app when there are no more windows
    milk_tea::run(&mut world);
}
