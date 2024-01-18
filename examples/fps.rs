use std::time::Instant;

use boba_engine::prelude::*;

#[derive(Debug)]
pub struct FpsPearl;

impl FpsPearl {
    fn update(&self, delta: f64) {
        println!("FPS: {}", 1. / delta);
    }
}

impl Pearl for FpsPearl {
    fn init_type(world: &mut World) {
        world.listen::<UpdateEvent>(|world, delta| {
            for pearl in world.iter::<Self>() {
                pearl.update(*delta);
            }
        });
    }
}

#[derive(Default)]
pub struct UpdateEvent {
    instant: Option<Instant>,
}

impl Event for UpdateEvent {
    type Data<'a> = f64;
}

impl UpdateEvent {
    pub fn lap(&mut self) -> Option<f64> {
        let last = self.instant.replace(Instant::now())?;
        Some(Instant::now().duration_since(last).as_secs_f64())
    }
}

fn main() {
    let mut world = World::new();
    world.insert(FpsPearl);

    let mut update = UpdateEvent::default();
    loop {
        if let Some(mut delta) = update.lap() {
            world.trigger::<UpdateEvent>(&mut delta);
        }
    }
}
