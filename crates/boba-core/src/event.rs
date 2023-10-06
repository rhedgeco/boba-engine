use crate::{arena::ArenaView, Pearl};

/// Marker trait that designates a struct can be used to trigger a [`World`](crate::World) event.
pub trait Event: 'static {}
impl<T: 'static> Event for T {}

pub trait EventListener<E: Event>: Pearl {
    fn update(event: &mut E, pearls: &mut ArenaView<Self>);
}

pub trait EventRegister<P: Pearl> {
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
