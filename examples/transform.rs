use std::time::Instant;

use boba_engine::prelude::*;

pub struct Update {
    instant: Option<Instant>,
}

impl Event for Update {
    type Data<'a> = f32;
}

impl Update {
    pub fn new() -> Self {
        Self { instant: None }
    }

    pub fn update(&mut self) -> f32 {
        match self.instant.replace(Instant::now()) {
            Some(last) => Instant::now().duration_since(last).as_secs_f32(),
            None => 0f32,
        }
    }
}

pub struct TransformRotator {
    transform: Link<Transform>,
    current: f32,
    speed: f32,
}

impl Pearl for TransformRotator {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }

    fn on_insert(view: &mut View<'_, Self>, _: Link<Self>) {
        let rotation = view.current;
        let mut transform = view.view(view.transform).unwrap();
        transform.set_local_rot(Quat::from_rotation_z(rotation.to_radians()))
    }
}

impl Listener<Update> for TransformRotator {
    fn update(view: &mut View<'_, Self>, delta_time: &f32) {
        view.current = (view.current + view.speed * delta_time) % 360f32;
        let rotation = view.current;
        let mut transform = view.view(view.transform).unwrap();
        transform.set_local_rot(Quat::from_rotation_z(rotation.to_radians()));
    }
}

pub struct TransformPrinter {
    transform: Link<Transform>,
}

impl Pearl for TransformPrinter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }
}

impl Listener<Update> for TransformPrinter {
    fn update(view: &mut View<'_, Self>, _: &f32) {
        let transform = view.view(view.transform).unwrap();
        println!("Child world_pos: {}", transform.world_pos());
    }
}

fn main() {
    let mut world = World::new();

    // create initial transform
    let t1 = world.insert(Transform::new());

    // create child transform
    let t2 = world.insert_and(Transform::from_pos(Vec3::X), |t| {
        t.set_parent(t1);
    });

    // create transform rotator
    world.insert(TransformRotator {
        transform: t1,
        current: 0f32,
        speed: 100f32,
    });

    // create transform printer
    world.insert(TransformPrinter { transform: t2 });

    // create update event
    let mut update = Update::new();
    loop {
        let delta_time = update.update();
        world.trigger::<Update>(&delta_time);
    }
}
