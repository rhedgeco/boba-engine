use boba_engine::prelude::*;

pub struct TransformRotator {
    transform: Link<Transform>,
    current: f32,
    speed: f32,
}

impl Pearl for TransformRotator {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }

    fn on_insert(mut ctx: InsertContext<Self>) {
        let rotation = ctx.view.current;
        let transform = ctx.view.transform;
        let mut transform = ctx.view.world_mut().get_view(transform).unwrap();
        transform.set_local_rot(Quat::from_rotation_z(rotation.to_radians()))
    }
}

impl Listener<Update> for TransformRotator {
    fn trigger(mut view: PearlView<Self>, event: &mut UpdateData) {
        view.current = (view.current + view.speed * event.delta_time()) % 360f32;
        let rotation = view.current;
        let transform = view.transform;
        let mut transform = view.world_mut().get_view(transform).unwrap();
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
    fn trigger(mut view: PearlView<Self>, _: &mut UpdateData) {
        let transform = view.transform;
        let transform = view.world_mut().get_view(transform).unwrap();
        println!("Child world_pos: {}", transform.world_pos());
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();

    // create initial transform
    let t1 = world.insert(Transform::new());

    // create child transform
    let t2 = world.insert_then(Transform::from_pos(Vec3::X), |mut t| {
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
    milk_tea::run(&mut world);
}
