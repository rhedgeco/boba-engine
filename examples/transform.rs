use boba_engine::prelude::*;

pub struct TransformRotator {
    transform: Link<Transform>,
    current: f32,
    speed: f32,
}

impl Pearl for TransformRotator {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTeaUpdate>();
    }

    fn on_insert(context: InsertContext<Self>) {
        let rotation = context.view.current;
        let mut transform = context.view.view(context.view.transform).unwrap();
        transform.set_local_rot(Quat::from_rotation_z(rotation.to_radians()))
    }
}

impl Listener<MilkTeaUpdate> for TransformRotator {
    fn update(view: &mut View<'_, Self>, event: &mut MilkTeaUpdate) {
        view.current = (view.current + view.speed * event.delta_time()) % 360f32;
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
        source.listen::<MilkTeaUpdate>();
    }
}

impl Listener<MilkTeaUpdate> for TransformPrinter {
    fn update(view: &mut View<'_, Self>, _: &mut MilkTeaUpdate) {
        let transform = view.view(view.transform).unwrap();
        println!("Child world_pos: {}", transform.world_pos());
    }
}

fn main() {
    env_logger::init();
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

    // run headless
    run_headless(&mut world);
}
