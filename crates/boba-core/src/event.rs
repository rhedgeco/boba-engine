use crate::{arena::ArenaView, Pearl};

/// Designates a struct can be used to trigger a [`World`](crate::World) event.
pub trait Event: 'static {
    type Data<'a>;
    fn event_data<'a>(&'a mut self) -> Self::Data<'a>;
}

pub trait EventListener<E: Event>: Pearl {
    fn update<'a>(event: E::Data<'a>, pearls: &mut ArenaView<Self>);
}

pub trait EventRegister<P: Pearl> {
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
