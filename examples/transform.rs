// use boba_engine::prelude::*;

// fn main() {
//     let mut transforms = TransformTree::new();
//     let t1 = transforms.spawn();
//     let t2 = transforms.spawn_with_parent(t1);

//     let mut t1view = transforms.view(t1).unwrap();
//     t1view.set_local_rot(Quat::from_rotation_z(90f32.to_radians()));

//     let mut t2view = transforms.view(t2).unwrap();
//     t2view.set_local_pos(Vec3::X);
//     println!("t2 world_pos: {}", t2view.world_pos);
// }

use boba_3d::transform::TransformView;
use boba_engine::prelude::*;

fn main() {
    let mut world = World::new();
    let t1 = Transform::from_rot(Quat::from_rotation_z(90f32.to_radians()));
    let t2 = Transform::from_pos(Vec3::X);
    let t1link = world.insert(t1);
    let t2link = world.insert_and(t2, |t2| {
        t2.set_parent(t1link);
    });

    let t2 = world.get(t2link).unwrap();
    println!("t2 world pos: {}", t2.world_pos());
}
