use crate::{
    world::{Link, Removed},
    World,
};

#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    fn init_type(world: &mut World) {}
    fn on_insert(link: Link<Self>, world: &mut World) {}
    fn on_remove(pearl: Removed<Self>, world: &mut World) {}
}
