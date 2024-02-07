use crate::world::{view::PearlView, InsertContext, RemoveContext};

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
    fn on_insert(ctx: InsertContext<Self>) {}
    fn on_remove(ctx: RemoveContext<Self>) {}
}

pub trait Listener<E: Event>: Pearl {
    fn trigger(pearl: PearlView<Self>, event: &mut E::Data<'_>);
}
