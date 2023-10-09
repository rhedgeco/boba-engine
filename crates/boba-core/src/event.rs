use crate::{BobaWorld, Pearl};

/// Designates that a struct can be used to trigger an event in a [`BobaWorld`]
pub trait Event: 'static {
    /// The data that will be passed into the triggered event
    type Data<'a>;
}

/// Trait that defines a callback for an [`Event`]
pub trait EventListener<E: Event>: Pearl {
    // /// [`Event`] callback function
    fn update<'a>(event: &mut E::Data<'a>, world: &mut BobaWorld);
}

/// Registers [`EventListener`] callbacks available to `P`
pub trait EventRegister<P: Pearl> {
    /// Registers the callback for [`Event`]
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
