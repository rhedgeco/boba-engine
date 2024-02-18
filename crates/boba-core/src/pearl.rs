use crate::world::{Inserted, PearlView, Removed};

pub trait Event: 'static {}
impl<T: 'static> Event for T {}

pub trait Listener<E: Event>: Pearl {
    fn trigger(pearl: PearlView<Self>, event: &mut E);
}

pub trait EventSource<P> {
    fn listen<E: Event>(&mut self)
    where
        P: Listener<E>;
}

#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    fn register(source: &mut impl EventSource<Self>) {}
    fn on_insert(pearl: Inserted<Self>) {}
    fn on_remove(pearl: Removed<Self>) {}
}
