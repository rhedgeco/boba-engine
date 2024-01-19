use crate::{
    world::{Inserted, Removed},
    World,
};

#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    /// called when the first pearl of its type is inserted into a world
    fn init_type(world: &mut World) {}

    /// called every time a pearl is inserted into a world
    fn on_insert(link: Inserted<Self>, world: &mut World) {}

    /// called every time a pearl is removed from a world
    fn on_remove(pearl: Removed<Self>, world: &mut World) {}
}
