use crate::{arena::ArenaView, Pearl};

/// Designates a struct can be used to trigger a [`World`](crate::World) event.
pub trait Event: 'static {
    /// Data type that is passed into am [`EventListener`]
    type Data<'a>;

    /// Provides the data necessary for an event update
    ///
    /// This will be called for every [`EventListener`]
    fn event_data<'a>(&'a mut self) -> Self::Data<'a>;
}

/// Trait that defines a callback for an [`Event`]
pub trait EventListener<E: Event>: Pearl {
    /// [`Event`] callback function
    fn update<'a>(event: E::Data<'a>, pearls: &mut ArenaView<Self>);
}

/// Registers [`EventListener`] callbacks available to `P`
pub trait EventRegister<P: Pearl> {
    /// Registers the callback for [`Event`]
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
