use crate::{
    event::EventRegistry,
    pearl::map::{Handle, MultiPearlMap},
    Event, Pearl,
};

/// The central data structure of `boba-core` that holds all the `Pearl` structs and resources.
#[derive(Default)]
pub struct World {
    events: EventRegistry,
    pearls: MultiPearlMap,
}

impl World {
    /// Returns a new empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts [`Pearl`] into this world, returning a [`Handle`] to its location.
    ///
    /// The pearl will be instantly registered with the worlds event system.
    pub fn insert_pearl<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        P::register(&mut self.events);
        self.pearls.insert_now(pearl)
    }

    /// Removes a [`Pearl`] if it exists from the world using its [`Handle`].
    pub fn remove_pearl<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.pearls.remove_now(handle)
    }

    /// Triggers an [`Event`] in this world.
    /// Any [`Pearl`] that is registered to listen for these events will be updated.
    pub fn trigger<E: Event>(&mut self, event: &mut E) {
        self.events.trigger::<E>(event, &mut self.pearls);
    }
}
