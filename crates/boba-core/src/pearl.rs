use crate::{
    world::{InsertLink, Removed, View},
    World,
};

pub trait Event: 'static {
    type Data<'a>;
}

pub trait SimpleEvent: 'static {}
impl<T: SimpleEvent> Event for T {
    type Data<'a> = Self;
}

pub trait EventSource<P> {
    fn listen<E: Event>(&mut self)
    where
        P: Listener<E>;
}

#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    fn register(source: &mut impl EventSource<Self>) {}
    fn on_insert(link: InsertLink<Self>, world: &mut World) {}
    fn on_remove(pearl: Removed<Self>, world: &mut World) {}
}

pub trait Listener<E: Event>: Sized + 'static {
    fn update(view: View<'_, Self>, data: &E::Data<'_>);
}
