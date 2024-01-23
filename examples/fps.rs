use std::time::Instant;

use boba_engine::prelude::*;

struct Update;
impl SimpleEvent for Update {}

#[derive(Default)]
struct FpsPrinter {
    instant: Option<Instant>,
}

impl Pearl for FpsPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for FpsPrinter {
    fn update(view: &mut View<'_, Self>, _: &Update) {
        let Some(past) = view.instant.replace(Instant::now()) else {
            return;
        };

        let delta = Instant::now().duration_since(past).as_secs_f64();
        println!("FPS: {}", 1. / delta);
    }
}

fn main() {
    let mut world = World::new();
    world.insert(FpsPrinter::default());
    loop {
        world.trigger_simple(&Update);
    }
}
